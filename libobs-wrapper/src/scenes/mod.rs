use std::{
    cell::RefCell,
    rc::Rc,
};

use getters0::Getters;
use libobs::{obs_scene_create, obs_scene_t, obs_set_output_source, obs_source_t};

use crate::{
    context::ObsContextShutdownZST, sources::ObsSourceRef, unsafe_send::WrappedObsScene, utils::{ObsError, ObsString, SourceInfo}
};

#[derive(Debug)]
struct _SceneDropGuard {
    scene: WrappedObsScene,
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
    scene: Rc<WrappedObsScene>,
    name: ObsString,
    #[get_mut]
    pub(crate) sources: Rc<RefCell<Vec<ObsSourceRef>>>,
    #[skip_getter]
    pub(crate) active_scene: Rc<RefCell<Option<WrappedObsScene>>>,

    #[skip_getter]
    _guard: Rc<_SceneDropGuard>,

    #[skip_getter]
    _shutdown: Rc<ObsContextShutdownZST>,
}

impl ObsSceneRef {
    pub(crate) fn new(name: ObsString, active_scene: Rc<RefCell<Option<WrappedObsScene>>>, shutdown: Rc<ObsContextShutdownZST>) -> Self {
        let scene = unsafe { obs_scene_create(name.as_ptr()) };

        Self {
            name,
            scene: Rc::new(WrappedObsScene(scene)),
            sources: Rc::new(RefCell::new(vec![])),
            active_scene: active_scene.clone(),
            _guard: Rc::new(_SceneDropGuard {
                scene: WrappedObsScene(scene),
            }),
            _shutdown: shutdown
        }
    }

    pub fn add_and_set(&self, channel: u32) {
        let mut s = self.active_scene.borrow_mut();
        *s = Some(WrappedObsScene(self.as_ptr()));

        unsafe {
            obs_set_output_source(channel, self.get_scene_source_ptr());
        }
    }

    pub fn get_scene_source_ptr(&self) -> *mut obs_source_t {
        unsafe { libobs::obs_scene_get_source(self.scene.0) }
    }

    pub fn add_source(&mut self, info: SourceInfo) -> Result<ObsSourceRef, ObsError> {
        let source = ObsSourceRef::new(info.id, info.name, info.settings, info.hotkey_data);

        return match source {
            Ok(x) => {
                unsafe {
                    libobs::obs_scene_add(self.scene.0, x.source.0);
                }
                let tmp = x.clone();
                self.sources.borrow_mut().push(x);
                Ok(tmp)
            }
            Err(x) => Err(x),
        };
    }

    pub fn get_source_by_index(&self, index: usize) -> Option<ObsSourceRef> {
        self.sources.borrow().get(index).map(|x| x.clone())
    }

    pub fn get_source_mut(&self, name: &str) -> Option<ObsSourceRef> {
        self.sources
        .borrow()
            .iter()
            .find(|x| x.name() == name)
            .map(|x| x.clone())
    }

    pub fn as_ptr(&self) -> *mut obs_scene_t {
        self.scene.0
    }
}
