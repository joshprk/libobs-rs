use crate::{data::{ObsData, ObsObjectUpdater}, runtime::ObsRuntime};

use super::ObsError;

#[async_trait::async_trait]
pub trait ObsUpdatable {
    /// Updates the object with the current settings.
    /// For examples please take a look at the [Github repository](https://github.com/joshprk/libobs-rs/blob/main/examples).
    async fn create_updater<'a, T: ObsObjectUpdater<'a, ToUpdate = Self> + Send + Sync>(&'a mut self) -> Result<T, ObsError>
    where
        Self: Sized + Send + Sync,
    {
        let runtime = self.runtime();
        T::create_update(runtime, self).await
    }

    fn runtime(&self) -> ObsRuntime;

    // We don't really need a mut here, but we do it anyway to give the dev a *feeling* of changing something
    async fn update_raw(&mut self, data: ObsData) -> Result<(), ObsError>;
    async fn reset_and_update_raw(&mut self, data: ObsData) -> Result<(), ObsError>;
}
