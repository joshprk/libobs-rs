mod startup;
pub use startup::*;

use crate::data::ObsData;

use super::ObsString;

#[derive(Debug)]
pub struct ObjectInfo {
    pub id: ObsString,
    pub name: ObsString,
    pub settings: Option<ObsData>,
    pub hotkey_data: Option<ObsData>,
}

impl ObjectInfo {
    pub fn new(
        id: impl Into<ObsString>,
        name: impl Into<ObsString>,
        settings: Option<ObsData>,
        hotkey_data: Option<ObsData>,
    ) -> Self {
        let id = id.into();
        let name = name.into();

        Self {
            id,
            name,
            settings,
            hotkey_data,
        }
    }
}

pub type OutputInfo = ObjectInfo;
pub type SourceInfo = ObjectInfo;
pub type FilterInfo = ObjectInfo;
pub type AudioEncoderInfo = ObjectInfo;
pub type VideoEncoderInfo = ObjectInfo;
