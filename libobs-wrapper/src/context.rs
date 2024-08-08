use std::{
    ffi::{c_char, CStr}, ptr, sync::{Arc, Mutex}, thread::{self, ThreadId}
};

use crate::{
    data::{output::ObsOutput, video::ObsVideoInfo},
    display::{ObsDisplay, ObsDisplayCreationData, VertexBuffers},
    enums::{ObsLogLevel, ObsResetVideoStatus, ObsVideoEncoderType},
    logger::{extern_log_callback, LOGGER},
    scenes::ObsScene,
    unsafe_send::WrappedObsScene,
    utils::{ObsError, ObsString, OutputInfo, StartupInfo},
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
    displays: Vec<ObsDisplay>,

    #[skip_getter]
    vertex_buffers: VertexBuffers,

    /// Outputs must be stored in order to prevent
    /// early freeing.
    #[allow(dead_code)]
    #[get_mut]
    pub(crate) outputs: Vec<ObsOutput>,

    #[get_mut]
    pub(crate) scenes: Vec<ObsScene>,

    #[skip_getter]
    pub(crate) active_scene: Arc<Mutex<Option<WrappedObsScene>>>,

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
        format!("{}.{}.{}", libobs::LIBOBS_API_MAJOR_VER, libobs::LIBOBS_API_MINOR_VER, libobs::LIBOBS_API_PATCH_VER)
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

        let mut log_callback = LOGGER.lock()
        .map_err(|_e| ObsError::MutexFailure)?;

        *log_callback = info.logger.take().expect("Logger can never be null");

        drop(log_callback);

        // Locale will only be used internally by
        // libobs for logging purposes, making it
        // unnecessary to support other languages.
        let locale_str = ObsString::new("en-US");
        let startup_status =
            unsafe { libobs::obs_startup(locale_str.as_ptr(), ptr::null(), ptr::null_mut()) };

        let mut log = LOGGER.lock().unwrap();
        log.log(ObsLogLevel::Info, format!("OBS {}", Self::get_version()));
        log.log(ObsLogLevel::Info, "---------------------------------".to_string());
        drop(log);

        if !startup_status {
            return Err(ObsError::Failure);
        }

        unsafe {
            libobs::obs_add_data_path(info.startup_paths.libobs_data_path().as_ptr());
            libobs::obs_add_module_path(
                info.startup_paths.plugin_bin_path().as_ptr(),
                info.startup_paths.plugin_data_path().as_ptr(),
            );
        }

        // Note that audio is meant to only be reset
        // once. See the link below for information.
        //
        // https://docs.obsproject.com/frontends
        unsafe {
            libobs::obs_reset_audio(info.obs_audio_info.as_ptr());
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

        unsafe {
            libobs::obs_load_all_modules();
            libobs::obs_post_load_modules();
            libobs::obs_log_loaded_modules();
        }

        let vertex_buffers = unsafe { VertexBuffers::initialize() };

        Ok(Self {
            locale: locale_str,
            startup_info: info,
            outputs: vec![],
            vertex_buffers,
            displays: vec![],
            active_scene: Arc::new(Mutex::new(None)),
            scenes: vec![],
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
            for output in self.outputs.iter_mut() {
                for video_encoder in output.get_video_encoders().iter_mut() {
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

    fn reset_video_internal(ovi: &mut ObsVideoInfo) -> ObsResetVideoStatus {
        let status =
            num_traits::FromPrimitive::from_i32(unsafe { libobs::obs_reset_video(ovi.as_ptr()) });

        return match status {
            Some(x) => x,
            None => ObsResetVideoStatus::Failure,
        };
    }

    pub fn output(&mut self, info: OutputInfo) -> Result<&mut ObsOutput, ObsError> {
        let output = ObsOutput::new(info.id, info.name, info.settings, info.hotkey_data);

        return match output {
            Ok(x) => {
                self.outputs.push(x);
                Ok(self.outputs.last_mut().unwrap())
            }

            Err(x) => Err(x),
        };
    }

    pub fn display(&mut self, data: ObsDisplayCreationData) -> Result<&mut ObsDisplay, ObsError> {
        self.displays.push(ObsDisplay::new(
            &self.vertex_buffers,
            &self.active_scene,
            data,
        ));

        Ok(self.displays.last_mut().unwrap())
    }

    pub fn remove_display(&mut self, display: &ObsDisplay) {
        self.remove_display_by_id(display.get_id());
    }

    pub fn remove_display_by_id(&mut self, id: usize) {
        self.displays.retain(|x| x.get_id() != id);
    }

    pub fn get_output(&mut self, name: &str) -> Option<&mut ObsOutput> {
        self.outputs
            .iter_mut()
            .find(|x| x.name().to_string().as_str() == name)
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

    pub fn get_best_encoder() -> ObsVideoEncoderType {
        Self::get_available_encoders().first().unwrap().clone()
    }

    pub fn get_available_encoders() -> Vec<ObsVideoEncoderType> {
        // from https://github.com/FFFFFFFXXXXXXX/libobs-recorder
        let mut n = 0;
        let mut encoders = Vec::new();
        let mut ptr: *const c_char = unsafe { std::mem::zeroed() };
        while unsafe { libobs::obs_enum_encoder_types(n, &mut ptr) } {
            n += 1;
            let cstring = unsafe { CStr::from_ptr(ptr) };
            if let Ok(enc) = cstring.to_str() {
                encoders.push(enc.into());
            }
        }
        encoders.sort_unstable();
        encoders
    }

    pub fn scene(&mut self, name: impl Into<ObsString>) -> &mut ObsScene {
        let scene = ObsScene::new(name.into(), self.active_scene.clone());
        self.scenes.push(scene);

        self.scenes.last_mut().unwrap()
    }
}

#[derive(Debug)]
struct _ObsContextShutdownZST {}

impl Drop for _ObsContextShutdownZST {
    fn drop(&mut self) {
        unsafe { libobs::obs_shutdown() }

        let r = LOGGER.lock();
        match r {
            Ok(mut logger) => {
                logger.log(ObsLogLevel::Info, "OBS context shutdown.".to_string());
                logger.log(ObsLogLevel::Info, format!("Number of memory leaks: {}", unsafe { libobs::bnum_allocs() }))
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
