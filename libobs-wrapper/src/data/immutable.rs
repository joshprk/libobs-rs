use libobs::obs_data_t;

use crate::{impl_obs_drop, run_with_obs, runtime::ObsRuntime, unsafe_send::Sendable, utils::ObsError};

use super::ObsData;

#[derive(Debug)]
/// Immutable wrapper around obs_data_t to be prevent modification and to be used in creation of other objects.
/// This should not be updated directly using the pointer, but instead through the corresponding update methods on the holder of this data.
pub struct ImmutableObsData {
    ptr: Sendable<*mut obs_data_t>,
    runtime: ObsRuntime
}

impl ImmutableObsData {
    pub async fn new(runtime: &ObsRuntime) -> Result<Self, ObsError> {
        let ptr = run_with_obs!(runtime, move || unsafe { libobs::obs_data_create() })?;

        Ok(ImmutableObsData {
            ptr: Sendable(ptr),
            runtime: runtime.clone()
        })
    }

    pub async fn from_raw(data: *mut obs_data_t, runtime: ObsRuntime) -> Self {
        ImmutableObsData {
            ptr: Sendable(data),
            runtime
        }
    }

    pub fn as_ptr(&self) -> *mut obs_data_t {
        self.ptr.0
    }
}

impl From<ObsData> for ImmutableObsData {
    fn from(mut data: ObsData) -> Self {
        // Set to null pointer to prevent double free
        let ptr = data.obs_data.0;

        data.obs_data.0 = std::ptr::null_mut();
        ImmutableObsData {
            ptr: Sendable(ptr),
            runtime: data.runtime.clone()
        }
    }
}

impl_obs_drop!(ImmutableObsData, (ptr), move || unsafe {
    libobs::obs_data_release(ptr.0)
});