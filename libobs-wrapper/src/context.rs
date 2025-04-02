use std::{
    ffi::CStr,
    pin::Pin,
    ptr,
    sync::{Arc, Mutex},
    thread::{self, ThreadId},
};

use crate::{
    data::{output::ObsOutputRef, video::ObsVideoInfo},
    display::{ObsDisplayCreationData, ObsDisplayRef, VertexBuffers},
    enums::{ObsLogLevel, ObsResetVideoStatus},
    logger::{extern_log_callback, internal_log_global, LOGGER},
    scenes::ObsSceneRef,
    unsafe_send::WrappedObsScene,
    utils::{ObsError, ObsModules, ObsString, OutputInfo, StartupInfo},
};
use anyhow::Result;
use getters0::Getters;
use libobs::{audio_output, video_output};
static OBS_THREAD_ID: Mutex<Option<ThreadId>> = Mutex::new(None);

/// Interface to the OBS context. Only one context
/// can exist across all threads and any attempt to
/// create a new context while there is an existing
/// one will error.
///
/// Note that the order of the struct values is
/// important! OBS is super specific about how it
/// does everything. Things are freed early to
/// latest from top to bottom.
#[derive(Debug, Getters)]
#[skip_new]
pub struct ObsContext {
    /// This string must be stored to keep the
    /// pointer passed to libobs valid.
    #[allow(dead_code)]
    locale: ObsString,
    /// Stores startup info for safe-keeping. This
    /// prevents any use-after-free as these do not
    /// get copied in libobs.
    startup_info: StartupInfo,

    #[get_mut]
    displays: Vec<Pin<Box<ObsDisplayRef>>>,

    #[skip_getter]
    vertex_buffers: VertexBuffers,

    /// Outputs must be stored in order to prevent
    /// early freeing.
    #[allow(dead_code)]
    #[get_mut]
    pub(crate) outputs: Vec<ObsOutputRef>,

    #[get_mut]
    pub(crate) scenes: Vec<ObsSceneRef>,

    #[skip_getter]
    pub(crate) active_scene: Arc<Mutex<Option<WrappedObsScene>>>,

    #[skip_getter]
    pub(crate) _obs_modules: ObsModules,

    /// This allows us to call obs_shutdown() after
    /// everything else has been freed. Doing other-
    /// wise completely crashes the program.
    #[skip_getter]
    _context_shutdown_zst: _ObsContextShutdownZST,
}

impl ObsContext {
    /// Initializes libobs on the current thread.
    ///
    /// Note that there can be only one ObsContext
    /// initialized at a time. This is because
    /// libobs is not completely thread-safe.
    ///
    /// Also note that this might leak a very tiny
    /// amount of memory. As a result, it is
    /// probably a good idea not to restart the
    /// OBS context repeatedly over a very long
    /// period of time. If anyone can fix this, it
    /// would be nice.
    pub fn new(info: StartupInfo) -> Result<ObsContext, ObsError> {
        // Checks that there are no other threads
        // using libobs using a static Mutex.
        //
        // Fun fact: this code caused a huge debate
        // about whether AtomicBool is UB or whatever
        // in the Rust Programming Discord server.
        // I didn't read too closely into it because
        // they were talking about what architecture
        // fridges have or something.
        //
        // Since this function is not meant to be
        // high-performance or called a thousand times,
        // a Mutex is fine here.
        let mutex_lock = OBS_THREAD_ID.lock();

        if let Ok(mut mutex_value) = mutex_lock {
            // Directly checks if the value of the
            // Mutex is false. If true, then error.
            if *mutex_value != None {
                return Err(ObsError::ThreadFailure);
            }

            // If the Mutex is None, then change
            // it to current thread ID so that no
            // other thread can use libobs while
            // the current thread is using it.
            *mutex_value = Some(thread::current().id());
        } else {
            return Err(ObsError::MutexFailure);
        }

        Self::init(info)
    }

    pub fn get_version() -> String {
        let version = unsafe { libobs::obs_get_version_string() };
        let version_cstr = unsafe { CStr::from_ptr(version) };
        version_cstr.to_string_lossy().into_owned()
    }

