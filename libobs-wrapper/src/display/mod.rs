//! For this display method to work, another preview window has to be created in order to create a swapchain
//! This is because the main window renderer is already handled by other processes

mod buffers;
mod creation_data;
mod enums;
mod window_manager;

pub(crate) use buffers::*;
pub use enums::*;
pub use creation_data::*;
use window_manager::DisplayWindowManager;

use std::{
    ffi::{c_void, CString},
    sync::{atomic::AtomicUsize, Arc, Mutex},
};

use libobs::{
    gs_draw, gs_draw_mode_GS_TRISTRIP, gs_effect_get_param_by_name, gs_effect_get_technique,
    gs_effect_set_vec4, gs_load_vertexbuffer, gs_matrix_identity, gs_matrix_pop, gs_matrix_push,
    gs_matrix_scale3f, gs_ortho, gs_projection_pop, gs_projection_push, gs_reset_viewport,
    gs_set_viewport, gs_technique_begin, gs_technique_begin_pass, gs_technique_end,
    gs_technique_end_pass, gs_viewport_pop, gs_viewport_push, obs_base_effect_OBS_EFFECT_SOLID,
    obs_display_size, obs_get_base_effect, obs_get_video_info,
    obs_render_main_texture_src_color_only, obs_video_info, vec4_create, vec4_set,
};

use crate::unsafe_send::{WrappedObsDisplay, WrappedObsScene};

static ID_COUNTER: AtomicUsize = AtomicUsize::new(1);
#[derive(Debug)]
pub struct ObsDisplay {
    display: WrappedObsDisplay,
    id: usize,
    scale: f32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    buffers: VertexBuffers,
    active_scene: Arc<Mutex<Option<WrappedObsScene>>>,
    manager: DisplayWindowManager
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
unsafe extern "C" fn render_display_old(data: *mut c_void, _cx: u32, _cy: u32) {
    obs_render_main_texture_src_color_only();
}
unsafe extern "C" fn render_display(data: *mut c_void, _cx: u32, _cy: u32) {
    log::trace!("Rendering display...");
    let s = &mut *(data as *mut ObsDisplay);

    let mut ovi: obs_video_info = std::mem::zeroed();

    obs_get_video_info(&mut ovi);

    log::trace!("Setting width...");
    s.width = (s.scale * ovi.base_width as f32) as u32;
    s.height = (s.scale * ovi.base_height as f32) as u32;

    gs_viewport_push();
    gs_projection_push();

    let display = s.display.0;
    let mut width: u32 = 0;
    let mut height: u32 = 0;

    obs_display_size(display, &mut width, &mut height);
    let right = width as f32 - s.x as f32;
    let bottom = height as f32 - s.x as f32;

    gs_ortho(-(s.x as f32), right, -(s.y as f32), bottom, -100.0, 100.0);

    //window->ui->preview->DrawOverflow();

    /* --------------------------------------- */
    log::trace!("Ortho...");
    gs_ortho(
        0.0,
        ovi.base_width as f32,
        0.0,
        ovi.base_height as f32,
        -100.0,
        100.0,
    );
    gs_set_viewport(s.x as i32, s.y as i32, s.width as i32, s.height as i32);
    log::trace!("Backdrop...");
    draw_backdrop(&s.buffers, ovi.base_width as f32, ovi.base_height as f32);

    //let scene = s.active_scene.lock().unwrap();
    /*if let Some(s) = scene.as_ref() {
        let t = &s.0;
        if !(*t).is_null() {
            let source = obs_scene_get_source(*t);
            if source != std::ptr::null_mut() {
                obs_source_video_render(source);
            }
        }
    } else {*/
    log::trace!("Render texture...");
    obs_render_main_texture_src_color_only();
    // }

    log::trace!("Load buffer...");
    gs_load_vertexbuffer(std::ptr::null_mut());

    /* --------------------------------------- */

    gs_ortho(-(s.x as f32), right, -(s.y as f32), bottom, -100.0, 100.0);
    gs_reset_viewport();

    /*	if (window->drawSafeAreas) {
        RenderSafeAreas(window->actionSafeMargin, targetCX, targetCY);
        RenderSafeAreas(window->graphicsSafeMargin, targetCX, targetCY);
        RenderSafeAreas(window->fourByThreeSafeMargin, targetCX,
                targetCY);
        RenderSafeAreas(window->leftLine, targetCX, targetCY);
        RenderSafeAreas(window->topLine, targetCX, targetCY);
        RenderSafeAreas(window->rightLine, targetCX, targetCY);
    }
     */

    //window->ui->preview->DrawSceneEditing();
    /*
       if (window->drawSpacingHelpers)
           window->ui->preview->DrawSpacingHelpers();
    */
    /* --------------------------------------- */

    gs_projection_pop();
    gs_viewport_pop();
}

impl ObsDisplay {
    #[cfg(target_family = "windows")]
    pub fn new(
        buffers: &VertexBuffers,
        active_scene: &Arc<Mutex<Option<WrappedObsScene>>>,
        data: creation_data::ObsDisplayCreationData,
    ) -> windows::core::Result<Self> {
        use creation_data::ObsDisplayCreationData;
        use libobs::gs_window;
        use std::sync::atomic::Ordering;
        use window_manager::DisplayWindowManager;

        let ObsDisplayCreationData {
            x,
            y,
            height,
            width,
            scale,
            parent_window,
            background_color,
            ..
        } = data.clone();

        let manager = DisplayWindowManager::new(
            parent_window.clone(),
            x as i32,
            y as i32,
            width as i32,
            height as i32,
        )?;

        let child_handle = manager.get_child_handle();
        let init_data = data.build(gs_window { hwnd: child_handle.0 });

        log::trace!("Creating obs display...");
        let display = unsafe { libobs::obs_display_create(&init_data, background_color) };

        let s = Self {
            display: WrappedObsDisplay(display),
            manager,
            id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            width,
            height,
            scale,
            x,
            y,
            buffers: buffers.clone(),
            active_scene: active_scene.clone(),
        };

        log::trace!("Adding draw callback...");
        unsafe {
            libobs::obs_display_add_draw_callback(
                display,
                Some(render_display),
                std::ptr::addr_of!(s) as *mut _,
            );
        }

        log::trace!("Display created!");
        Ok(s)
    }

    pub fn get_id(&self) -> usize {
        self.id
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
