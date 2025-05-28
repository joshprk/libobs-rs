use std::sync::Arc;

use getters0::Getters;
use libobs::{obs_scene_t, obs_set_output_source, obs_source_t};

use crate::{
    impl_obs_drop, impl_signal_manager, run_with_obs, runtime::ObsRuntime, sources::{ObsFilterRef, ObsSourceRef}, unsafe_send::Sendable, utils::{async_sync::RwLock, ObsError, ObsString, SourceInfo}, Vec2
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

    pub(crate) signals: Arc<ObsSceneSignals>,
}

impl ObsSceneRef {
    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub(crate) async fn new(
        name: ObsString,
        active_scene: Arc<RwLock<Option<Sendable<*mut obs_scene_t>>>>,
        runtime: ObsRuntime,
    ) -> Result<Self, ObsError> {
        let name_ptr = name.as_ptr();
        let scene = run_with_obs!(runtime, (name_ptr), move || unsafe {
            Sendable(libobs::obs_scene_create(name_ptr))
        }).await?;

        let signals = Arc::new(ObsSceneSignals::new(&scene, runtime.clone()).await?);
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
            signals,
        })
    }

    #[deprecated = "Use ObsSceneRef::set_to_channel instead"]
    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn add_and_set(&self, channel: u32) -> Result<(), ObsError> {
        self.set_to_channel(channel).await
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn set_to_channel(&self, channel: u32) -> Result<(), ObsError> {
        let mut s = self.active_scene.write().await;
        *s = Some(self.as_ptr());

        let scene_source_ptr = self.get_scene_source_ptr().await?;
        run_with_obs!(self.runtime, (scene_source_ptr), move || unsafe {
            obs_set_output_source(channel, scene_source_ptr);
        }).await
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn get_scene_source_ptr(&self) -> Result<Sendable<*mut obs_source_t>, ObsError> {
        let scene_ptr = self.scene.clone();
        run_with_obs!(self.runtime, (scene_ptr), move || unsafe {
            Sendable(libobs::obs_scene_get_source(scene_ptr))
        }).await
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
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
        }).await?;

        if ptr.0.is_null() {
            return Err(ObsError::NullPointer);
        }

        source.scene_item = Some(ptr.clone());
        self.sources.write().await.push(source.clone());
        Ok(source)
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn get_source_by_index(&self, index: usize) -> Option<ObsSourceRef> {
        self.sources.read().await.get(index).map(|x| x.clone())
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn get_source_mut(&self, name: &str) -> Option<ObsSourceRef> {
        self.sources
            .read()
            .await
            .iter()
            .find(|x| x.name() == name)
            .map(|x| x.clone())
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
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
        }).await?;

        Ok(())
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn add_source_filter(&self, source_name: &str, filter_ref: &ObsFilterRef) -> Result<(), ObsError> {
        match self.sources
            .read()
            .await
            .iter()
            .find(|x| x.name() == source_name) {
        Some(source) => {
                let source_ptr = source.source.clone();
                let filter_ptr = filter_ref.source.clone();
                run_with_obs!(self.runtime, (source_ptr, filter_ptr), move || unsafe {
                    Sendable(libobs::obs_source_filter_add(source_ptr, filter_ptr))
                }).await?;
                Ok(())
            }
        _ => Err(ObsError::SourceNotFound),
        }
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn remove_source_filter(&self, source_name: &str, filter_ref: &ObsFilterRef) -> Result<(), ObsError> {
        match self.sources
            .read()
            .await
            .iter()
            .find(|x| x.name() == source_name) {
        Some(source) => {
                let source_ptr = source.source.clone();
                let filter_ptr = filter_ref.source.clone();
                run_with_obs!(self.runtime, (source_ptr, filter_ptr), move || unsafe {
                    Sendable(libobs::obs_source_filter_remove(source_ptr, filter_ptr))
                }).await?;
                Ok(())
            }
        _ => Err(ObsError::SourceNotFound),
        }
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn get_source_position(&self, name: &str) -> Result<Vec2, ObsError> {
        match self.sources
                .read()
                .await
                .iter()
                .find(|x| x.name() == name)
                .map(|x| x.scene_item.clone()) {
            Some(Some(scene_item)) => {
                let position = run_with_obs!(self.runtime, (scene_item), move || unsafe {
                    let mut main_pos: libobs::vec2 = std::mem::zeroed();
                    Sendable(libobs::obs_sceneitem_get_pos(scene_item, &mut main_pos));
                    Vec2::from(main_pos)
                }).await?;
                Ok(position)
            }
            _ => Err(ObsError::SourceNotFound),
        }
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn get_source_scale(&self, name: &str) -> Result<Vec2, ObsError> {
        match self.sources
                .read()
                .await
                .iter()
                .find(|x| x.name() == name)
                .map(|x| x.scene_item.clone()) {
            Some(Some(scene_item)) => {
                let scale = run_with_obs!(self.runtime, (scene_item), move || unsafe {
                    let mut main_pos: libobs::vec2 = std::mem::zeroed();
                    Sendable(libobs::obs_sceneitem_get_scale(scene_item, &mut main_pos));
                    Vec2::from(main_pos)
                }).await?;
                Ok(scale)
            }
            _ => Err(ObsError::SourceNotFound),
        }
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn set_source_position(&self, name: &str, position: Vec2) -> Result<(), ObsError> {
        match self.sources
                .read()
                .await
                .iter()
                .find(|x| x.name() == name)
                .map(|x| x.scene_item.clone()) {
            Some(Some(scene_item)) => {
                run_with_obs!(self.runtime, (scene_item), move || unsafe {
                    Sendable(libobs::obs_sceneitem_set_pos(scene_item, &position.into()));
                }).await?;
                Ok(())
            }
            _ => Err(ObsError::SourceNotFound),
        }
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    pub async fn set_source_scale(&self, name: &str, scale: Vec2) -> Result<(), ObsError> {
        match self.sources
                .read()
                .await
                .iter()
                .find(|x| x.name() == name)
                .map(|x| x.scene_item.clone()) {
            Some(Some(scene_item)) => {
                run_with_obs!(self.runtime, (scene_item), move || unsafe {
                    Sendable(libobs::obs_sceneitem_set_scale(scene_item, &scale.into()));
                }).await?;
                Ok(())
            }
            _ => Err(ObsError::SourceNotFound),
        }
    }

    pub fn as_ptr(&self) -> Sendable<*mut obs_scene_t> {
        Sendable(self.scene.0)
    }
}

impl_signal_manager!(|scene_ptr| {
    let source_ptr = libobs::obs_scene_get_source(scene_ptr);

    libobs::obs_source_get_signal_handler(source_ptr)
}, ObsSceneSignals for ObsSceneRef<*mut libobs::obs_scene_t>, [
    "item_add": {
        struct ItemAddSignal {
            POINTERS {
                item: *mut libobs::obs_sceneitem_t,
            }
        }
    },
    "item_remove": {
        struct ItemRemoveSignal {
            POINTERS {
                item: *mut libobs::obs_sceneitem_t,
            }
        }
    },
    "reorder": {},
    "refresh": {},
    "item_visible": {
        struct ItemVisibleSignal {
            visible: bool;
            POINTERS {
                item: *mut libobs::obs_sceneitem_t,
            }
        }
    },
    "item_locked": {
        struct ItemLockedSignal {
            locked: bool;
            POINTERS {
                item: *mut libobs::obs_sceneitem_t,
            }
        }
    },
    "item_select": {
        struct ItemSelectSignal {
            POINTERS {
                item: *mut libobs::obs_sceneitem_t,
            }
        }
    },
    "item_deselect": {
        struct ItemDeselectSignal {
            POINTERS {
                item: *mut libobs::obs_sceneitem_t,
            }
        }
    },
    "item_transform": {
        struct ItemTransformSignal {
            POINTERS {
                item: *mut libobs::obs_sceneitem_t,
            }
        }
    }
]);