    pub fn log(&self, level: ObsLogLevel, msg: &str) {
        let mut log = LOGGER.lock().unwrap();
        log.log(level, msg.to_string());
    }

    /// Initializes the libobs context and prepares
    /// it for recording.
    ///
    /// More specifically, it calls `obs_startup`,
    /// `obs_reset_video`, `obs_reset_audio`, and
    /// registers the video and audio encoders.
    ///
    /// At least on Windows x64, it seems that
    /// resetting video and audio is necessary to
    /// prevent a memory leak when restarting the
    /// OBS context. This memory leak is not severe
    /// (~10 KB per restart), but the point is
    /// safety. Thank you @tt2468 for the help!
    fn init(mut info: StartupInfo) -> Result<ObsContext, ObsError> {
        // Sets the logger to the one passed in
        unsafe {
            libobs::base_set_log_handler(Some(extern_log_callback), std::ptr::null_mut());
        }

        let mut log_callback = LOGGER.lock().map_err(|_e| ObsError::MutexFailure)?;

        *log_callback = info.logger.take().expect("Logger can never be null");

        drop(log_callback);

        // Locale will only be used internally by
        // libobs for logging purposes, making it
        // unnecessary to support other languages.
        let locale_str = ObsString::new("en-US");
        let startup_status =
            unsafe { libobs::obs_startup(locale_str.as_ptr(), ptr::null(), ptr::null_mut()) };

        internal_log_global(ObsLogLevel::Info, format!("OBS {}", Self::get_version()));
        internal_log_global(
            ObsLogLevel::Info,
            "---------------------------------".to_string(),
        );

        if !startup_status {
            return Err(ObsError::Failure);
        }

        let mut obs_modules = ObsModules::add_paths(&info.startup_paths);

        // Note that audio is meant to only be reset
        // once. See the link below for information.
        //
        // https://docs.obsproject.com/frontends
        unsafe {
            libobs::obs_reset_audio2(info.obs_audio_info.as_ptr());
        }

        // Resets the video context. Note that this
        // is similar to Self::reset_video, but it
        // does not call that function because the
        // ObsContext struct is not created yet,
        // and also because there is no need to free
        // anything tied to the OBS context.
        let reset_video_status = Self::reset_video_internal(&mut info.obs_video_info);

        if reset_video_status != ObsResetVideoStatus::Success {
            return Err(ObsError::ResetVideoFailure(reset_video_status));
        }

        obs_modules.load_modules();

        internal_log_global(
            ObsLogLevel::Info,
            "==== Startup complete ===============================================".to_string(),
        );

        let vertex_buffers = unsafe { VertexBuffers::initialize() };

        Ok(Self {
            locale: locale_str,
            startup_info: info,
            outputs: vec![],
            vertex_buffers,
            displays: vec![],
            active_scene: Arc::new(Mutex::new(None)),
            scenes: vec![],
            _obs_modules: obs_modules,
            _context_shutdown_zst: _ObsContextShutdownZST {},
        })
    }

    /// Resets the OBS video context. This is often called
    /// when one wants to change a setting related to the
    /// OBS video info sent on startup.
    ///
    /// It is important to register your video encoders to
    /// a video handle after you reset the video context
    /// if you are using a video handle other than the
    /// main video handle. For convenience, this function
    /// sets all video encoder back to the main video handler
    /// by default.
    ///
    /// Note that you cannot reset the graphics module
    /// without destroying the entire OBS context. Trying
    /// so will result in an error.
    pub fn reset_video(&mut self, mut ovi: ObsVideoInfo) -> Result<(), ObsError> {
        // You cannot change the graphics module without
        // completely destroying the entire OBS context.
        if self.startup_info.obs_video_info.graphics_module() != ovi.graphics_module() {
            return Err(ObsError::ResetVideoFailureGraphicsModule);
        }

        let reset_video_status = Self::reset_video_internal(&mut ovi);

        if reset_video_status != ObsResetVideoStatus::Success {
            return Err(ObsError::ResetVideoFailure(reset_video_status));
        } else {
            for output in self.outputs.iter() {
                for video_encoder in output.get_video_encoders().iter() {
                    unsafe {
                        libobs::obs_encoder_set_video(
                            video_encoder.as_ptr(),
                            ObsContext::get_video_ptr().unwrap(),
                        )
                    }
                }
            }

            self.startup_info.obs_video_info = ovi;
            return Ok(());
        }
    }

