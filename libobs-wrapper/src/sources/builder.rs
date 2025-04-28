use crate::{
    data::ObsObjectBuilder,
    scenes::ObsSceneRef,
    utils::ObsError,
};

use super::ObsSourceRef;

#[cfg_attr(not(feature="blocking"), async_trait::async_trait)]
pub trait ObsSourceBuilder: ObsObjectBuilder {
    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn add_to_scene<'a>(self, scene: &'a mut ObsSceneRef) -> Result<ObsSourceRef, ObsError>
    where
        Self: Sized,
    {
        let s = self.build().await?;
        scene.add_source(s).await
    }
}