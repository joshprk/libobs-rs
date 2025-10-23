//! Use the `libobs-source` crate to create sources like `window_capture` for obs

use crate::{
    data::ObsData,
    runtime::ObsRuntime,
    utils::{traits::ObsUpdatable, ObjectInfo, ObsError, ObsString},
};

use super::updater::ObsDataUpdater;

pub trait StringEnum {
    fn to_str(&self) -> &str;
}

//TODO Use generics to make the build function return a trait rather than a struct
/// Trait for building OBS sources.
pub trait ObsObjectBuilder {
    fn new<T: Into<ObsString> + Send + Sync>(
        name: T,
        runtime: ObsRuntime,
    ) -> Result<Self, ObsError>
    where
        Self: Sized;

    /// Returns the name of the source.
    fn get_name(&self) -> ObsString;

    fn build(self) -> Result<ObjectInfo, ObsError>
    where
        Self: Sized;

    fn get_settings(&self) -> &ObsData;
    fn get_settings_updater(&mut self) -> &mut ObsDataUpdater;

    fn get_hotkeys(&self) -> &ObsData;
    fn get_hotkeys_updater(&mut self) -> &mut ObsDataUpdater;

    /// Returns the ID of the source.
    fn get_id() -> ObsString;
}

pub trait ObsObjectUpdater<'a> {
    type ToUpdate: ObsUpdatable;
    fn create_update(
        runtime: ObsRuntime,
        updatable: &'a mut Self::ToUpdate,
    ) -> Result<Self, ObsError>
    where
        Self: Sized;

    fn get_settings(&self) -> &ObsData;
    fn get_settings_updater(&mut self) -> &mut ObsDataUpdater;

    fn update(self) -> Result<(), ObsError>;

    /// Returns the ID of the object
    fn get_id() -> ObsString;
}
