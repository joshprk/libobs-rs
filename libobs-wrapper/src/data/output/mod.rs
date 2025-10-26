use std::sync::{Arc, RwLock};
use std::{ffi::CStr, ptr};

use anyhow::bail;
use getters0::Getters;
use libobs::{
    audio_output, obs_encoder_set_audio, obs_encoder_set_video, obs_output, obs_output_active,
    obs_output_create, obs_output_get_last_error, obs_output_pause, obs_output_release,
    obs_output_set_audio_encoder, obs_output_set_video_encoder, obs_output_start, obs_output_stop,
    obs_output_update, video_output,
};

use crate::enums::ObsOutputStopSignal;
use crate::runtime::ObsRuntime;
use crate::unsafe_send::Sendable;
use crate::utils::{AudioEncoderInfo, OutputInfo, VideoEncoderInfo};
use crate::{impl_obs_drop, impl_signal_manager, run_with_obs};

use crate::{
    encoders::{audio::ObsAudioEncoder, video::ObsVideoEncoder},
    utils::{ObsError, ObsString},
};

use super::ObsData;

mod replay_buffer;
pub use replay_buffer::*;

#[derive(Debug)]
struct _ObsDropGuard {
    output: Sendable<*mut obs_output>,
    runtime: ObsRuntime,
}

impl_obs_drop!(_ObsDropGuard, (output), move || unsafe {
    obs_output_release(output);
});

#[derive(Debug, Getters, Clone)]
#[skip_new]
/// A reference to an OBS output.
///
/// This struct represents an output in OBS, which is responsible for
/// outputting encoded audio and video data to a destination such as:
/// - A file (recording)
/// - A streaming service (RTMP, etc.)
/// - A replay buffer
///
/// The output is associated with video and audio encoders that convert
/// raw media to the required format before sending/storing.
pub struct ObsOutputRef {
    /// Settings for the output
    pub(crate) settings: Arc<RwLock<Option<ObsData>>>,

    /// Hotkey configuration data for the output
    pub(crate) hotkey_data: Arc<RwLock<Option<ObsData>>>,

    /// Video encoders attached to this output
    #[get_mut]
    pub(crate) video_encoders: Arc<RwLock<Vec<Arc<ObsVideoEncoder>>>>,

    /// Audio encoders attached to this output
    #[get_mut]
    pub(crate) audio_encoders: Arc<RwLock<Vec<Arc<ObsAudioEncoder>>>>,

    /// Pointer to the underlying OBS output
    #[skip_getter]
    pub(crate) output: Sendable<*mut obs_output>,

    /// The type identifier of this output
    pub(crate) id: ObsString,

    /// The unique name of this output
    pub(crate) name: ObsString,

    /// RAII guard that ensures proper cleanup when the output is dropped
    #[skip_getter]
    _drop_guard: Arc<_ObsDropGuard>,

    #[skip_getter]
    pub(crate) runtime: ObsRuntime,

    pub(crate) signal_manager: Arc<ObsOutputSignals>,
}

