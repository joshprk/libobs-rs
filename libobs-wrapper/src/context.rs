//! OBS Context Management
//!
//! This module provides the core functionality for interacting with libobs.
//! The primary type is [`ObsContext`], which serves as the main entry point for
//! all OBS operations.
//!
//! # Overview
//!
//! The `ObsContext` represents an initialized OBS environment and provides methods to:
//! - Initialize the OBS runtime
//! - Create and manage scenes
//! - Create and manage outputs (recording, streaming)
//! - Access and configure video/audio settings
//! - Download and bootstrap OBS binaries at runtime
//!
//! # Thread Safety
//!
//! OBS operations must be performed on a single thread. The `ObsContext` handles
//! this requirement by creating a dedicated thread for OBS operations and providing
//! a thread-safe interface to interact with it.
//!
//! # Examples
//!
//! Creating a basic OBS context:
//!
//! ```no_run
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! use libobs_wrapper::context::ObsContext;
//!
//! let context = ObsContext::builder().start().await?;
//! # Ok(())
//! # }
//! ```
//!
//! For more examples refer to the [examples](https://github.com/joshprk/libobs-rs/tree/main/examples) directory in the repository.

use std::{collections::HashMap, ffi::CStr, pin::Pin, sync::Arc, thread::ThreadId};

use crate::{
    data::{output::ObsOutputRef, video::ObsVideoInfo, ObsData},
    display::{ObsDisplayCreationData, ObsDisplayRef},
    enums::{ObsLogLevel, ObsResetVideoStatus},
    logger::LOGGER,
    run_with_obs,
    runtime::{ObsRuntime, ObsRuntimeReturn},
    scenes::ObsSceneRef,
    sources::{ObsFilterRef, ObsSourceBuilder},
    unsafe_send::Sendable,
    utils::{
        FilterInfo, ObsError, ObsModules, ObsString, OutputInfo, StartupInfo
    },
};
use crate::utils::async_sync::{Mutex, RwLock};
use getters0::Getters;
use libobs::{audio_output, obs_scene_t, video_output};

lazy_static::lazy_static! {
    pub(crate) static ref OBS_THREAD_ID: Mutex<Option<ThreadId>> = Mutex::new(None);
}

// Note to developers of this library:
// I've updated everything in the ObsContext to use Rc and RefCell.
// Then the obs context shutdown hook is given to each children of for example scenes and displays.
// That way, obs is not shut down as long as there are still displays or scenes alive.
// This is a bit of a hack, but it works would be glad to hear your thoughts on this.

/// Interface to the OBS context. Only one context
/// can exist across all threads and any attempt to
/// create a new context while there is an existing
/// one will error.
///
/// Note that the order of the struct values is
/// important! OBS is super specific about how it
/// does everything. Things are freed early to
/// latest from top to bottom.
#[derive(Debug, Getters, Clone)]
#[skip_new]
pub struct ObsContext {
    /// Stores startup info for safe-keeping. This
    /// prevents any use-after-free as these do not
    /// get copied in libobs.
    startup_info: Arc<RwLock<StartupInfo>>,

    #[get_mut]
    // Key is display id, value is the display fixed in heap
    displays: Arc<RwLock<HashMap<usize, Arc<Pin<Box<ObsDisplayRef>>>>>>,

    /// Outputs must be stored in order to prevent
    /// early freeing.
    #[allow(dead_code)]
    #[get_mut]
    pub(crate) outputs: Arc<RwLock<Vec<ObsOutputRef>>>,

    #[get_mut]
    pub(crate) scenes: Arc<RwLock<Vec<ObsSceneRef>>>,

    // Filters are on the level of the context because they are not scene specific
    #[get_mut]
    pub(crate) filters: Arc<RwLock<Vec<ObsFilterRef>>>,

    #[skip_getter]
    pub(crate) active_scene: Arc<RwLock<Option<Sendable<*mut obs_scene_t>>>>,

    #[skip_getter]
    pub(crate) _obs_modules: Arc<ObsModules>,

    /// This struct must be the last element which makes sure
    /// that everything else has been freed already before the runtime
    /// shuts down
    pub(crate) runtime: ObsRuntime,
}

