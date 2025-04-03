//! For this display method to work, another preview window has to be created in order to create a swapchain
//! This is because the main window renderer is already handled by other processes

mod creation_data;
mod enums;
mod window_manager;

pub use creation_data::*;
pub use enums::*;
use parking_lot::RwLock;
pub use window_manager::*;

use std::{
    cell::RefCell,
    ffi::c_void,
    marker::PhantomPinned,
    rc::Rc,
    sync::{atomic::AtomicUsize, Arc},
};

use libobs::{
    gs_ortho, gs_projection_pop, gs_projection_push, gs_set_viewport, gs_viewport_pop,
    gs_viewport_push, obs_get_video_info, obs_render_main_texture, obs_video_info,
};

use crate::unsafe_send::{WrappedObsDisplay, WrappedVoidPtr};

static ID_COUNTER: AtomicUsize = AtomicUsize::new(1);
#[derive(Debug, Clone)]
//TODO: This has to be checked again, I'm unsure with pinning and draw callbacks from OBS
/// # NEVER STORE THIS REF DIRECTLY!!
/// This is a wrapper around the obs_display struct and contains direct memory references.
/// You should ALWAYS use the context to get to this struct, and as said NEVER store it.
pub struct ObsDisplayRef {
    display: Rc<WrappedObsDisplay>,
    id: usize,

    // The callbacks and obs display first
    _guard: Rc<RefCell<_DisplayDropGuard>>,

    // Keep for window, manager is accessed by render thread as well so Arc and RwLock
    manager: Arc<RwLock<DisplayWindowManager>>,
    /// This must not be moved in memory as the draw callback is a raw pointer to this struct
    _fixed_in_heap: PhantomPinned,
}

unsafe extern "C" fn render_display(data: *mut c_void, _cx: u32, _cy: u32) {
    let s = &*(data as *mut ObsDisplayRef);

    let (x, y) = s.get_pos();
    let (width, height) = s.get_size();

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
    gs_set_viewport(x as i32, y as i32, width as i32, height as i32);
    //draw_backdrop(&s.buffers, ovi.base_width as f32, ovi.base_height as f32);

    obs_render_main_texture();

    gs_projection_pop();
    gs_viewport_pop();
}

impl ObsDisplayRef {
    #[cfg(target_family = "windows")]
    /// Call initialize to ObsDisplay#create the display
    /// NOTE: This must be pinned to prevent the draw callbacks from having a invalid pointer. DO NOT UNPIN
    pub(crate) fn new(data: creation_data::ObsDisplayCreationData) -> anyhow::Result<std::pin::Pin<Box<Self>>> {
        use std::{cell::RefCell, sync::atomic::Ordering};

        use anyhow::bail;
        use creation_data::ObsDisplayCreationData;
        use libobs::gs_window;
        use parking_lot::lock_api::RwLock;
        use window_manager::DisplayWindowManager;

        let ObsDisplayCreationData {
            x,
            y,
            height,
            width,
            parent_window,
            background_color,
            ..
        } = data.clone();

        let mut manager =
            DisplayWindowManager::new(parent_window.clone(), x as i32, y as i32, width, height)?;

        let child_handle = manager.get_child_handle();
        let init_data = data.build(gs_window {
            hwnd: child_handle.0,
        });

        log::trace!("Creating obs display...");
        let display = unsafe { libobs::obs_display_create(&init_data, background_color) };
        if display.is_null() {
            bail!("OBS failed to create display");
        }

        manager.obs_display = Some(WrappedObsDisplay(display));

        let mut instance = Box::pin(Self {
            display: Rc::new(WrappedObsDisplay(display)),
            manager: Arc::new(RwLock::new(manager)),
            id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            _guard: Rc::new(RefCell::new(_DisplayDropGuard {
                display: WrappedObsDisplay(display),
                self_ptr: None,
            })),
            _fixed_in_heap: PhantomPinned,
        });

        let instance_ptr = unsafe { instance.as_mut().get_unchecked_mut() as *mut _ as *mut c_void };

        instance._guard.borrow_mut().self_ptr = Some(WrappedVoidPtr(instance_ptr));

        log::trace!(
            "Adding draw callback with display {:?} and draw callback params at {:?} (pos is {:?})...",
            instance.display,
            instance_ptr,
            instance.get_pos()
        );
        unsafe {
            libobs::obs_display_add_draw_callback(
                instance.display.0,
                Some(render_display),
                instance_ptr,
            );
        }

        Ok(instance)
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

#[derive(Debug)]
struct _DisplayDropGuard {
    display: WrappedObsDisplay,
    self_ptr: Option<WrappedVoidPtr>,
}

impl Drop for _DisplayDropGuard {
    fn drop(&mut self) {
        unsafe {
            if let Some(ptr) = &self.self_ptr {
                log::trace!("Destroying display with callback at {:?}...", ptr.0);
                libobs::obs_display_remove_draw_callback(
                    self.display.0,
                    Some(render_display),
                    ptr.0,
                );
            }

            libobs::obs_display_destroy(self.display.0);
        }
    }
}