impl ObsOutputRef {
    /// Creates a new output reference from the given output info and runtime.
    ///
    /// # Arguments
    /// * `output` - The output information containing ID, name, and optional settings
    /// * `runtime` - The OBS runtime instance
    ///
    /// # Returns
    /// A Result containing the new ObsOutputRef or an error
    pub(crate) fn new(output: OutputInfo, runtime: ObsRuntime) -> Result<Self, ObsError> {
        let (output, id, name, settings, hotkey_data) = runtime
            .run_with_obs_result(|| {
                let OutputInfo {
                    id,
                    name,
                    settings,
                    hotkey_data,
                } = output;

                let settings_ptr = match settings.as_ref() {
                    Some(x) => x.as_ptr(),
                    None => Sendable(ptr::null_mut()),
                };

                let hotkey_data_ptr = match hotkey_data.as_ref() {
                    Some(x) => x.as_ptr(),
                    None => Sendable(ptr::null_mut()),
                };

                let output = unsafe {
                    obs_output_create(
                        id.as_ptr().0,
                        name.as_ptr().0,
                        settings_ptr.0,
                        hotkey_data_ptr.0,
                    )
                };

                if output.is_null() {
                    bail!("Null pointer returned from obs_output_create");
                }

                Ok((Sendable(output), id, name, settings, hotkey_data))
            })
            .map_err(|e| ObsError::InvocationError(e.to_string()))?
            .map_err(|_| ObsError::NullPointer)?;

        let signal_manager = ObsOutputSignals::new(&output, runtime.clone())?;
        Ok(Self {
            settings: Arc::new(RwLock::new(settings)),
            hotkey_data: Arc::new(RwLock::new(hotkey_data)),

            video_encoders: Arc::new(RwLock::new(vec![])),
            audio_encoders: Arc::new(RwLock::new(vec![])),

            output: output.clone(),
            id,
            name,

            _drop_guard: Arc::new(_ObsDropGuard {
                output,
                runtime: runtime.clone(),
            }),

            runtime,
            signal_manager: Arc::new(signal_manager),
        })
    }

    /// Returns a list of all video encoders attached to this output.
    ///
    /// # Returns
    /// A vector of Arc-wrapped ObsVideoEncoder instances
    pub fn get_video_encoders(&self) -> Result<Vec<Arc<ObsVideoEncoder>>, ObsError> {
        self.video_encoders
            .read()
            .map_err(|e| ObsError::LockError(e.to_string()))
            .map(|x| x.clone())
    }

    /// Creates and attaches a new video encoder to this output.
    ///
    /// This method creates a new video encoder using the provided information,
    /// sets up the video handler, and attaches it to this output.
    ///
    /// # Arguments
    /// * `info` - Information for creating the video encoder
    /// * `handler` - The video output handler
    ///
    /// # Returns
    /// A Result containing an Arc-wrapped ObsVideoEncoder or an error
    pub fn video_encoder(
        &mut self,
        info: VideoEncoderInfo,
        handler: Sendable<*mut video_output>,
    ) -> Result<Arc<ObsVideoEncoder>, ObsError> {
        let video_enc = ObsVideoEncoder::new(
            info.id,
            info.name,
            info.settings,
            info.hotkey_data,
            self.runtime.clone(),
        )?;

        let encoder_ptr = video_enc.encoder.clone();
        let output_ptr = self.output.clone();
        let handler = Sendable(handler);

        run_with_obs!(
            self.runtime,
            (encoder_ptr, output_ptr, handler),
            move || unsafe {
                obs_encoder_set_video(encoder_ptr, handler.0);
                obs_output_set_video_encoder(output_ptr, encoder_ptr);
            }
        )?;

        let tmp = Arc::new(video_enc);
        self.video_encoders
            .write()
            .map_err(|e| ObsError::LockError(e.to_string()))?
            .push(tmp.clone());

        Ok(tmp)
    }

    /// Attaches an existing video encoder to this output.
    ///
    /// # Arguments
    /// * `encoder` - The video encoder to attach
    ///
    /// # Returns
    /// A Result indicating success or an error
    pub fn set_video_encoder(&mut self, encoder: ObsVideoEncoder) -> Result<(), ObsError> {
        if encoder.encoder.0.is_null() {
            return Err(ObsError::NullPointer);
        }

        let output = self.output.clone();
        let encoder_ptr = encoder.as_ptr();

        run_with_obs!(self.runtime, (output, encoder_ptr), move || unsafe {
            obs_output_set_video_encoder(output, encoder_ptr);
        })?;

        if !self
            .video_encoders
            .read()
            .map_err(|e| ObsError::LockError(e.to_string()))?
            .iter()
            .any(|x| x.encoder.0 == encoder.as_ptr().0)
        {
            let tmp = Arc::new(encoder);

            self.video_encoders
                .write()
                .map_err(|e| ObsError::LockError(e.to_string()))?
                .push(tmp.clone());
        }

        Ok(())
    }

