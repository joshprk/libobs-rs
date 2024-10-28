//! For this display method to work, another preview window has to be created in order to create a swapchain
//! This is because the main window renderer is already handled by other processes

mod buffers;
mod creation_data;
mod enums;
mod window_manager;

pub(crate) use buffers::*;
pub use creation_data::*;
pub use enums::*;
pub use window_manager::{DisplayWindowManager, WindowPositionTrait};

use std::{
    ffi::{c_void, CString},
    sync::atomic::AtomicUsize,
};

use libobs::{
    gs_draw, gs_draw_mode_GS_TRISTRIP, gs_effect_get_param_by_name, gs_effect_get_technique,
    gs_effect_set_vec4, gs_load_vertexbuffer, gs_matrix_identity, gs_matrix_pop, gs_matrix_push,
    gs_matrix_scale3f, gs_ortho, gs_projection_pop, gs_projection_push, gs_set_viewport, gs_technique_begin, gs_technique_begin_pass, gs_technique_end,
    gs_technique_end_pass, gs_viewport_pop, gs_viewport_push, obs_base_effect_OBS_EFFECT_SOLID,
    obs_get_base_effect, obs_get_video_info, obs_render_main_texture,
    obs_video_info, vec4_create, vec4_set,
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

unsafe fn draw_backdrop(buffers: &VertexBuffers, width: f32, height: f32) {
    let solid = obs_get_base_effect(obs_base_effect_OBS_EFFECT_SOLID);
    let c = CString::new("color").unwrap();

    let color = gs_effect_get_param_by_name(solid, c.as_ptr());
    let s = CString::new("Solid").unwrap();
    let tech = gs_effect_get_technique(solid, s.as_ptr());

    let mut color_val = vec4_create();
    vec4_set(&mut color_val, 0.0, 0.0, 0.0, 1.0);
    gs_effect_set_vec4(color, &color_val);

    gs_technique_begin(tech);
    gs_technique_begin_pass(tech, 0);
    gs_matrix_push();
    gs_matrix_identity();
    gs_matrix_scale3f(width as f32, height as f32, 1.0);

    let box_v = buffers.box_buffer.lock().unwrap();
    gs_load_vertexbuffer(box_v.0);
    drop(box_v);

    gs_draw(gs_draw_mode_GS_TRISTRIP, 0, 0);

    gs_matrix_pop();
    gs_technique_end_pass(tech);
    gs_technique_end(tech);

    gs_load_vertexbuffer(std::ptr::null_mut());
}

unsafe extern "C" fn render_display(data: *mut c_void, _cx: u32, _cy: u32) {
    let s = &mut *(data as *mut ObsDisplay);
    let m = &s.manager;

    let (x, y) = m.get_pos();
    let (width, height) = m.get_size();

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
    pub fn create(&mut self) -> () {
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
