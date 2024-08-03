use std::ffi::CString;
use std::{borrow::Borrow, ffi::CStr, ptr};

use libobs::{
    audio_output, calldata_get_data, calldata_t, obs_encoder_set_audio, obs_encoder_set_video,
    obs_output, obs_output_active, obs_output_create, obs_output_get_last_error,
    obs_output_get_name, obs_output_get_signal_handler, obs_output_release,
    obs_output_set_audio_encoder, obs_output_set_video_encoder, obs_output_start, obs_output_stop,
    obs_set_output_source, signal_handler_connect, video_output,
};

use crate::enums::ObsOutputSignal;
use crate::signals::{rec_output_signal, OUTPUT_SIGNALS};
use crate::utils::{AudioEncoderInfo, SourceInfo, VideoEncoderInfo};

use crate::{
    encoders::{audio::ObsAudioEncoder, video::ObsVideoEncoder},
    sources::ObsSource,
    utils::{ObsError, ObsString},
};

use super::ObsData;

#[derive(Debug)]
pub struct ObsOutput {
    pub(crate) output: *mut obs_output,
    pub(crate) id: ObsString,
    pub(crate) name: ObsString,
    pub(crate) settings: Option<ObsData>,
    pub(crate) hotkey_data: Option<ObsData>,
    pub(crate) video_encoders: Vec<ObsVideoEncoder>,
    pub(crate) audio_encoders: Vec<ObsAudioEncoder>,
    pub(crate) sources: Vec<ObsSource>,
}

impl ObsOutput {
    pub fn new(
        id: impl Into<ObsString>,
        name: impl Into<ObsString>,
        settings: Option<ObsData>,
        hotkey_data: Option<ObsData>,
    ) -> Result<Self, ObsError> {
        // Likely unnecessary as this is private and only
        // constructible with ObsContext member functions.
        /*if let Ok(thread_id) = wrapper::OBS_THREAD_ID.lock() {
            if *thread_id != Some(thread::current().id()) {
                return Err(ObsError::CreateThreadError)
            }
        } else {
            panic!();
        }*/

        let id = id.into();
        let name = name.into();

        let settings_ptr = match settings.borrow() {
            Some(x) => x.as_ptr(),
            None => ptr::null_mut(),
        };

        let hotkey_data_ptr = match hotkey_data.borrow() {
            Some(x) => x.as_ptr(),
            None => ptr::null_mut(),
        };

        let output =
            unsafe { obs_output_create(id.as_ptr(), name.as_ptr(), settings_ptr, hotkey_data_ptr) };

        if output == ptr::null_mut() {
            return Err(ObsError::NullPointer);
        }

        //TODO connect signal handler
        let handler = unsafe { obs_output_get_signal_handler(output) };
        unsafe {
            let signal = ObsString::new("stop");
            signal_handler_connect(
                handler,
                signal.as_ptr(),
                Some(signal_handler),
                ptr::null_mut(),
            )
        };

        Ok(Self {
            output,
            id,
            name,
            settings,
            hotkey_data,
            video_encoders: vec![],
            audio_encoders: vec![],
            sources: vec![],
        })
    }

    pub fn get_video_encoders(&mut self) -> &mut Vec<ObsVideoEncoder> {
        &mut self.video_encoders
    }

    pub fn video_encoder(
        &mut self,
        info: VideoEncoderInfo,
        handler: *mut video_output,
    ) -> Result<&mut ObsVideoEncoder, ObsError> {
        let video_enc = ObsVideoEncoder::new(info.id, info.name, info.settings, info.hotkey_data);

        return match video_enc {
            Ok(x) => {
                unsafe { obs_encoder_set_video(x.encoder, handler) }
                unsafe { obs_output_set_video_encoder(self.output, x.encoder) }
                self.video_encoders.push(x);

                Ok(self.video_encoders.last_mut().unwrap())
            }
            Err(x) => Err(x),
        };
    }

    pub fn audio_encoder(
        &mut self,
        info: AudioEncoderInfo,
        mixer_idx: usize,
        handler: *mut audio_output,
    ) -> Result<&mut ObsAudioEncoder, ObsError> {
        let audio_enc = ObsAudioEncoder::new(
            info.id,
            info.name,
            info.settings,
            mixer_idx,
            info.hotkey_data,
        );

        return match audio_enc {
            Ok(x) => {
                unsafe { obs_encoder_set_audio(x.encoder, handler) }
                unsafe { obs_output_set_audio_encoder(self.output, x.encoder, mixer_idx) }
                self.audio_encoders.push(x);

                Ok(self.audio_encoders.last_mut().unwrap())
            }
            Err(x) => Err(x),
        };
    }

    pub fn source(&mut self, info: SourceInfo, channel: u32) -> Result<&mut ObsSource, ObsError> {
        let source = ObsSource::new(info.id, info.name, info.settings, info.hotkey_data);

        return match source {
            Ok(x) => {
                unsafe { obs_set_output_source(channel, x.source) }
                self.sources.push(x);
                Ok(self.sources.last_mut().unwrap())
            }
            Err(x) => Err(x),
        };
    }

    pub fn start(&mut self) -> Result<(), ObsError> {
        if unsafe { !obs_output_active(self.output) } {
            let res = unsafe { obs_output_start(self.output) };
            if res {
                return Ok(());
            }

            let err = unsafe { obs_output_get_last_error(self.output) };
            let c_str = unsafe { CStr::from_ptr(err) };
            let err_str = c_str.to_str().ok().map(|x| x.to_string());

            return Err(ObsError::OutputStartFailure(err_str));
        }

        Err(ObsError::OutputAlreadyActive)
    }

    pub fn stop(&mut self) -> Result<(), ObsError> {
        if unsafe { obs_output_active(self.output) } {
            unsafe { obs_output_stop(self.output) }

            let signal = rec_output_signal(&self)
                .map_err(|e| ObsError::OutputStopFailure(Some(e.to_string())))?;

            log::debug!("Signal: {:?}", signal);
            if signal == ObsOutputSignal::Success {
                return Ok(());
            }

            return Err(ObsError::OutputStopFailure(Some(signal.to_string())));
        }

        return Err(ObsError::OutputStopFailure(Some(
            "Output is not active.".to_string(),
        )));
    }

    // Getters
    pub fn name(&self) -> &ObsString {
        &self.name
    }

    pub fn id(&self) -> &ObsString {
        &self.id
    }

    pub fn settings(&self) -> &Option<ObsData> {
        &self.settings
    }

    pub fn hotkey_data(&self) -> &Option<ObsData> {
        &self.hotkey_data
    }

    pub fn video_encoders(&self) -> &Vec<ObsVideoEncoder> {
        &self.video_encoders
    }

    pub fn audio_encoders(&self) -> &Vec<ObsAudioEncoder> {
        &self.audio_encoders
    }

    pub fn sources(&self) -> &Vec<ObsSource> {
        &self.sources
    }
}

impl Drop for ObsOutput {
    fn drop(&mut self) {
        unsafe { obs_output_release(self.output) }
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

        let signal = ObsOutputSignal::try_from(code as i32);
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
