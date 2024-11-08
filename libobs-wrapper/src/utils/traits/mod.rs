use crate::data::{ObsData, ObsObjectUpdater};

pub trait ObsUpdatable {
    fn update<'a, T: ObsObjectUpdater<'a, ToUpdate = Self>>(&'a mut self) -> T
    where
        Self: Sized,
    {
        T::create_update(self)
    }

    // We don't really need a mut here, but we do it anyway to give the dev a *feeling* of changing something
    fn update_raw(&mut self, data: ObsData);
    fn reset_and_update_raw(&mut self, data: ObsData);
}
