use std::{ffi::c_char, mem::MaybeUninit, path::{Path, PathBuf}};

use libobs::{calldata_get_string, calldata_t, obs_output_get_proc_handler, proc_handler_call};

use crate::utils::{ObsError, ObsString};

use super::ObsOutputRef;

pub trait ReplayBufferOutput {
    fn save_buffer(&self) -> Result<Box<Path>, ObsError>;
}

impl ReplayBufferOutput for ObsOutputRef {
    fn save_buffer(&self) -> Result<Box<Path>, ObsError> {
        let ph = unsafe { obs_output_get_proc_handler(self.output.0) };
        if ph.is_null() {
            return Err(ObsError::OutputSaveBufferFailure(
                "Failed to get proc handler.".to_string(),
            ));
        }

        let name = ObsString::new("save");
        let call_success = unsafe {
            let mut calldata = MaybeUninit::<calldata_t>::zeroed();
            proc_handler_call(ph, name.as_ptr(), calldata.as_mut_ptr())
        };

        if !call_success {
            return Err(ObsError::OutputSaveBufferFailure(
                "Failed to call proc handler.".to_string(),
            ));
        }

        let func_get = ObsString::new("get_last_replay");
        let last_replay = unsafe {
            let mut calldata = MaybeUninit::<calldata_t>::zeroed();
            let success = proc_handler_call(ph, func_get.as_ptr(), calldata.as_mut_ptr());

            if !success {
                return Err(ObsError::OutputSaveBufferFailure(
                    "Failed to call get_last_replay.".to_string(),
                ));
            }

            calldata.assume_init()
        };

        let path_get = ObsString::new("path");
        let path = unsafe {
            let mut s = MaybeUninit::<*const c_char>::uninit();

            let res = calldata_get_string(&last_replay, path_get.as_ptr(), s.as_mut_ptr());
            if !res {
                return Err(ObsError::OutputSaveBufferFailure(
                    "Failed to get path from last replay.".to_string(),
                ));
            }

            let s: *const c_char = s.assume_init();
            let path = std::ffi::CStr::from_ptr(s).to_str().unwrap();

            PathBuf::from(path)
        };

        Ok(path.into_boxed_path())
    }
}
