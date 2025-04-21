use libobs::obs_data_t;

use crate::unsafe_send::Sendable;

use super::ObsData;

#[derive(Debug)]
/// Immutable wrapper around obs_data_t to be prevent modification and to be used in creation of other objects.
/// This should not be updated directly using the pointer, but instead through the corresponding update methods on the holder of this data.
pub struct ImmutableObsData(Sendable<*mut obs_data_t>);

impl ImmutableObsData {
    pub fn new() -> Self {
        let ptr = unsafe { libobs::obs_data_create() };

        ImmutableObsData(Sendable(ptr))
    }

    pub fn as_ptr(&self) -> *mut obs_data_t {
        self.0 .0
    }
}

impl From<ObsData> for ImmutableObsData {
    fn from(mut data: ObsData) -> Self {
        // Set to null pointer to prevent double free
        let ptr = data.obs_data.0;

        data.obs_data.0 = std::ptr::null_mut();
        ImmutableObsData(WrappedObsData(ptr))
    }
}

impl From<*mut obs_data_t> for ImmutableObsData {
    fn from(data: *mut obs_data_t) -> Self {
        ImmutableObsData(WrappedObsData(data))
    }
}

impl Drop for ImmutableObsData {
    fn drop(&mut self) {
        unsafe { libobs::obs_data_release(self.0 .0) }
    }
}
