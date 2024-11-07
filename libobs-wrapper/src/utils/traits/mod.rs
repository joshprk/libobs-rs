use crate::data::{ObsData, ObsObjectUpdater};

pub trait ObsUpdatable {
    fn update<T: ObsObjectUpdater<Self>>(&mut self) -> T
    where
        Self: Sized,
    {
        T::create_update(self)
    }

    fn update_raw(&mut self, data: ObsData);
}
