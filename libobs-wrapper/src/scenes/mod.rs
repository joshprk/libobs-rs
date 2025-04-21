use std::sync::Arc;

use getters0::Getters;
use libobs::{obs_scene_create, obs_scene_t, obs_set_output_source, obs_source_t};
use tokio::sync::RwLock;

use crate::{
    run_with_obs,
    runtime::ObsRuntime,
    sources::ObsSourceRef,
    unsafe_send::Sendable,
    utils::{ObsError, ObsString, SourceInfo},
};

#[derive(Debug)]
struct _SceneDropGuard {
    scene: Sendable<*mut obs_scene_t>,
    runtime: ObsRuntime,
}

impl Drop for _SceneDropGuard {
    fn drop(&mut self) {
        unsafe {
            libobs::obs_scene_release(self.scene.0);
        }
    }
}

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
        let scene = run_with_obs!(runtime, (name_ptr), move || obs_scene_create(name_ptr))?;

        Ok(Self {
            name,
            scene: Arc::new(Sendable(scene)),
            sources: Arc::new(RwLock::new(vec![])),
            active_scene: active_scene.clone(),
            _guard: Arc::new(_SceneDropGuard {
                scene: Sendable(scene),
                runtime: runtime.clone(),
            }),
            runtime,
        })
    }

    pub async fn add_and_set(&self, channel: u32) -> Result<(), ObsError> {
        let mut s = self.active_scene.write().await;
        *s = Some(Sendable(self.as_ptr()));

        let scene_source_ptr = self.get_scene_source_ptr().await?;
        run_with_obs!(self.runtime, (scene_source_ptr), move || {
            obs_set_output_source(channel, scene_source_ptr);
        })
    }

    pub async fn get_scene_source_ptr(&self) -> Result<*mut obs_source_t, ObsError> {
        let scene_ptr = self.scene.0;
        run_with_obs!(self.runtime, (scene_ptr), move || {
            libobs::obs_scene_get_source(scene_ptr)
        })
    }

    pub async fn add_source(&mut self, info: SourceInfo) -> Result<ObsSourceRef, ObsError> {
        let source = ObsSourceRef::new(
            info.id,
            info.name,
            info.settings,
            info.hotkey_data,
            self.runtime.clone(),
        )
        .await?;

        let scene_ptr = self.scene.0;
        let source_ptr = source.source.0;
        run_with_obs!(self.runtime, (scene_ptr, source_ptr), move || {
            libobs::obs_scene_add(scene_ptr, source_ptr);
        })?;

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

    pub async fn remove_source(&mut self, name: ObsString) -> Result<(), ObsError> {
        // Find the source by name
        let index = if let Some(index) = self
            .sources
            .read()
            .await
            .iter()
            .position(|x| x.name == name)
        {
            let name_ptr = name.as_ptr();
            let scene_ptr = self.scene.0;
            run_with_obs!(self.runtime, (name_ptr, scene_ptr), move || {
                // Find the scene item for this source
                let scene_item = libobs::obs_scene_find_source(scene_ptr, name_ptr);
                if !scene_item.is_null() {
                    // Remove the scene item
                    libobs::obs_sceneitem_remove(scene_item);
                    // Release the scene item reference
                    libobs::obs_sceneitem_release(scene_item);
                }
            })?;

            index
        } else {
            return Err(ObsError::SourceNotFound);
        };

        // Remove from our sources list
        self.sources.write().await.remove(index);
        Ok(())
    }

    pub fn as_ptr(&self) -> *mut obs_scene_t {
        self.scene.0
    }
}
