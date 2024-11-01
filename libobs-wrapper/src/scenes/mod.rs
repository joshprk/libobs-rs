use std::sync::{Arc, Mutex};

use getters0::Getters;
use libobs::{obs_scene_create, obs_scene_t, obs_set_output_source, obs_source_t};

use crate::{
    sources::ObsSource, unsafe_send::WrappedObsScene, utils::{ObsError, ObsString, SourceInfo}
};

#[derive(Debug, Getters)]
#[skip_new]
pub struct ObsScene {
    #[skip_getter]
    scene: WrappedObsScene,
    name: ObsString,
    #[get_mut]
    pub(crate) sources: Vec<ObsSource>,
    #[skip_getter]
    pub(crate) active_scene: Arc<Mutex<Option<WrappedObsScene>>>,
}

impl ObsScene {
    pub fn new(name: ObsString, active_scene: Arc<Mutex<Option<WrappedObsScene>>>) -> Self {
        let scene = unsafe { obs_scene_create(name.as_ptr()) };

        Self {
            name,
            scene: WrappedObsScene(scene),
            sources: vec![],
            active_scene: active_scene.clone(),
        }
    }

    pub fn add_and_set(&self, channel: u32) {
        let mut s = self.active_scene.lock().unwrap();
        *s = Some(WrappedObsScene(self.as_ptr()));

        unsafe {
            obs_set_output_source(channel, self.get_source_ptr());
        }
    }

    pub fn get_source_ptr(&self) -> *mut obs_source_t {
        unsafe { libobs::obs_scene_get_source(self.scene.0) }
    }

    pub fn add_source(&mut self, info: SourceInfo) -> Result<&mut ObsSource, ObsError> {
        let source = ObsSource::new(info.id, info.name, info.settings, info.hotkey_data);

        return match source {
            Ok(x) => {
                unsafe {
                    libobs::obs_scene_add(self.scene.0, x.source.0);
                }
                self.sources.push(x);
                Ok(self.sources.last_mut().unwrap())
            }
            Err(x) => Err(x),
        };
    }

    pub fn as_ptr(&self) -> *mut obs_scene_t {
        self.scene.0
    }
}

impl Drop for ObsScene {
    fn drop(&mut self) {
        unsafe {
            libobs::obs_scene_release(self.scene.0);
        }
    }
}