    pub fn get_video_ptr() -> Result<*mut video_output, ObsError> {
        if let Ok(mutex_value) = OBS_THREAD_ID.lock() {
            if *mutex_value != Some(thread::current().id()) {
                return Err(ObsError::ThreadFailure);
            }
        } else {
            return Err(ObsError::MutexFailure);
        }

        Ok(unsafe { libobs::obs_get_video() })
    }

    pub fn get_audio_ptr() -> Result<*mut audio_output, ObsError> {
        if let Ok(mutex_value) = OBS_THREAD_ID.lock() {
            if *mutex_value != Some(thread::current().id()) {
                return Err(ObsError::ThreadFailure);
            }
        } else {
            return Err(ObsError::MutexFailure);
        }

        Ok(unsafe { libobs::obs_get_audio() })
    }

    fn reset_video_internal(ovi: &mut ObsVideoInfo) -> ObsResetVideoStatus {
        let status =
            num_traits::FromPrimitive::from_i32(unsafe { libobs::obs_reset_video(ovi.as_ptr()) });

        return match status {
            Some(x) => x,
            None => ObsResetVideoStatus::Failure,
        };
    }

    pub fn output(&mut self, info: OutputInfo) -> Result<ObsOutputRef, ObsError> {
        let output = ObsOutputRef::new(info.id, info.name, info.settings, info.hotkey_data);

        return match output {
            Ok(x) => {
                let tmp = x.clone();
                self.outputs.push(x);
                Ok(tmp)
            }

            Err(x) => Err(x),
        };
    }

    pub fn display(
        &mut self,
        data: ObsDisplayCreationData,
    ) -> Result<Pin<Box<ObsDisplayRef>>, ObsError> {
        let display = ObsDisplayRef::new(&self.vertex_buffers, data)
            .map_err(|e| ObsError::DisplayCreationError(e.to_string()))?;

        self.displays.push(display.clone());

        Ok(display)
    }

    pub fn remove_display(&mut self, display: &ObsDisplayRef) {
        self.remove_display_by_id(display.id());
    }

    pub fn remove_display_by_id(&mut self, id: usize) {
        self.displays.retain(|x| x.id() != id);
    }

    pub fn get_output(&mut self, name: &str) -> Option<ObsOutputRef> {
        self.outputs
            .iter()
            .find(|x| x.name().to_string().as_str() == name)
            .map(|e| e.clone())
    }

    pub fn scene(&mut self, name: impl Into<ObsString>) -> ObsSceneRef {
        let scene = ObsSceneRef::new(name.into(), self.active_scene.clone());
        let tmp = scene.clone();

        self.scenes.push(scene);
        tmp
    }
}

#[derive(Debug)]
struct _ObsContextShutdownZST {}

impl Drop for _ObsContextShutdownZST {
    fn drop(&mut self) {
        // Clean up sources
        for i in 0..libobs::MAX_CHANNELS {
            unsafe { libobs::obs_set_output_source(i, ptr::null_mut()) };
        }

        unsafe { libobs::obs_shutdown() }

        let r = LOGGER.lock();
        match r {
            Ok(mut logger) => {
                logger.log(ObsLogLevel::Info, "OBS context shutdown.".to_string());
                let allocs = unsafe { libobs::bnum_allocs() };

                // Increasing this to 1 because of whats described below
                let level = if allocs > 1 {
                    ObsLogLevel::Error
                } else {
                    ObsLogLevel::Info
                };
                // One memory leak is expected here because OBS does not free array elements of the obs_data_path when calling obs_add_data_path
                // even when obs_remove_data_path is called. This is a bug in OBS.
                logger.log(level, format!("Number of memory leaks: {}", allocs))
            }
            Err(_) => {
                println!("OBS context shutdown. (but couldn't lock logger)");
            }
        }

        if let Ok(mut mutex_value) = OBS_THREAD_ID.lock() {
            *mutex_value = None;
        } else if !thread::panicking() {
            panic!()
        }
    }
}
