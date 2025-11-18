use std::fmt::Debug;
use std::sync::{Arc, RwLock};

use getters0::Getters;
use libobs::{obs_scene_t, obs_source_t, obs_transform_info, obs_video_info};
use num_traits::ToPrimitive;

use crate::enums::ObsBounds;
use crate::{
    graphics::Vec2,
    impl_obs_drop, impl_signal_manager, run_with_obs,
    runtime::ObsRuntime,
    sources::{ObsFilterRef, ObsSourceRef},
    unsafe_send::Sendable,
    utils::{ObsError, ObsString, SourceInfo},
};

#[derive(Debug)]
struct _SceneDropGuard {
    scene: Sendable<*mut obs_scene_t>,
    runtime: ObsRuntime,
}

impl_obs_drop!(_SceneDropGuard, (scene), move || unsafe {
    let scene_source = libobs::obs_scene_get_source(scene);
    libobs::obs_source_release(scene_source);
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
    pub(crate) fn new(
        name: ObsString,
        active_scene: Arc<RwLock<Option<Sendable<*mut obs_scene_t>>>>,
        runtime: ObsRuntime,
    ) -> Result<Self, ObsError> {
        let name_ptr = name.as_ptr();
        let scene = run_with_obs!(runtime, (name_ptr), move || unsafe {
            Sendable(libobs::obs_scene_create(name_ptr))
        })?;

        let signals = Arc::new(ObsSceneSignals::new(&scene, runtime.clone())?);
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
    pub fn add_and_set(&self, channel: u32) -> Result<(), ObsError> {
        self.set_to_channel(channel)
    }

    pub fn set_to_channel(&self, channel: u32) -> Result<(), ObsError> {
        let mut s = self
            .active_scene
            .write()
            .map_err(|e| ObsError::LockError(format!("{:?}", e)))?;

        *s = Some(self.as_ptr());

        let scene_source_ptr = self.get_scene_source_ptr()?;
        run_with_obs!(self.runtime, (scene_source_ptr), move || unsafe {
            libobs::obs_set_output_source(channel, scene_source_ptr);
        })
    }

    pub fn get_scene_source_ptr(&self) -> Result<Sendable<*mut obs_source_t>, ObsError> {
        let scene_ptr = self.scene.clone();
        run_with_obs!(self.runtime, (scene_ptr), move || unsafe {
            Sendable(libobs::obs_scene_get_source(scene_ptr))
        })
    }

    pub fn add_source(&mut self, info: SourceInfo) -> Result<ObsSourceRef, ObsError> {
        let mut source = ObsSourceRef::new(
            info.id,
            info.name,
            info.settings,
            info.hotkey_data,
            self.runtime.clone(),
        )?;

        let scene_ptr = self.scene.clone();
        let source_ptr = source.source.clone();

        let ptr = run_with_obs!(self.runtime, (scene_ptr, source_ptr), move || unsafe {
            Sendable(libobs::obs_scene_add(scene_ptr, source_ptr))
        })?;

        if ptr.0.is_null() {
            return Err(ObsError::NullPointer);
        }

        source.scene_item = Some(ptr.clone());
        self.sources
            .write()
            .map_err(|e| ObsError::LockError(format!("{:?}", e)))?
            .push(source.clone());
        Ok(source)
    }

    pub fn get_source_by_index(&self, index: usize) -> Result<Option<ObsSourceRef>, ObsError> {
        let r = self
            .sources
            .read()
            .map_err(|e| ObsError::LockError(format!("{:?}", e)))?
            .get(index)
            .cloned();
        Ok(r)
    }

    pub fn get_source_mut(&self, name: &str) -> Result<Option<ObsSourceRef>, ObsError> {
        let r = self
            .sources
            .read()
            .map_err(|e| ObsError::LockError(format!("{:?}", e)))?
            .iter()
            .find(|x| x.name() == name)
            .cloned();

        Ok(r)
    }

    pub fn remove_source(&mut self, source: &ObsSourceRef) -> Result<(), ObsError> {
        let scene_item = source.scene_item.clone();
        let Some(scene_item_ptr) = scene_item else {
            return Err(ObsError::SourceNotFound);
        };

        run_with_obs!(self.runtime, (scene_item_ptr), move || unsafe {
            // Remove the scene item
            libobs::obs_sceneitem_remove(scene_item_ptr);
            // Release the scene item reference
            libobs::obs_sceneitem_release(scene_item_ptr);
        })?;

        Ok(())
    }

    pub fn add_source_filter(
        &self,
        source: &ObsSourceRef,
        filter_ref: &ObsFilterRef,
    ) -> Result<(), ObsError> {
        let source_ptr = source.source.clone();
        let filter_ptr = filter_ref.source.clone();
        run_with_obs!(self.runtime, (source_ptr, filter_ptr), move || unsafe {
            libobs::obs_source_filter_add(source_ptr, filter_ptr);
        })?;
        Ok(())
    }

    pub fn remove_source_filter(
        &self,
        source: &ObsSourceRef,
        filter_ref: &ObsFilterRef,
    ) -> Result<(), ObsError> {
        let source_ptr = source.source.clone();
        let filter_ptr = filter_ref.source.clone();
        run_with_obs!(self.runtime, (source_ptr, filter_ptr), move || unsafe {
            libobs::obs_source_filter_remove(source_ptr, filter_ptr);
        })?;
        Ok(())
    }

    pub fn get_source_position(&self, source: &ObsSourceRef) -> Result<Vec2, ObsError> {
        let scene_item = source.scene_item.clone();
        let Some(scene_item_ptr) = scene_item else {
            return Err(ObsError::SourceNotFound);
        };

        let position = run_with_obs!(self.runtime, (scene_item_ptr), move || unsafe {
            let mut main_pos: libobs::vec2 = std::mem::zeroed();
            libobs::obs_sceneitem_get_pos(scene_item_ptr, &mut main_pos);
            Vec2::from(main_pos)
        })?;

        Ok(position)
    }

    pub fn get_source_scale(&self, source: &ObsSourceRef) -> Result<Vec2, ObsError> {
        let scene_item = source.scene_item.clone();
        let Some(scene_item_ptr) = scene_item else {
            return Err(ObsError::SourceNotFound);
        };

        let scale = run_with_obs!(self.runtime, (scene_item_ptr), move || unsafe {
            let mut main_pos: libobs::vec2 = std::mem::zeroed();
            libobs::obs_sceneitem_get_scale(scene_item_ptr, &mut main_pos);
            Vec2::from(main_pos)
        })?;

        Ok(scale)
    }

    pub fn set_source_position(
        &self,
        source: &ObsSourceRef,
        position: Vec2,
    ) -> Result<(), ObsError> {
        let scene_item = source.scene_item.clone();
        let Some(scene_item_ptr) = scene_item else {
            return Err(ObsError::SourceNotFound);
        };

        run_with_obs!(self.runtime, (scene_item_ptr), move || unsafe {
            libobs::obs_sceneitem_set_pos(scene_item_ptr, &position.into());
        })?;

        Ok(())
    }

    pub fn set_source_scale(&self, source: &ObsSourceRef, scale: Vec2) -> Result<(), ObsError> {
        let scene_item = source.scene_item.clone();
        let Some(scene_item_ptr) = scene_item else {
            return Err(ObsError::SourceNotFound);
        };

        run_with_obs!(self.runtime, (scene_item_ptr), move || unsafe {
            libobs::obs_sceneitem_set_scale(scene_item_ptr, &scale.into());
        })?;

        Ok(())
    }

    /// Fits the given source to the screen size.
    /// If the source is locked, no action is taken.
    ///
    /// Returns `Ok(true)` if the source was resized, `Ok(false)` if the source was locked and not resized.
    pub fn fit_source_to_screen(&self, source_info: &ObsSourceRef) -> Result<bool, ObsError> {
        let scene_item = source_info.scene_item.clone();

        if scene_item.is_none() {
            return Err(ObsError::SourceNotFound);
        }

        let scene_item_ptr = scene_item.unwrap();
        let is_locked = {
            run_with_obs!(self.runtime, (scene_item_ptr), move || unsafe {
                libobs::obs_sceneitem_locked(scene_item_ptr)
            })?
        };

        if is_locked {
            return Ok(false);
        }

        let ovi = run_with_obs!(self.runtime, (), move || unsafe {
            let mut ovi = std::mem::MaybeUninit::<obs_video_info>::uninit();
            libobs::obs_get_video_info(ovi.as_mut_ptr());

            Sendable(ovi.assume_init())
        })?;

        let bounds_crop = run_with_obs!(self.runtime, (scene_item_ptr), move || unsafe {
            libobs::obs_sceneitem_get_bounds_crop(scene_item_ptr)
        })?;

        let item_info = obs_transform_info {
            pos: Vec2::new(0.0, 0.0).into(),
            scale: Vec2::new(1.0, 1.0).into(),
            alignment: libobs::OBS_ALIGN_LEFT | libobs::OBS_ALIGN_TOP,
            rot: 0.0,
            bounds: Vec2::new(ovi.0.base_width as f32, ovi.0.base_height as f32).into(),
            bounds_type: ObsBounds::ScaleInner
                .to_i32()
                .expect("Failed to convert ObsBounds to i32"),
            bounds_alignment: libobs::OBS_ALIGN_CENTER,
            crop_to_bounds: bounds_crop,
        };

        let item_info = Sendable(item_info);
        run_with_obs!(self.runtime, (scene_item_ptr, item_info), move || unsafe {
            libobs::obs_sceneitem_set_info2(scene_item_ptr, &item_info);
        })?;

        Ok(true)
    }

    pub fn as_ptr(&self) -> Sendable<*mut obs_scene_t> {
        Sendable(self.scene.0)
    }
}

impl_signal_manager!(|scene_ptr| unsafe {
    let source_ptr = libobs::obs_scene_get_source(scene_ptr);

    libobs::obs_source_get_signal_handler(source_ptr)
}, ObsSceneSignals for ObsSceneRef<*mut obs_scene_t>, [
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
