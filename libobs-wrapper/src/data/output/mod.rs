use std::ffi::CString;
use std::sync::Arc;
use std::{ffi::CStr, ptr};

use anyhow::bail;
use getters0::Getters;
use libobs::{
    audio_output, calldata_get_data, calldata_t, obs_encoder_set_audio, obs_encoder_set_video,
    obs_output, obs_output_active, obs_output_create, obs_output_get_last_error,
    obs_output_get_name, obs_output_get_signal_handler, obs_output_release,
    obs_output_set_audio_encoder, obs_output_set_video_encoder, obs_output_start, obs_output_stop,
    obs_output_update, signal_handler_connect, signal_handler_disconnect, video_output,
};
use tokio::sync::RwLock;

use crate::enums::ObsOutputStopSignal;
use crate::runtime::ObsRuntime;
use crate::signals::{rec_output_signal, OUTPUT_SIGNALS};
use crate::unsafe_send::Sendable;
use crate::utils::{AudioEncoderInfo, OutputInfo, VideoEncoderInfo};
use crate::{impl_obs_drop, run_with_obs};

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
    let handler = obs_output_get_signal_handler(output);
    let signal = ObsString::new("stop");
    signal_handler_disconnect(
        handler,
        signal.as_ptr().0,
        Some(signal_handler),
        ptr::null_mut(),
    );

    obs_output_release(output);
});

#[derive(Debug, Getters, Clone)]
#[skip_new]
pub struct ObsOutputRef {
    pub(crate) settings: Arc<RwLock<Option<ObsData>>>,
    pub(crate) hotkey_data: Arc<RwLock<Option<ObsData>>>,

    #[get_mut]
    pub(crate) video_encoders: Arc<RwLock<Vec<Arc<ObsVideoEncoder>>>>,

    #[get_mut]
    pub(crate) audio_encoders: Arc<RwLock<Vec<Arc<ObsAudioEncoder>>>>,

    #[skip_getter]
    pub(crate) output: Sendable<*mut obs_output>,
    pub(crate) id: ObsString,
    pub(crate) name: ObsString,

    #[skip_getter]
    _drop_guard: Arc<_ObsDropGuard>,

    #[skip_getter]
    pub(crate) runtime: ObsRuntime,
}

impl ObsOutputRef {
    pub(crate) async fn new(output: OutputInfo, runtime: ObsRuntime) -> Result<Self, ObsError> {
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
                    obs_output_create(id.as_ptr().0, name.as_ptr().0, settings_ptr.0, hotkey_data_ptr.0)
                };

                if output == ptr::null_mut() {
                    bail!("Null pointer returned from obs_output_create");
                }

                let handler = unsafe { obs_output_get_signal_handler(output) };
                unsafe {
                    let signal = ObsString::new("stop");
                    signal_handler_connect(
                        handler,
                        signal.as_ptr().0,
                        Some(signal_handler),
                        ptr::null_mut(),
                    )
                };

                return Ok((Sendable(output), id, name, settings, hotkey_data));
            })
            .await
            .map_err(|e| ObsError::InvocationError(e.to_string()))?
            .map_err(|_| ObsError::NullPointer)?;

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
        })
    }

    pub async fn get_video_encoders(&self) -> Vec<Arc<ObsVideoEncoder>> {
        self.video_encoders.read().await.clone()
    }

    pub async fn video_encoder(
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
        )
        .await?;

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
        self.video_encoders.write().await.push(tmp.clone());

        Ok(tmp)
    }

    pub async fn set_video_encoder(&mut self, encoder: ObsVideoEncoder) -> Result<(), ObsError> {
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
            .await
            .iter()
            .any(|x| x.encoder.0 == encoder.as_ptr().0)
        {
            let tmp = Arc::new(encoder);

            self.video_encoders.write().await.push(tmp.clone());
        }

        Ok(())
    }

    pub async fn update_settings(&mut self, settings: ObsData) -> Result<(), ObsError> {
        let output = self.output.clone();
        let output_active = run_with_obs!(self.runtime, (output), move || unsafe {
            obs_output_active(output)
        })?;

        if !output_active {
            let settings_ptr = settings.as_ptr();

            run_with_obs!(self.runtime, (output, settings_ptr), move || unsafe {
                obs_output_update(output, settings_ptr)
            })?;

            self.settings.write().await.replace(settings);
            Ok(())
        } else {
            Err(ObsError::OutputAlreadyActive)
        }
    }

    pub async fn audio_encoder(
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
        )
        .await?;

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
        self.audio_encoders.write().await.push(x.clone());
        Ok(x)
    }

    pub async fn set_audio_encoder(
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
            .await
            .iter()
            .any(|x| x.encoder.0 == encoder.encoder.0)
        {
            let tmp = Arc::new(encoder);
            self.audio_encoders.write().await.push(tmp.clone());
        }

        Ok(())
    }

    pub async fn start(&self) -> Result<(), ObsError> {
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

    pub async fn stop(&mut self) -> Result<(), ObsError> {
        let output_ptr = self.output.clone();
        let output_active = run_with_obs!(self.runtime, (output_ptr), move || unsafe {
            obs_output_active(output_ptr)
        })?;

        if output_active {
            run_with_obs!(self.runtime, (output_ptr), move || unsafe {
                obs_output_stop(output_ptr)
            })?;

            let signal = rec_output_signal(&self).await
                .map_err(|e| ObsError::OutputStopFailure(Some(e.to_string())))?;

            log::debug!("Signal: {:?}", signal);
            if signal == ObsOutputStopSignal::Success {
                return Ok(());
            }

            return Err(ObsError::OutputStopFailure(Some(signal.to_string())));
        }

        return Err(ObsError::OutputStopFailure(Some(
            "Output is not active.".to_string(),
        )));
    }
}

extern "C" fn signal_handler(_data: *mut std::ffi::c_void, cd: *mut calldata_t) {
    unsafe {
        let mut output = ptr::null_mut();
        let output_str = CString::new("output").unwrap();
        let output_got = calldata_get_data(
            cd,
            output_str.as_ptr(),
            &mut output as *mut _ as *mut std::ffi::c_void,
            size_of::<*mut std::ffi::c_void>(),
        );
        if !output_got {
            return;
        }

        let mut code = 0i64;
        let code_str = CString::new("code").unwrap();
        let code_got = calldata_get_data(
            cd,
            code_str.as_ptr(),
            &mut code as *mut _ as *mut std::ffi::c_void,
            size_of::<i64>(),
        );

        if !code_got {
            return;
        }

        let name = obs_output_get_name(output as *mut _);
        let name_str = CStr::from_ptr(name).to_string_lossy().to_string();

        let signal = ObsOutputStopSignal::try_from(code as i32);
        if signal.is_err() {
            return;
        }

        let signal = signal.unwrap();
        let r = OUTPUT_SIGNALS.read();
        if r.is_err() {
            return;
        }

        let r = r.unwrap().0.send((name_str, signal));
        if let Err(e) = r {
            eprintln!("Couldn't send msg {:?}", e);
            return;
        }
    }
}
