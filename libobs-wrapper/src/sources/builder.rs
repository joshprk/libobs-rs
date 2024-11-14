use crate::{
    data::ObsObjectBuilder,
    scenes::ObsSceneRef,
    utils::ObsError,
};

use super::ObsSourceRef;

pub const UPDATE_SOURCE_NAME: &'static str =
    "OBS_INTERNAL_UPDATE (if you see this, you've build a source wrong)";

pub trait ObsSourceBuilder: ObsObjectBuilder {
    fn add_to_scene<'a>(self, scene: &'a mut ObsSceneRef) -> Result<ObsSourceRef, ObsError>
    where
        Self: Sized,
    {
        scene.add_source(self.build())
    }
}