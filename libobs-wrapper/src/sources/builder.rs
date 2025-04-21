use crate::{
    data::ObsObjectBuilder,
    scenes::ObsSceneRef,
    utils::ObsError,
};

use super::ObsSourceRef;

pub const UPDATE_SOURCE_NAME: &'static str =
    "OBS_INTERNAL_UPDATE (if you see this, you've build a source wrong)";

#[async_trait::async_trait(?Send)]
pub trait ObsSourceBuilder: ObsObjectBuilder {
    async fn add_to_scene<'a>(self, scene: &'a mut ObsSceneRef) -> Result<ObsSourceRef, ObsError>
    where
        Self: Sized,
    {
        let s = self.build().await?;
        scene.add_source(s).await
    }
}