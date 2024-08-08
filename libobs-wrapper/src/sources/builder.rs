use crate::{data::ObsObjectBuilder, scenes::ObsScene, utils::ObsError};

use super::ObsSource;

pub trait ObsSourceBuilder : ObsObjectBuilder {
    fn add_to_scene<'a>(
        self,
        scene: &'a mut ObsScene
    ) -> Result<&'a mut ObsSource, ObsError>
    where
        Self: Sized,
    {
        scene.add_source(self.build())
    }
}