use std::{collections::HashMap, ffi::CStr, pin::Pin, sync::Arc, thread::ThreadId};

use crate::{
    data::{output::ObsOutputRef, video::ObsVideoInfo, ObsData},
    display::{ObsDisplayCreationData, ObsDisplayRef},
    enums::{ObsLogLevel, ObsResetVideoStatus},
    logger::LOGGER,
    run_with_obs,
    runtime::{ObsRuntime, ObsRuntimeReturn},
    scenes::ObsSceneRef,
    unsafe_send::Sendable,
    utils::{ObsError, ObsModules, ObsString, OutputInfo, StartupInfo},
};
use anyhow::Result;
use futures::future::join_all;
use getters0::Getters;
use libobs::{audio_output, obs_scene_t, video_output};
use tokio::sync::{Mutex, RwLock};
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

    #[skip_getter]
    pub(crate) active_scene: Arc<RwLock<Option<Sendable<*mut obs_scene_t>>>>,

    #[skip_getter]
    pub(crate) _obs_modules: Arc<ObsModules>,

    #[skip_getter]
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
    pub async fn new(info: StartupInfo) -> Result<ObsContextReturn, ObsError> {
        // Spawning runtime, I'll keep this as function for now
        let runtime = ObsRuntime::startup(info).await?;

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
            runtime,
            startup_info: Arc::new(RwLock::new(info)),
        };

        #[cfg(feature = "bootstrapper")]
        return Ok(ObsContextReturn::Done(context));

        #[cfg(not(feature = "bootstrapper"))]
        return Ok(context);
    }

    pub async fn get_version(&self) -> Result<String, ObsError> {
        let res = run_with_obs!(self, || {
            let version = libobs::obs_get_version_string();
            let version_cstr = CStr::from_ptr(version);

            Ok(version_cstr.to_string_lossy().into_owned())
        })?;

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
    pub async fn reset_video(&mut self, mut ovi: ObsVideoInfo) -> Result<(), ObsError> {
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
        let vid_ptr = self.startup_info.read().await.obs_video_info.as_ptr();
        let reset_video_status = run_with_obs!(self, || Ok(libobs::obs_reset_video(vid_ptr)))?;
        let reset_video_status = num_traits::FromPrimitive::from_i32(reset_video_status);

        let reset_video_status = match reset_video_status {
            Some(x) => x,
            None => ObsResetVideoStatus::Failure,
        };

        if reset_video_status != ObsResetVideoStatus::Success {
            return Err(ObsError::ResetVideoFailure(reset_video_status));
        } else {
            let outputs = self
                .outputs
                .read()
                .await
                .clone()
                .into_iter()
                .map(|x| x.get_video_encoders())
                .collect::<Vec<_>>();

            let video_encoders = join_all(outputs)
                .await
                .into_iter()
                .map(|e| e.into_iter().map(|e| Sendable(e.as_ptr())))
                .flatten()
                .collect::<Vec<_>>();

            let vid_ptr = self.get_video_ptr().await.unwrap();
            run_with_obs!(self, (vid_ptr), || {
                for encoder_ptr in video_encoders.into_iter() {
                    libobs::obs_encoder_set_video(encoder_ptr.0, vid_ptr);
                }

                Ok(())
            })?;

            self.startup_info.write().await.obs_video_info = ovi;
            return Ok(());
        }
    }

    pub async fn get_video_ptr(&self) -> Result<*mut video_output, ObsError> {
        // Removed safeguards here because ptr are not sendable and this OBS context should never be used across threads
        Ok(run_with_obs!(self, || { Ok(Sendable(libobs::obs_get_video())) })?.0)
    }

    pub async fn get_audio_ptr(&self) -> Result<*mut audio_output, ObsError> {
        // Removed safeguards here because ptr are not sendable and this OBS context should never be used across threads
        Ok(run_with_obs!(self, || { Ok(Sendable(libobs::obs_get_audio())) })?.0)
    }

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

    /// Creates a new display and returns its ID.
    pub async fn display(&mut self, data: ObsDisplayCreationData) -> Result<usize, ObsError> {
        let display = ObsDisplayRef::new(data, self.runtime.clone())
            .map_err(|e| ObsError::DisplayCreationError(e.to_string()))?;

        let id = display.id();
        self.displays.write().await.insert(id, Arc::new(display));
        Ok(id)
    }

    pub async fn remove_display(&mut self, display: &ObsDisplayRef) {
        self.remove_display_by_id(display.id()).await;
    }

    pub async fn remove_display_by_id(&mut self, id: usize) {
        self.displays.write().await.remove(&id);
    }

    pub async fn get_display_by_id(&self, id: usize) -> Option<Arc<Pin<Box<ObsDisplayRef>>>> {
        self.displays.read().await.get(&id).cloned()
    }

    pub async fn get_output(&mut self, name: &str) -> Option<ObsOutputRef> {
        self.outputs
            .read().await
            .iter()
            .find(|x| x.name().to_string().as_str() == name)
            .map(|e| e.clone())
    }

    pub async fn update_output(&mut self, name: &str, settings: ObsData) -> Result<(), ObsError> {
        match self
            .outputs
            .write().await
            .iter_mut()
            .find(|x| x.name().to_string().as_str() == name)
        {
            Some(output) => output.update_settings(settings),
            None => Err(ObsError::OutputNotFound),
        }
    }

    pub async fn scene(&mut self, name: impl Into<ObsString>) -> ObsSceneRef {
        let scene = ObsSceneRef::new(
            name.into(),
            self.active_scene.clone(),
            self.runtime.clone(),
        );

        let tmp = scene.clone();
        self.scenes.write().await.push(scene);

        tmp
    }

    pub async fn get_scene(&mut self, name: &str) -> Option<ObsSceneRef> {
        self.scenes
            .read().await
            .iter()
            .find(|x| x.name().to_string().as_str() == name)
            .map(|e| e.clone())
    }
}
