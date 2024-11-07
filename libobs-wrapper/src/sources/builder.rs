use crate::{
    data::ObsObjectBuilder,
    scenes::ObsScene,
    utils::ObsError,
};

use super::ObsSource;

pub const UPDATE_SOURCE_NAME: &'static str =
    "OBS_INTERNAL_UPDATE (if you see this, you've build a source wrong)";

pub trait ObsSourceExt: ObsObjectBuilder {
    fn add_to_scene<'a>(self, scene: &'a mut ObsScene) -> Result<&'a mut ObsSource, ObsError>
    where
        Self: Sized,
    {
        scene.add_source(self.build())
    }
}