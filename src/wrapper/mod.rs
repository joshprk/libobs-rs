#![warn(non_upper_case_globals)]
#![warn(non_camel_case_types)]
#![warn(non_snake_case)]
#![warn(clippy::approx_constant)]
#![warn(clippy::unreadable_literal)]
#![warn(rustdoc::bare_urls)]

mod obstypes;

use std::ffi::{c_char, CStr};
use std::{sync::Mutex, thread};
use std::thread::ThreadId;
use std::ptr;

pub use obstypes::{ObsAudioInfo, ObsData, ObsError, ObsVideoEncoderType, ObsPath, ObsResetVideoStatus, ObsString, ObsVideoInfo, ObsVideoInfoBuilder};
use obstypes::ObsOutput;

use crate::{audio_output, video_output};

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
#[derive(Debug)]
pub struct ObsContext {
    /// This string must be stored to keep the
    /// pointer passed to libobs valid.
    #[allow(dead_code)]
    locale: ObsString,
    /// Stores startup info for safe-keeping. This
    /// prevents any use-after-free as these do not
    /// get copied in libobs.
    startup_info: StartupInfo,
    /// Outputs must be stored in order to prevent
    /// early freeing.
    #[allow(dead_code)]
    outputs: Vec<ObsOutput>,
    /// This allows us to call obs_shutdown() after
    /// everything else has been freed. Doing other-
    /// wise completely crashes the program.
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
                return Err(ObsError::ThreadFailure)
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
        // Locale will only be used internally by 
        // libobs for logging purposes, making it
        // unnecessary to support other languages.
        let locale_str = ObsString::new("en-US");
        let startup_status = unsafe { crate::obs_startup(
            locale_str.as_ptr(), 
            ptr::null(), 
            ptr::null_mut()
        ) };

        if !startup_status {
            return Err(ObsError::Failure)
        }
        
        unsafe { 
            crate::obs_add_data_path(
                info.startup_paths.libobs_data_path().as_ptr()
            );
        }
        
        // Resets the video context. Note that this
        // is similar to Self::reset_video, but it
        // does not call that function because the
        // ObsContext struct is not created yet,
        // and also because there is no need to free
        // anything tied to the OBS context.
        let reset_video_status = Self::reset_video_internal(&mut info.obs_video_info);

        if reset_video_status != ObsResetVideoStatus::Success {
            return Err(ObsError::ResetVideoFailure(reset_video_status))
        }

        // Note that audio is meant to only be reset
        // once. See the link below for information.
        //
        // https://docs.obsproject.com/frontends
        unsafe {
            crate::obs_reset_audio(info.obs_audio_info.as_ptr());
        }
        
        unsafe {
            crate::obs_add_module_path(
                info.startup_paths.plugin_bin_path().as_ptr(), 
                info.startup_paths.plugin_data_path().as_ptr()
            );

            crate::obs_load_all_modules();
            crate::obs_post_load_modules();
            crate::obs_log_loaded_modules();
        }

        Ok(Self {
            locale: locale_str,
            startup_info: info,
            outputs: vec![],
            _context_shutdown_zst: _ObsContextShutdownZST {},
        })
    }

    /// Resets the OBS video context. This is often called
    /// when one wants to change a setting related to the
    /// OBS video info sent on startup.
    /// 
    /// Note that you cannot reset the graphics module
    /// without destroying the entire OBS context. Trying 
    /// so will result in an error.
    pub fn reset_video(&mut self, mut ovi: ObsVideoInfo) -> Result<(), ObsError> {
        // You cannot change the graphics module without
        // completely destroying the entire OBS context.
        if self.startup_info.obs_video_info.graphics_module() != ovi.graphics_module() {
            return Err(ObsError::ResetVideoFailureGraphicsModule)
        }

        let reset_video_status = Self::reset_video_internal(&mut ovi);
        
        if reset_video_status != ObsResetVideoStatus::Success {
            return Err(ObsError::ResetVideoFailure(reset_video_status))
        } else {
            self.startup_info.obs_video_info = ovi;
            return Ok(())
        }
    }

    fn reset_video_internal(ovi: &mut ObsVideoInfo) -> ObsResetVideoStatus {
        let status = num_traits::FromPrimitive::from_i32(
            unsafe { crate::obs_reset_video(ovi.as_ptr()) }
        );
        
        return match status {
            Some(x) => x,
            None    => ObsResetVideoStatus::Failure,
        }
    }

    pub fn output(&mut self, info: OutputInfo) -> Result<&mut ObsOutput, ObsError> {
        let output = ObsOutput::new(
            info.id,
            info.name, 
            info.settings, 
            info.hotkey_data,
        );

        return match output {
            Ok(x)   => {
                self.outputs.push(x);
                Ok(self.outputs.last_mut().unwrap())
            },

            Err(x)  => Err(x),
        }
    }

    pub fn get_video_ptr() -> Result<*mut video_output, ObsError> {
        if let Ok(mutex_value) = OBS_THREAD_ID.lock() {
            if *mutex_value != Some(thread::current().id()) {
                return Err(ObsError::ThreadFailure)
            } 
        } else {
            return Err(ObsError::MutexFailure)
        }

        Ok(unsafe { crate::obs_get_video() })
    }
    
    pub fn get_audio_ptr() -> Result<*mut audio_output, ObsError> {
        if let Ok(mutex_value) = OBS_THREAD_ID.lock() {
            if *mutex_value != Some(thread::current().id()) {
                return Err(ObsError::ThreadFailure)
            } 
        } else {
            return Err(ObsError::MutexFailure)
        }

        Ok(unsafe { crate::obs_get_audio() })
    }

    pub fn get_best_encoder() -> ObsVideoEncoderType {
        Self::get_available_encoders()
            .first()
            .unwrap()
            .clone()
    }

    pub fn get_available_encoders() -> Vec<ObsVideoEncoderType> {
        // from https://github.com/FFFFFFFXXXXXXX/libobs-recorder
        let mut n = 0;
        let mut encoders = Vec::new();
        let mut ptr: *const c_char = unsafe { std::mem::zeroed() };
        while unsafe { crate::obs_enum_encoder_types(n, &mut ptr) } {
            n += 1;
            let cstring = unsafe { CStr::from_ptr(ptr) };
            if let Ok(enc) = cstring.to_str() {
                encoders.push(enc.into());
            }
        }
        encoders.sort_unstable();
        encoders
    }
}

