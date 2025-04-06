use crate::{data::output::ObsOutputRef, sources::ObsSourceRef};

use super::{ObsPropertyObject, ObsPropertyObjectPrivate};

impl ObsPropertyObject for ObsSourceRef {}
impl ObsPropertyObjectPrivate for ObsSourceRef {
    fn get_properties_raw(&self) -> *mut libobs::obs_properties_t {
        unsafe { libobs::obs_source_properties(self.source.0) }
    }
}

impl ObsPropertyObject for ObsOutputRef {}
impl ObsPropertyObjectPrivate for ObsOutputRef {
    fn get_properties_raw(&self) -> *mut libobs::obs_properties_t {
        unsafe { libobs::obs_output_properties(self.output.0) }
    }
}
