//! For this display method to work, another preview window has to be created in order to create a swapchain
//! This is because the main window renderer is already handled by other processes

mod creation_data;
mod enums;
mod window_manager;

pub use creation_data::*;
pub use enums::*;
pub use window_manager::*;

use std::{
    ffi::c_void,
    marker::PhantomPinned,
    sync::{atomic::AtomicUsize, Arc, RwLock},
};

use libobs::{
    gs_ortho, gs_projection_pop, gs_projection_push, gs_set_viewport, gs_viewport_pop,
    gs_viewport_push, obs_get_video_info, obs_render_main_texture, obs_video_info,
};

use crate::{run_with_obs, runtime::ObsRuntime, unsafe_send::Sendable, utils::ObsError};

static ID_COUNTER: AtomicUsize = AtomicUsize::new(1);
#[derive(Debug, Clone)]
//TODO: This has to be checked again, I'm unsure with pinning and draw callbacks from OBS
/// # NEVER STORE THIS REF DIRECTLY!!
/// This is a wrapper around the obs_display struct and contains direct memory references.
/// You should ALWAYS use the context to get to this struct, and as said NEVER store it.
pub struct ObsDisplayRef {
    display: Sendable<*mut libobs::obs_display_t>,
    id: usize,

    // The callbacks and obs display first
    _guard: Arc<RwLock<_DisplayDropGuard>>,

    // Keep for window, manager is accessed by render thread as well so Arc and RwLock
    manager: Arc<RwLock<DisplayWindowManager>>,
    /// This must not be moved in memory as the draw callback is a raw pointer to this struct
    _fixed_in_heap: PhantomPinned,

    /// Stored so the obs context is not dropped while this is alive
    pub(crate) runtime: ObsRuntime,
}

unsafe extern "C" fn render_display(data: *mut c_void, _cx: u32, _cy: u32) {
    let s = &*(data as *mut ObsDisplayRef);

    let r = s.get_size();
    if r.is_err() {
        log::error!("Failed to get display size: {:?}", r.err());
        return;
    }

    let (width, height) = r.unwrap();
    let mut ovi: obs_video_info = std::mem::zeroed();
    obs_get_video_info(&mut ovi);

    gs_viewport_push();
    gs_projection_push();

    gs_ortho(
        0.0f32,
        ovi.base_width as f32,
        0.0f32,
        ovi.base_height as f32,
        -100.0f32,
        100.0f32,
    );
    gs_set_viewport(0, 0, width as i32, height as i32);
    //draw_backdrop(&s.buffers, ovi.base_width as f32, ovi.base_height as f32);

    obs_render_main_texture();

    gs_projection_pop();
    gs_viewport_pop();
}

impl ObsDisplayRef {
    #[cfg(target_family = "windows")]
    /// Call initialize to ObsDisplay#create the display
    /// NOTE: This must be pinned to prevent the draw callbacks from having a invalid pointer. DO NOT UNPIN
    pub(crate) fn new(
        data: creation_data::ObsDisplayCreationData,
        runtime: ObsRuntime,
    ) -> anyhow::Result<std::pin::Pin<Box<Self>>> {
        use std::sync::atomic::Ordering;

        use anyhow::bail;
        use creation_data::ObsDisplayCreationData;
        use libobs::gs_window;
        use window_manager::DisplayWindowManager;

        use crate::run_with_obs;

        let ObsDisplayCreationData {
            x,
            y,
            height,
            width,
            window_handle,
            background_color,
            create_child,
            ..
        } = data.clone();

        let mut manager = if create_child {
            DisplayWindowManager::new_child(window_handle.clone(), x, y, width, height)?
        } else {
            DisplayWindowManager::new(window_handle.clone(), x, y, width, height)
        };

        let preview_window_handle = Sendable(manager.get_window_handle());
        let init_data = Sendable(data.build(gs_window {
            hwnd: preview_window_handle.0 .0,
        }));

        log::trace!("Creating obs display...");
        let display = run_with_obs!(runtime, (init_data), move || unsafe {
            Sendable(libobs::obs_display_create(&init_data, background_color))
        })?;

        if display.0.is_null() {
            bail!("OBS failed to create display");
        }

        manager.obs_display = Some(display.clone());
        let mut instance = Box::pin(Self {
            display: display.clone(),
            manager: Arc::new(RwLock::new(manager)),
            id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            _guard: Arc::new(RwLock::new(_DisplayDropGuard {
                display,
                self_ptr: None,
                runtime: runtime.clone(),
            })),
            _fixed_in_heap: PhantomPinned,
            runtime: runtime.clone(),
        });

        let instance_ptr =
            Sendable(unsafe { instance.as_mut().get_unchecked_mut() as *mut _ as *mut c_void });

        instance
            ._guard
            .write()
            .map_err(|e| ObsError::LockError(format!("{:?}", e)))?
            .self_ptr = Some(instance_ptr.clone());

        let pos = instance.get_pos();
        log::trace!(
            "Adding draw callback with display {:?} and draw callback params at {:?} (pos is {:?})...",
            instance.display,
            instance_ptr,
            pos
        );
        let display_ptr = instance.display.clone();
        run_with_obs!(runtime, (display_ptr, instance_ptr), move || unsafe {
            libobs::obs_display_add_draw_callback(display_ptr, Some(render_display), instance_ptr);
        })?;

        Ok(instance)
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

#[derive(Debug)]
struct _DisplayDropGuard {
    display: Sendable<*mut libobs::obs_display_t>,
    self_ptr: Option<Sendable<*mut c_void>>,
    runtime: ObsRuntime,
}

impl _DisplayDropGuard {
    pub fn inner_drop(
        r: ObsRuntime,
        display: Sendable<*mut libobs::obs_display_t>,
        self_ptr: Option<Sendable<*mut c_void>>,
    ) -> Result<(), ObsError> {
        run_with_obs!(r, (display), move || unsafe {
            if let Some(ptr) = &self_ptr {
                log::trace!("Destroying display with callback at {:?}...", ptr.0);
                libobs::obs_display_remove_draw_callback(display, Some(render_display), ptr.0);
            }

            libobs::obs_display_destroy(display);
        })
    }
}

impl Drop for _DisplayDropGuard {
    fn drop(&mut self) {
        log::trace!("Dropping ObsDisplayRef guard...");

        let display = self.display.clone();
        let self_ptr = self.self_ptr.clone();
        let r = self.runtime.clone();

        #[cfg(not(feature = "no_blocking_drops"))]
        {
            let r = _DisplayDropGuard::inner_drop(r, display, self_ptr);
            if std::thread::panicking() {
                return;
            }

            r.unwrap();
        }

        #[cfg(feature = "no_blocking_drops")]
        {
            tokio::task::spawn_blocking(move || {
                _DisplayDropGuard::inner_drop(r, display, self_ptr).unwrap();
            });
        }
    }
}
