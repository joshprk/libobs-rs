use crate::data::{ObsData, ObsObjectUpdater};

use super::ObsError;

#[async_trait::async_trait]
pub trait ObsUpdatable {
    /// Updates the object with the current settings.
    /// Note that this example requires the `libobs-sources` crate.
    /// ## Example usage
    /// ```rust
    /// use libobs_wrapper::data::ObsObjectUpdater;
    /// let source = WindowCaptureSourceBuilder::new("test_capture")
    ///     .set_window(&window)
    ///     .add_to_scene(scene)
    ///     .unwrap();
    ///
    /// // Do other stuff with source
    ///
    /// // Update the source with the corresponding updater like so
    /// source.create_updater::<WindowCaptureSourceUpdater>();
    /// ```
    fn create_updater<'a, T: ObsObjectUpdater<'a, ToUpdate = Self>>(&'a mut self) -> T
    where
        Self: Sized,
    {
        T::create_update(self)
    }

    // We don't really need a mut here, but we do it anyway to give the dev a *feeling* of changing something
    async fn update_raw(&mut self, data: ObsData) -> Result<(), ObsError>;
    async fn reset_and_update_raw(&mut self, data: ObsData) -> Result<(), ObsError>;
}
