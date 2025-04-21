use std::{
    ffi::c_char,
    mem::MaybeUninit,
    path::{Path, PathBuf},
};

use libobs::{calldata_get_string, calldata_t, obs_output_get_proc_handler, proc_handler_call};

use crate::{
    run_with_obs,
    utils::{ObsError, ObsString},
};

use super::ObsOutputRef;

#[async_trait::async_trait(?Send)]
pub trait ReplayBufferOutput {
    async fn save_buffer(&self) -> Result<Box<Path>, ObsError>;
}

#[async_trait::async_trait(?Send)]
impl ReplayBufferOutput for ObsOutputRef {
    async fn save_buffer(&self) -> Result<Box<Path>, ObsError> {
        let output_ptr = self.output.0;

        let path = run_with_obs!(self.runtime, (output_ptr), move || {
            let ph = obs_output_get_proc_handler(output_ptr);
            if ph.is_null() {
                return Err(ObsError::OutputSaveBufferFailure(
                    "Failed to get proc handler.".to_string(),
                ));
            }

            let name = ObsString::new("save");
            let call_success = {
                let mut calldata = MaybeUninit::<calldata_t>::zeroed();
                proc_handler_call(ph, name.as_ptr(), calldata.as_mut_ptr())
            };

            if !call_success {
                return Err(ObsError::OutputSaveBufferFailure(
                    "Failed to call proc handler.".to_string(),
                ));
            }

            let func_get = ObsString::new("get_last_replay");
            let last_replay = {
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

            let mut s = MaybeUninit::<*const c_char>::uninit();

            let res = calldata_get_string(&last_replay, path_get.as_ptr(), s.as_mut_ptr());
            if !res {
                return Err(ObsError::OutputSaveBufferFailure(
                    "Failed to get path from last replay.".to_string(),
                ));
            }

            let s: *const c_char = s.assume_init();
            let path = std::ffi::CStr::from_ptr(s).to_str().unwrap();

            Ok(PathBuf::from(path))
        })??;

        Ok(path.into_boxed_path())
    }
}
