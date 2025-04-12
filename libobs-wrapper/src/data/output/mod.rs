use std::cell::RefCell;
use std::ffi::CString;
use std::rc::Rc;
use std::{ffi::CStr, ptr};

use getters0::Getters;
use libobs::{
    audio_output, calldata_get_data, calldata_t, obs_encoder_set_audio, obs_encoder_set_video, obs_output_active, obs_output_create, obs_output_get_last_error, obs_output_get_name, obs_output_get_signal_handler, obs_output_release, obs_output_set_audio_encoder, obs_output_set_video_encoder, obs_output_start, obs_output_stop, obs_output_update, signal_handler_connect, signal_handler_disconnect, video_output
};

use crate::context::ObsContextShutdownZST;
use crate::enums::ObsOutputSignal;
use crate::signals::{rec_output_signal, OUTPUT_SIGNALS};
use crate::unsafe_send::WrappedObsOutput;
use crate::utils::{AudioEncoderInfo, OutputInfo, VideoEncoderInfo};

use crate::{
    encoders::{audio::ObsAudioEncoder, video::ObsVideoEncoder},
    utils::{ObsError, ObsString},
};

use super::ObsData;

mod replay_buffer;
pub use replay_buffer::*;

#[derive(Debug)]
struct _ObsDropGuard {
    output: WrappedObsOutput,
}

impl Drop for _ObsDropGuard {
    fn drop(&mut self) {
        unsafe {
            let handler = obs_output_get_signal_handler(self.output.0);
            let signal = ObsString::new("stop");
            signal_handler_disconnect(
                handler,
                signal.as_ptr(),
                Some(signal_handler),
                ptr::null_mut(),
            );

            obs_output_release(self.output.0);
        }
    }
}

#[derive(Debug, Getters, Clone)]
#[skip_new]
pub struct ObsOutputRef {
    pub(crate) settings: Rc<RefCell<Option<ObsData>>>,
    pub(crate) hotkey_data: Rc<RefCell<Option<ObsData>>>,

    #[get_mut]
    pub(crate) video_encoders: Rc<RefCell<Vec<Rc<ObsVideoEncoder>>>>,

    #[get_mut]
    pub(crate) audio_encoders: Rc<RefCell<Vec<Rc<ObsAudioEncoder>>>>,

    #[skip_getter]
    pub(crate) output: Rc<WrappedObsOutput>,
    pub(crate) id: ObsString,
    pub(crate) name: ObsString,

    #[skip_getter]
    _drop_guard: Rc<_ObsDropGuard>,

    #[skip_getter]
    _shutdown: Rc<ObsContextShutdownZST>
}


