use std::sync::Arc;

use getters0::Getters;
use libobs::{obs_scene_t, obs_set_output_source, obs_source_t};
use tokio::sync::RwLock;

use crate::{
    impl_obs_drop, run_with_obs, runtime::ObsRuntime, sources::ObsSourceRef, unsafe_send::Sendable, utils::{ObsError, ObsString, SourceInfo}
};

#[derive(Debug)]
struct _SceneDropGuard {
    scene: Sendable<*mut obs_scene_t>,
    runtime: ObsRuntime,
}

impl_obs_drop!(_SceneDropGuard, (scene), move || unsafe {
    libobs::obs_scene_release(scene);
});

#[derive(Debug, Clone, Getters)]
#[skip_new]
pub struct ObsSceneRef {
    #[skip_getter]
    scene: Arc<Sendable<*mut obs_scene_t>>,
    name: ObsString,
    #[get_mut]
    pub(crate) sources: Arc<RwLock<Vec<ObsSourceRef>>>,
    #[skip_getter]
    pub(crate) active_scene: Arc<RwLock<Option<Sendable<*mut obs_scene_t>>>>,

    #[skip_getter]
    _guard: Arc<_SceneDropGuard>,

    #[skip_getter]
    runtime: ObsRuntime,
}

impl ObsSceneRef {
    pub(crate) async fn new(
        name: ObsString,
        active_scene: Arc<RwLock<Option<Sendable<*mut obs_scene_t>>>>,
        runtime: ObsRuntime,
    ) -> Result<Self, ObsError> {
        let name_ptr = name.as_ptr();
        let scene = run_with_obs!(runtime, (name_ptr), move || unsafe {
            Sendable(libobs::obs_scene_create(name_ptr))
        })?;

        Ok(Self {
            name,
            scene: Arc::new(scene.clone()),
            sources: Arc::new(RwLock::new(vec![])),
            active_scene: active_scene.clone(),
            _guard: Arc::new(_SceneDropGuard {
                scene,
                runtime: runtime.clone(),
            }),
            runtime,
        })
    }

    pub async fn add_and_set(&self, channel: u32) -> Result<(), ObsError> {
        let mut s = self.active_scene.write().await;
        *s = Some(self.as_ptr());

        let scene_source_ptr = self.get_scene_source_ptr().await?;
        run_with_obs!(self.runtime, (scene_source_ptr), move || unsafe {
            obs_set_output_source(channel, scene_source_ptr);
        })
    }

    pub async fn get_scene_source_ptr(&self) -> Result<Sendable<*mut obs_source_t>, ObsError> {
        let scene_ptr = self.scene.clone();
        run_with_obs!(self.runtime, (scene_ptr), move || unsafe {
            Sendable(libobs::obs_scene_get_source(scene_ptr))
        })
    }

    pub async fn add_source(&mut self, info: SourceInfo) -> Result<ObsSourceRef, ObsError> {
        let mut source = ObsSourceRef::new(
            info.id,
            info.name,
            info.settings,
            info.hotkey_data,
            self.runtime.clone(),
        )
        .await?;

        let scene_ptr = self.scene.clone();
        let source_ptr = source.source.clone();

        let ptr = run_with_obs!(self.runtime, (scene_ptr, source_ptr), move || unsafe {
            Sendable(libobs::obs_scene_add(scene_ptr, source_ptr))
        })?;

        if ptr.0.is_null() {
            return Err(ObsError::NullPointer);
        }

        source.scene_item = Some(ptr.clone());
        self.sources.write().await.push(source.clone());
        Ok(source)
    }

    pub async fn get_source_by_index(&self, index: usize) -> Option<ObsSourceRef> {
        self.sources.read().await.get(index).map(|x| x.clone())
    }

    pub async fn get_source_mut(&self, name: &str) -> Option<ObsSourceRef> {
        self.sources
            .read()
            .await
            .iter()
            .find(|x| x.name() == name)
            .map(|x| x.clone())
    }

    pub async fn remove_source(&mut self, source: &ObsSourceRef) -> Result<(), ObsError> {
        let scene_item_ptr = source.scene_item.clone();
        if scene_item_ptr.is_none() {
            return Err(ObsError::SourceNotFound);
        }

        let scene_item_ptr = scene_item_ptr.unwrap();
        run_with_obs!(self.runtime, (scene_item_ptr), move || unsafe {
            // Remove the scene item
            libobs::obs_sceneitem_remove(scene_item_ptr);
            // Release the scene item reference
            libobs::obs_sceneitem_release(scene_item_ptr);
        })?;

        Ok(())
    }

    pub fn as_ptr(&self) -> Sendable<*mut obs_scene_t> {
        Sendable(self.scene.0)
    }
}
