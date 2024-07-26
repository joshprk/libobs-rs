use super::{obstypes::ObsSource, ObsData, ObsError, ObsOutput, ObsString, SourceInfo};

#[cfg(target_family = "windows")]
mod window_capture;

#[cfg(target_family = "windows")]
pub use window_capture::WindowCaptureSourceBuilder;

#[allow(private_bounds)]
pub(super) trait ObsSourceBuilderPrivate {
    fn take_settings(&mut self) -> Option<ObsData>;
    fn take_hotkeys(&mut self) -> Option<ObsData>;
}

//TODO Use generics to make the build function return a trait rather than a struct
/// Trait for building OBS sources.
pub trait ObsSourceBuilder: ObsSourceBuilderPrivate {
    fn new(name: impl Into<ObsString>) -> Self;

    /// Returns the name of the source.
    fn get_name(&self) -> ObsString;

    /// Adds the obs source to the output on the given channel
    fn add<'a>(
        mut self,
        output: &'a mut ObsOutput,
        channel: u32,
    ) -> Result<&'a mut ObsSource, ObsError>
    where
        Self: Sized,
    {
        let settings = self.take_settings();
        let hotkeys = self.take_hotkeys();

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


/// Implements boilerplate code for the `ObsSourceBuilder` trait.
/// Note: The struct must contain the following fields:
/// - settings: `Option<ObsData>`
/// - hotkeys: `Option<ObsData>`
/// - name: `ObsString`
macro_rules! impl_obs_source_builder {
    () => {
        compile_error!("First argument must be a struct. Look at the documentation for required fields");
    };
    ($builder:ident) => {
        compile_error!("Second argument must be a valid obs source id");
    };
    ($builder:ident, $id: expr) => {
        impl ObsSourceBuilder for $builder {
            fn new(name: impl Into<ObsString>) -> Self {
                Self {
                    settings: None,
                    hotkeys: None,
                    name: name.into(),
                }
            }

            fn get_settings(&self) -> &Option<ObsData> {
                &self.settings
            }

            fn get_settings_mut(&mut self) -> &mut Option<ObsData> {
                &mut self.settings
            }

            fn get_hotkeys(&self) -> &Option<ObsData> {
                &self.hotkeys
            }

            fn get_hotkeys_mut(&mut self) -> &mut Option<ObsData> {
                &mut self.hotkeys
            }

            fn get_name(&self) -> ObsString {
                self.name.clone()
            }

            fn get_id() -> ObsString {
                $id.into()
            }
        }

        impl ObsSourceBuilderPrivate for $builder {
            fn take_settings(&mut self) -> Option<ObsData> {
                self.settings.take()
            }

            fn take_hotkeys(&mut self) -> Option<ObsData> {
                self.hotkeys.take()
            }
        }
    };
}

pub(super) use impl_obs_source_builder;