    /// Updates the settings of this output.
    ///
    /// Note: This can only be done when the output is not active.
    ///
    /// # Arguments
    /// * `settings` - The new settings to apply
    ///
    /// # Returns
    /// A Result indicating success or an error
    pub fn update_settings(&mut self, settings: ObsData) -> Result<(), ObsError> {
        let output = self.output.clone();
        let output_active = run_with_obs!(self.runtime, (output), move || unsafe {
            obs_output_active(output)
        })?;

        if !output_active {
            let settings_ptr = settings.as_ptr();

            run_with_obs!(self.runtime, (output, settings_ptr), move || unsafe {
                obs_output_update(output, settings_ptr)
            })?;

            self.settings
                .write()
                .map_err(|e| ObsError::LockError(e.to_string()))?
                .replace(settings);
            Ok(())
        } else {
            Err(ObsError::OutputAlreadyActive)
        }
    }

    /// Creates and attaches a new audio encoder to this output.
    ///
    /// This method creates a new audio encoder using the provided information,
    /// sets up the audio handler, and attaches it to this output at the specified mixer index.
    ///
    /// # Arguments
    /// * `info` - Information for creating the audio encoder
    /// * `mixer_idx` - The mixer index to use (typically 0 for primary audio)
    /// * `handler` - The audio output handler
    ///
    /// # Returns
    /// A Result containing an Arc-wrapped ObsAudioEncoder or an error
    pub fn audio_encoder(
        &mut self,
        info: AudioEncoderInfo,
        mixer_idx: usize,
        handler: Sendable<*mut audio_output>,
    ) -> Result<Arc<ObsAudioEncoder>, ObsError> {
        let audio_enc = ObsAudioEncoder::new(
            info.id,
            info.name,
            info.settings,
            mixer_idx,
            info.hotkey_data,
            self.runtime.clone(),
        )?;

        let encoder_ptr = audio_enc.encoder.clone();
        let output_ptr = self.output.clone();

        run_with_obs!(
            self.runtime,
            (handler, encoder_ptr, output_ptr),
            move || unsafe {
                obs_encoder_set_audio(encoder_ptr, handler);
                obs_output_set_audio_encoder(output_ptr, encoder_ptr, mixer_idx);
            }
        )?;

        let x = Arc::new(audio_enc);
        self.audio_encoders
            .write()
            .map_err(|e| ObsError::LockError(e.to_string()))?
            .push(x.clone());
        Ok(x)
    }

    /// Attaches an existing audio encoder to this output at the specified mixer index.
    ///
    /// # Arguments
    /// * `encoder` - The audio encoder to attach
    /// * `mixer_idx` - The mixer index to use (typically 0 for primary audio)
    ///
    /// # Returns
    /// A Result indicating success or an error
    pub fn set_audio_encoder(
        &mut self,
        encoder: ObsAudioEncoder,
        mixer_idx: usize,
    ) -> Result<(), ObsError> {
        if encoder.encoder.0.is_null() {
            return Err(ObsError::NullPointer);
        }

        let encoder_ptr = encoder.encoder.clone();
        let output_ptr = self.output.clone();
        run_with_obs!(self.runtime, (output_ptr, encoder_ptr), move || unsafe {
            obs_output_set_audio_encoder(output_ptr, encoder_ptr, mixer_idx)
        })?;

        if !self
            .audio_encoders
            .read()
            .map_err(|e| ObsError::LockError(e.to_string()))?
            .iter()
            .any(|x| x.encoder.0 == encoder.encoder.0)
        {
            let tmp = Arc::new(encoder);
            self.audio_encoders
                .write()
                .map_err(|e| ObsError::LockError(e.to_string()))?
                .push(tmp.clone());
        }

        Ok(())
    }

