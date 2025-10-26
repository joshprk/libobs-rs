use crate::{data::ObsObjectBuilder, scenes::ObsSceneRef, utils::ObsError};

use super::ObsSourceRef;

pub trait ObsSourceBuilder: ObsObjectBuilder {
    fn add_to_scene(self, scene: &mut ObsSceneRef) -> Result<ObsSourceRef, ObsError>
    where
        Self: Sized,
    {
        let s = self.build()?;
        scene.add_source(s)
    }
}
