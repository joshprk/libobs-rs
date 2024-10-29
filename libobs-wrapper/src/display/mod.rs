//! For this display method to work, another preview window has to be created in order to create a swapchain
//! This is because the main window renderer is already handled by other processes

mod buffers;
mod creation_data;
mod enums;
mod window_manager;

pub(crate) use buffers::*;
pub use creation_data::*;
pub use enums::*;
pub use window_manager::*;

use std::{
    ffi::c_void,
    sync::atomic::AtomicUsize,
};

use libobs::{
    gs_ortho, gs_projection_pop, gs_projection_push, gs_set_viewport, gs_viewport_pop, gs_viewport_push, obs_get_video_info, obs_render_main_texture,
    obs_video_info,
};

use crate::unsafe_send::WrappedObsDisplay;

static ID_COUNTER: AtomicUsize = AtomicUsize::new(1);
#[derive(Debug)]
pub struct ObsDisplay {
    display: WrappedObsDisplay,
    id: usize,

    _buffers: VertexBuffers,
    is_initialized: bool,

    // Keep for window
    manager: DisplayWindowManager,
}

unsafe extern "C" fn render_display(data: *mut c_void, _cx: u32, _cy: u32) {
    let s = &mut *(data as *mut ObsDisplay);

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

impl ObsDisplay {
    #[cfg(target_family = "windows")]
    /// Call initialize to ObsDisplay#create the display
    pub fn new(
        buffers: &VertexBuffers,
        data: creation_data::ObsDisplayCreationData,
    ) -> windows::core::Result<Self> {
        use std::sync::atomic::Ordering;

        use creation_data::ObsDisplayCreationData;
        use libobs::gs_window;
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

        let mut manager = DisplayWindowManager::new(
            parent_window.clone(),
            x as i32,
            y as i32,
            width,
            height,
        )?;

        let child_handle = manager.get_child_handle();
        let init_data = data.build(gs_window {
            hwnd: child_handle.0,
        });

        log::trace!("Creating obs display...");
        let display = unsafe { libobs::obs_display_create(&init_data, background_color) };

        manager.obs_display = Some(WrappedObsDisplay(display));
        Ok(Self {
            display: WrappedObsDisplay(display),
            manager,
            id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            _buffers: buffers.clone(),
            is_initialized: false,
        })
    }

    /// Adds draw callbacks to the display.
    pub fn create(&mut self) {
        if self.is_initialized {
            return;
        }

        log::trace!("Adding draw callback with display {:?}...", self.display.0);
        unsafe {
            libobs::obs_display_add_draw_callback(
                self.display.0,
                Some(render_display),
                self as *mut _ as *mut c_void,
            );
        }

        log::trace!("Display created!");
        self.is_initialized = true;
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn manager_mut(&mut self) -> &mut DisplayWindowManager {
        &mut self.manager
    }
}

impl Drop for ObsDisplay {
    fn drop(&mut self) {
        unsafe {
            libobs::obs_display_remove_draw_callback(
                self.display.0,
                Some(render_display),
                std::ptr::null_mut(),
            );
            libobs::obs_display_destroy(self.display.0);
        }
    }
}