#[cfg(not(feature = "bootstrapper"))]
pub type ObsContextReturn = ObsContext;
#[cfg(feature = "bootstrapper")]
pub enum ObsContextReturn {
    /// The OBS context is ready to use
    Done(ObsContext),

    /// The application must be restarted to apply OBS updates
    Restart,
}

impl ObsContext {
    pub fn builder() -> StartupInfo {
        StartupInfo::new()
    }

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
    /// period of time. Unfortunately the memory
    /// leak is caused by a bug in libobs itself.
    ///
    /// If the `bootstrapper` feature is enabled, and ObsContextReturn::Restart is returned,
    /// the application must be restarted to apply the updates and initialization can not continue.
    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn new(info: StartupInfo) -> Result<ObsContextReturn, ObsError> {
        // Spawning runtime, I'll keep this as function for now
        let runtime = ObsRuntime::startup(info).await?;

        #[cfg(feature = "bootstrapper")]
        if matches!(runtime, ObsRuntimeReturn::Restart) {
            return Ok(ObsContextReturn::Restart);
        }

        let (runtime, obs_modules, info) = match runtime {
            ObsRuntimeReturn::Done(r) => r,
            ObsRuntimeReturn::Restart => unreachable!(),
        };

        let context = Self {
            _obs_modules: Arc::new(obs_modules),
            active_scene: Default::default(),
            displays: Default::default(),
            outputs: Default::default(),
            scenes: Default::default(),
            filters: Default::default(),
            runtime,
            startup_info: Arc::new(RwLock::new(info)),
        };

        #[cfg(feature = "bootstrapper")]
        return Ok(ObsContextReturn::Done(context));

        #[cfg(not(feature = "bootstrapper"))]
        return Ok(context);
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn get_version(&self) -> Result<String, ObsError> {
        let res = run_with_obs!(self.runtime, || unsafe {
            let version = libobs::obs_get_version_string();
            let version_cstr = CStr::from_ptr(version);

            version_cstr.to_string_lossy().into_owned()
        }).await?;

        Ok(res)
    }

    pub fn log(&self, level: ObsLogLevel, msg: &str) {
        let mut log = LOGGER.lock().unwrap();
        log.log(level, msg.to_string());
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
    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn reset_video(&mut self, ovi: ObsVideoInfo) -> Result<(), ObsError> {
        // You cannot change the graphics module without
        // completely destroying the entire OBS context.
        if self
            .startup_info
            .read()
            .await
            .obs_video_info
            .graphics_module()
            != ovi.graphics_module()
        {
            return Err(ObsError::ResetVideoFailureGraphicsModule);
        }

        // Resets the video context. Note that this
        // is similar to Self::reset_video, but it
        // does not call that function because the
        // ObsContext struct is not created yet,
        // and also because there is no need to free
        // anything tied to the OBS context.
        let mut vid = self.startup_info.write().await;
        let vid_ptr = Sendable(vid.obs_video_info.as_ptr());

        let reset_video_status = run_with_obs!(self.runtime, (vid_ptr), move || unsafe {
            libobs::obs_reset_video(vid_ptr)
        }).await?;

        drop(vid);
        let reset_video_status = num_traits::FromPrimitive::from_i32(reset_video_status);

        let reset_video_status = match reset_video_status {
            Some(x) => x,
            None => ObsResetVideoStatus::Failure,
        };

        if reset_video_status != ObsResetVideoStatus::Success {
            return Err(ObsError::ResetVideoFailure(reset_video_status));
        } else {
            let outputs = self.outputs.read().await.clone();
            let mut video_encoders = vec![];

            for output in outputs.iter() {
                let encoders = output.get_video_encoders().await;
                video_encoders.extend(encoders.into_iter().map(|e| e.as_ptr()));
            }

            let vid_ptr = self.get_video_ptr().await?;
            run_with_obs!(self.runtime, (vid_ptr), move || unsafe {
                for encoder_ptr in video_encoders.into_iter() {
                    libobs::obs_encoder_set_video(encoder_ptr.0, vid_ptr);
                }
            }).await?;

            self.startup_info.write().await.obs_video_info = ovi;
            return Ok(());
        }
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn get_video_ptr(&self) -> Result<Sendable<*mut video_output>, ObsError> {
        // Removed safeguards here because ptr are not sendable and this OBS context should never be used across threads
        run_with_obs!(self.runtime, || unsafe {
            Sendable(libobs::obs_get_video())
        }).await
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn get_audio_ptr(&self) -> Result<Sendable<*mut audio_output>, ObsError> {
        // Removed safeguards here because ptr are not sendable and this OBS context should never be used across threads
        run_with_obs!(self.runtime, || unsafe {
            Sendable(libobs::obs_get_audio())
        }).await
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn data(&self) -> Result<ObsData, ObsError> {
        ObsData::new(self.runtime.clone()).await
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn output(&mut self, info: OutputInfo) -> Result<ObsOutputRef, ObsError> {
        let output = ObsOutputRef::new(info, self.runtime.clone()).await;

        return match output {
            Ok(x) => {
                let tmp = x.clone();
                self.outputs.write().await.push(x);
                Ok(tmp)
            }

            Err(x) => Err(x),
        };
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn obs_filter(&mut self, info: FilterInfo) -> Result<ObsFilterRef, ObsError> {
        let filter = ObsFilterRef::new(info.id, info.name, info.settings, info.hotkey_data, self.runtime.clone()).await;

        return match filter {
            Ok(x) => {
                let tmp = x.clone();
                self.filters.write().await.push(x);
                Ok(tmp)
            }

            Err(x) => Err(x),
        };
    }

    /// Creates a new display and returns its ID.
    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn display(
        &mut self,
        data: ObsDisplayCreationData,
    ) -> Result<Pin<Box<ObsDisplayRef>>, ObsError> {
        let display = ObsDisplayRef::new(data, self.runtime.clone())
            .await
            .map_err(|e| ObsError::DisplayCreationError(e.to_string()))?;

        let display_clone = display.clone();

        let id = display.id();
        self.displays.write().await.insert(id, Arc::new(display));
        Ok(display_clone)
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn remove_display(&mut self, display: &ObsDisplayRef) {
        self.remove_display_by_id(display.id()).await;
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn remove_display_by_id(&mut self, id: usize) {
        self.displays.write().await.remove(&id);
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn get_display_by_id(&self, id: usize) -> Option<Arc<Pin<Box<ObsDisplayRef>>>> {
        self.displays.read().await.get(&id).cloned()
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn get_output(&mut self, name: &str) -> Option<ObsOutputRef> {
        self.outputs
            .read()
            .await
            .iter()
            .find(|x| x.name().to_string().as_str() == name)
            .map(|e| e.clone())
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn update_output(&mut self, name: &str, settings: ObsData) -> Result<(), ObsError> {
        match self
            .outputs
            .write()
            .await
            .iter_mut()
            .find(|x| x.name().to_string().as_str() == name)
        {
            Some(output) => output.update_settings(settings).await,
            None => Err(ObsError::OutputNotFound),
        }
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn get_filter(&mut self, name: &str) -> Option<ObsFilterRef> {
        self.filters
            .read()
            .await
            .iter()
            .find(|x| x.name().to_string().as_str() == name)
            .map(|e| e.clone())
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn scene<T: Into<ObsString> + Send + Sync>(
        &mut self,
        name: T,
    ) -> Result<ObsSceneRef, ObsError> {
        let scene =
            ObsSceneRef::new(name.into(), self.active_scene.clone(), self.runtime.clone()).await?;

        let tmp = scene.clone();
        self.scenes.write().await.push(scene);

        Ok(tmp)
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn get_scene(&mut self, name: &str) -> Option<ObsSceneRef> {
        self.scenes
            .read()
            .await
            .iter()
            .find(|x| x.name().to_string().as_str() == name)
            .map(|e| e.clone())
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn source_builder<T: ObsSourceBuilder, K: Into<ObsString> + Send + Sync>(
        &self,
        name: K,
    ) -> Result<T, ObsError> {
        T::new(name.into(), self.runtime.clone()).await
    }
}
