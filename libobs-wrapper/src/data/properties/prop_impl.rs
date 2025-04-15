use crate::{data::output::ObsOutputRef, sources::ObsSourceRef, utils::ObsString};

use super::{ObsPropertyObject, ObsPropertyObjectPrivate};

impl ObsPropertyObject for ObsSourceRef {}
impl ObsPropertyObjectPrivate for ObsSourceRef {
    fn get_properties_raw(&self) -> *mut libobs::obs_properties_t {
        unsafe { libobs::obs_source_properties(self.source.0) }
    }

    fn get_properties_by_id_raw(id: ObsString) -> *mut libobs::obs_properties_t {
        unsafe { libobs::obs_get_source_properties(id.as_ptr()) }
    }
}

impl ObsPropertyObject for ObsOutputRef {}
impl ObsPropertyObjectPrivate for ObsOutputRef {
    fn get_properties_raw(&self) -> *mut libobs::obs_properties_t {
        unsafe { libobs::obs_output_properties(self.output.0) }
    }

    fn get_properties_by_id_raw(id: ObsString) -> *mut libobs::obs_properties_t {
        unsafe { libobs::obs_get_output_properties(id.as_ptr()) }
    }
}
