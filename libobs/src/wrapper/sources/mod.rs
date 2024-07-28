//! Use the `libobs-source` crate to create sources like `window_capture` for obs


use super::{ObsData, ObsError, ObsOutput, ObsSource, ObsString, SourceInfo};

pub trait StringEnum {
    fn to_str(&self) -> &str;
}

//TODO Use generics to make the build function return a trait rather than a struct
/// Trait for building OBS sources.
pub trait ObsSourceBuilder {
    fn new(name: impl Into<ObsString>) -> Self;

    /// Returns the name of the source.
    fn get_name(&self) -> ObsString;

    /// Adds the obs source to the output on the given channel
    fn add_to_output<'a>(
        mut self,
        output: &'a mut ObsOutput,
        channel: u32,
    ) -> Result<&'a mut ObsSource, ObsError>
    where
        Self: Sized,
    {
        let settings = self.get_settings_mut().take();
        let hotkeys = self.get_hotkeys_mut().take();

        let source = SourceInfo::new(Self::get_id(), self.get_name(), settings, hotkeys);
        output.source(source, channel)
    }

    fn get_settings(&self) -> &Option<ObsData>;
    fn get_settings_mut(&mut self) -> &mut Option<ObsData>;

    fn get_hotkeys(&self) -> &Option<ObsData>;
    fn get_hotkeys_mut(&mut self) -> &mut Option<ObsData>;

    fn get_or_create_settings(&mut self) -> &mut ObsData {
        self.get_settings_mut().get_or_insert_with(ObsData::new)
    }

    /// Returns the ID of the source.
    fn get_id() -> ObsString;
}