impl ObsOutputRef {
    pub(crate) fn new(output: OutputInfo, context: Rc<ObsContextShutdownZST>) -> Result<Self, ObsError> {
        let OutputInfo {
            id,
            name,
            settings,
            hotkey_data,
        } = output;

        let settings_ptr = match settings.as_ref() {
            Some(x) => x.as_ptr(),
            None => ptr::null_mut(),
        };

        let hotkey_data_ptr = match hotkey_data.as_ref() {
            Some(x) => x.as_ptr(),
            None => ptr::null_mut(),
        };

        let output =
            unsafe { obs_output_create(id.as_ptr(), name.as_ptr(), settings_ptr, hotkey_data_ptr) };

        if output == ptr::null_mut() {
            return Err(ObsError::NullPointer);
        }

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
            output: Rc::new(WrappedObsOutput(output)),
            id,
            name,
            settings: Rc::new(RefCell::new(settings)),
            hotkey_data: Rc::new(RefCell::new(hotkey_data)),
            video_encoders: Rc::new(RefCell::new(vec![])),
            audio_encoders: Rc::new(RefCell::new(vec![])),
            _drop_guard: Rc::new(_ObsDropGuard {
                output: WrappedObsOutput(output),
            }),
            _shutdown: context
        })
    }

    pub fn get_video_encoders(&self) -> Vec<Rc<ObsVideoEncoder>> {
        self.video_encoders.borrow().clone()
    }

    pub fn video_encoder(
        &mut self,
        info: VideoEncoderInfo,
        handler: *mut video_output,
    ) -> Result<Rc<ObsVideoEncoder>, ObsError> {
        let video_enc = ObsVideoEncoder::new(info.id, info.name, info.settings, info.hotkey_data);

        return match video_enc {
            Ok(x) => {
                unsafe { obs_encoder_set_video(x.encoder.0, handler) }
                unsafe { obs_output_set_video_encoder(self.output.0, x.encoder.0) }

                let tmp = Rc::new(x);
                self.video_encoders.borrow_mut().push(tmp.clone());

                Ok(tmp)
            }
            Err(x) => Err(x),
        };
    }

    pub fn set_video_encoder(&mut self, encoder: ObsVideoEncoder) -> Result<(), ObsError> {
        if encoder.encoder.0.is_null() {
            return Err(ObsError::NullPointer);
        }

        unsafe { obs_output_set_video_encoder(self.output.0, encoder.as_ptr()) }

        if !self.video_encoders.borrow().iter().any(|x| x.encoder.0 == encoder.as_ptr()) {
            let tmp = Rc::new(encoder);
            self.video_encoders.borrow_mut().push(tmp.clone());
        }
        
        Ok(())
    }

    pub fn update_settings(&mut self, settings: ObsData) -> Result<(), ObsError> {
        if unsafe { !obs_output_active(self.output.0) } {
            unsafe { obs_output_update(self.output.0, settings.as_ptr()) }
            self.settings.borrow_mut().replace(settings);
            Ok(())
        } else {
            Err(ObsError::OutputAlreadyActive)
        }
    }

    pub fn audio_encoder(
        &mut self,
        info: AudioEncoderInfo,
        mixer_idx: usize,
        handler: *mut audio_output,
    ) -> Result<Rc<ObsAudioEncoder>, ObsError> {
        let audio_enc = ObsAudioEncoder::new(
            info.id,
            info.name,
            info.settings,
            mixer_idx,
            info.hotkey_data,
        );

        return match audio_enc {
            Ok(x) => {
                unsafe { obs_encoder_set_audio(x.encoder.0, handler) }
                unsafe { obs_output_set_audio_encoder(self.output.0, x.encoder.0, mixer_idx) }

                let x = Rc::new(x);

                self.audio_encoders.borrow_mut().push(x.clone());
                Ok(x)
            }
            Err(x) => Err(x),
        };
    }

    pub fn set_audio_encoder(&mut self, encoder: ObsAudioEncoder, mixer_idx: usize) -> Result<(), ObsError> {
        if encoder.encoder.0.is_null() {
            return Err(ObsError::NullPointer);
        }

        unsafe { obs_output_set_audio_encoder(self.output.0, encoder.encoder.0, mixer_idx) }

        if !self.audio_encoders.borrow().iter().any(|x| x.encoder.0 == encoder.encoder.0) {
            let tmp = Rc::new(encoder);
            self.audio_encoders.borrow_mut().push(tmp.clone());
        }
        
        Ok(())
    }

    pub fn start(&self) -> Result<(), ObsError> {
        if unsafe { !obs_output_active(self.output.0) } {
            let res = unsafe { obs_output_start(self.output.0) };
            if res {
                return Ok(());
            }

            let err = unsafe { obs_output_get_last_error(self.output.0) };
            let c_str = unsafe { CStr::from_ptr(err) };
            let err_str = c_str.to_str().ok().map(|x| x.to_string());

            return Err(ObsError::OutputStartFailure(err_str));
        }

        Err(ObsError::OutputAlreadyActive)
    }

    pub fn stop(&mut self) -> Result<(), ObsError> {
        if unsafe { obs_output_active(self.output.0) } {
            unsafe { obs_output_stop(self.output.0) }

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