    /// Starts the output.
    ///
    /// This begins the encoding and streaming/recording process.
    ///
    /// # Returns
    /// A Result indicating success or an error (e.g., if the output is already active)
    pub fn start(&self) -> Result<(), ObsError> {
        let output_ptr = self.output.clone();
        let output_active = run_with_obs!(self.runtime, (output_ptr), move || unsafe {
            obs_output_active(output_ptr)
        })?;

        if !output_active {
            let res = run_with_obs!(self.runtime, (output_ptr), move || unsafe {
                obs_output_start(output_ptr)
            })?;

            if res {
                return Ok(());
            }

            let err = run_with_obs!(self.runtime, (output_ptr), move || unsafe {
                Sendable(obs_output_get_last_error(output_ptr))
            })?;

            let c_str = unsafe { CStr::from_ptr(err.0) };
            let err_str = c_str.to_str().ok().map(|x| x.to_string());

            return Err(ObsError::OutputStartFailure(err_str));
        }

        Err(ObsError::OutputAlreadyActive)
    }

    /// Pause or resume the output.
    ///
    /// # Arguments
    ///
    /// * `pause` - `true` to pause the output, `false` to resume the output.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The output was paused or resumed successfully.
    /// * `Err(ObsError::OutputPauseFailure(Some(String)))` - The output failed to pause or resume.
    pub fn pause(&self, pause: bool) -> Result<(), ObsError> {
        let output_ptr = self.output.clone();
        let output_active = run_with_obs!(self.runtime, (output_ptr), move || unsafe {
            obs_output_active(output_ptr)
        })?;

        if output_active {
            let res = run_with_obs!(self.runtime, (output_ptr), move || unsafe {
                obs_output_pause(output_ptr, pause)
            })?;

            if res {
                Ok(())
            } else {
                let err = run_with_obs!(self.runtime, (output_ptr), move || unsafe {
                    Sendable(obs_output_get_last_error(output_ptr))
                })?;

                let c_str = unsafe { CStr::from_ptr(err.0) };
                let err_str = c_str.to_str().ok().map(|x| x.to_string());

                Err(ObsError::OutputPauseFailure(err_str))
            }
        } else {
            Err(ObsError::OutputPauseFailure(Some(
                "Output is not active.".to_string(),
            )))
        }
    }

    /// Stops the output.
    ///
    /// This ends the encoding and streaming/recording process.
    /// The method waits for a stop signal and returns the result.
    ///
    /// # Returns
    /// A Result indicating success or an error with details about why stopping failed
    pub fn stop(&mut self) -> Result<(), ObsError> {
        let output_ptr = self.output.clone();
        let output_active = run_with_obs!(self.runtime, (output_ptr), move || unsafe {
            obs_output_active(output_ptr)
        })?;

        if output_active {
            let mut rx = self.signal_manager.on_stop()?;
            run_with_obs!(self.runtime, (output_ptr), move || unsafe {
                obs_output_stop(output_ptr)
            })?;

            let signal = rx.blocking_recv().map_err(|_| ObsError::NoSenderError)?;
            log::debug!("Signal: {:?}", signal);
            if signal == ObsOutputStopSignal::Success {
                return Ok(());
            }

            return Err(ObsError::OutputStopFailure(Some(signal.to_string())));
        }

        Err(ObsError::OutputStopFailure(Some(
            "Output is not active.".to_string(),
        )))
    }

    pub fn as_ptr(&self) -> Sendable<*mut obs_output> {
        self.output.clone()
    }
}

impl_signal_manager!(|ptr| unsafe { libobs::obs_output_get_signal_handler(ptr) }, ObsOutputSignals for ObsOutputRef<*mut libobs::obs_output>, [
    "start": {},
    "stop": {code: crate::enums::ObsOutputStopSignal},
    "pause": {},
    "unpause": {},
    "starting": {},
    "stopping": {},
    "activate": {},
    "deactivate": {},
    "reconnect": {},
    "reconnect_success": {},
]);