pub type VideoEncoderInfo = OutputInfo;
pub type AudioEncoderInfo = ObjectInfo;
pub type SourceInfo = ObjectInfo;
pub type OutputInfo = ObjectInfo;

#[derive(Debug)]
pub struct ObjectInfo {
    id: ObsString,
    name: ObsString,
    settings: Option<ObsData>,
    hotkey_data: Option<ObsData>,
}

impl ObjectInfo {
    pub fn new(
        id: impl Into<ObsString>, 
        name: impl Into<ObsString>, 
        settings: Option<ObsData>, 
        hotkey_data: Option<ObsData>
    ) -> Self {
        let id = id.into();
        let name = name.into();

        Self {
            id,
            name,
            settings,
            hotkey_data,
        }
    }
}

/// Contains information to start a libobs context.
/// This is passed to the creation of `ObsContext`.
#[derive(Clone, Debug)]
pub struct StartupInfo {
    startup_paths: StartupPaths,
    obs_video_info: ObsVideoInfo,
    obs_audio_info: ObsAudioInfo,
}

impl StartupInfo {
    pub fn new() -> StartupInfo {
        Self::default()
    }

    pub fn set_startup_paths(mut self, paths: StartupPaths) -> Self {
        self.startup_paths = paths;
        self
    }

    pub fn set_video_info(mut self, ovi: ObsVideoInfo) -> Self {
        self.obs_video_info = ovi;
        self
    }
}

impl Default for StartupInfo {
    fn default() -> StartupInfo {
        Self {
            startup_paths: StartupPaths::default(),
            obs_video_info: ObsVideoInfo::default(),
            obs_audio_info: ObsAudioInfo::default(),
        }
    }
}

/// Contains the necessary paths for starting the
/// libobs context built from `ObsPath`.
/// 
/// Note that these strings are copied when parsed,
/// meaning that these can be freed immediately
/// after all three strings have been used.
#[derive(Clone, Debug)]
pub struct StartupPaths {
    libobs_data_path: ObsString,
    plugin_bin_path: ObsString,
    plugin_data_path: ObsString,
}

impl StartupPaths {
    pub fn new(libobs_data_path: ObsPath, plugin_bin_path: ObsPath, plugin_data_path: ObsPath) -> StartupPaths {
        Self {
            libobs_data_path: libobs_data_path.build(),
            plugin_bin_path: plugin_bin_path.build(),
            plugin_data_path: plugin_data_path.build(),
        }
    }

    pub fn libobs_data_path(&self) -> &ObsString {
        &(self.libobs_data_path)
    }

    pub fn plugin_bin_path(&self) -> &ObsString {
        &(self.plugin_bin_path)
    }

    pub fn plugin_data_path(&self) -> &ObsString {
        &(self.plugin_data_path)
    }
}

impl Default for StartupPaths {
    fn default() -> Self {
        StartupPathsBuilder::new().build()
    }
}

#[derive(Clone, Debug)]
pub struct StartupPathsBuilder {
    libobs_data_path: ObsPath,
    plugin_bin_path: ObsPath,
    plugin_data_path: ObsPath,
}

impl StartupPathsBuilder {
    fn new() -> Self {
        Self {
            libobs_data_path: ObsPath::from_relative("data/libobs"),
            plugin_bin_path: ObsPath::from_relative("obs-plugins/64bit"),
            plugin_data_path: ObsPath::from_relative("data/obs-plugins"),
        }
    }

    pub fn build(self) -> StartupPaths {
        StartupPaths {
            libobs_data_path: self.libobs_data_path.build(),
            plugin_bin_path: self.plugin_bin_path.build(),
            plugin_data_path: self.plugin_data_path.build(),
        }
    }

    pub fn libobs_data_path(mut self, value: ObsPath) -> Self {
        self.libobs_data_path = value;
        self
    }

    pub fn plugin_bin_path(mut self, value: ObsPath) -> Self {
        self.plugin_bin_path = value;
        self
    }

    pub fn plugin_data_path(mut self, value: ObsPath) -> Self {
        self.plugin_data_path = value;
        self
    }
}

impl Default for StartupPathsBuilder {
    fn default() -> StartupPathsBuilder {
        Self::new()
    }
}

#[derive(Debug)]
struct _ObsContextShutdownZST {}

impl Drop for _ObsContextShutdownZST {
    fn drop(&mut self) {
        unsafe { crate::obs_shutdown() }
        
        if let Ok(mut mutex_value) = OBS_THREAD_ID.lock() {
            *mutex_value = None;
        } else if !thread::panicking() {
            panic!()
        }
    }
}