mod buffers;
mod enums;
pub(crate) use buffers::*;
pub use enums::*;

use std::{
    ffi::{c_void, CString},
    sync::{atomic::AtomicUsize, Arc, Mutex},
};

use libobs::{
    gs_draw, gs_draw_mode_GS_TRISTRIP, gs_effect_get_param_by_name, gs_effect_get_technique, gs_effect_set_vec4, gs_init_data, gs_load_vertexbuffer, gs_matrix_identity, gs_matrix_pop, gs_matrix_push, gs_matrix_scale3f, gs_ortho, gs_projection_pop, gs_projection_push, gs_reset_viewport, gs_set_viewport, gs_technique_begin, gs_technique_begin_pass, gs_technique_end, gs_technique_end_pass, gs_viewport_pop, gs_viewport_push, gs_window, obs_base_effect_OBS_EFFECT_SOLID, obs_display_size, obs_get_base_effect, obs_get_video_info, obs_scene_get_source, obs_source_video_render, obs_video_info, vec4_create, vec4_set
};
use num_traits::ToPrimitive;

use crate::unsafe_send::{WrappedObsDisplay, WrappedObsScene};

static ID_COUNTER: AtomicUsize = AtomicUsize::new(1);
pub struct ObsDisplayCreationData {
    #[cfg(target_family = "windows")]
    window: windows::Win32::Foundation::HWND,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    format: GsColorFormat,
    zsformat: GsZstencilFormat,
    adapter: u32,
    backbuffers: u32,
    background_color: u32,
    scale: f32,
}

impl ObsDisplayCreationData {
    #[cfg(target_family = "windows")]
    pub fn new(window: windows::Win32::Foundation::HWND, x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            window,
            format: GsColorFormat::BGRA,
            zsformat: GsZstencilFormat::ZSNone,
            x,
            y,
            width,
            height,
            adapter: 0,
            backbuffers: 0,
            background_color: 0,
            scale: 1.0,
        }
    }

    pub fn set_format(mut self, format: GsColorFormat) -> Self {
        self.format = format;
        self
    }

    pub fn set_zsformat(mut self, zsformat: GsZstencilFormat) -> Self {
        self.zsformat = zsformat;
        self
    }

    pub fn set_adapter(mut self, adapter: u32) -> Self {
        self.adapter = adapter;
        self
    }

    pub fn set_backbuffers(mut self, backbuffers: u32) -> Self {
        self.backbuffers = backbuffers;
        self
    }

    pub fn set_background_color(mut self, background_color: u32) -> Self {
        self.background_color = background_color;
        self
    }

    pub fn set_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    fn build(self) -> (gs_init_data, u32, f32, (u32, u32)) {
        let data = gs_init_data {
            cx: self.width,
            cy: self.height,
            format: self.format.to_i32().unwrap(),
            zsformat: self.zsformat.to_i32().unwrap(),
            window: gs_window {
                #[cfg(target_family = "windows")]
                hwnd: std::ptr::addr_of!(self.window.0) as *mut _,
            },
            adapter: self.adapter,
            num_backbuffers: self.backbuffers,
        };

        (data, self.background_color, self.scale, (self.x, self.y))
    }
}

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

    let mut ovi: obs_video_info = std::mem::zeroed();

    obs_get_video_info(&mut ovi);

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

    gs_ortho(
        0.0,
        ovi.base_width as f32,
        0.0,
        ovi.base_height as f32,
        -100.0,
        100.0,
    );
    gs_set_viewport(s.x as i32, s.y as i32, s.width as i32, s.height as i32);

    draw_backdrop(&s.buffers, ovi.base_width as f32, ovi.base_height as f32);

    let scene = s.active_scene.lock().unwrap();
    if let Some(s) = scene.as_ref() {
        let t = &s.0;
        if !(*t).is_null() {
            let source = obs_scene_get_source(*t);
            if source != std::ptr::null_mut() {
                obs_source_video_render(source);
            }
        }
    }
    /* else {
        obs_render_main_texture_src_color_only();
    }*/

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
        data: ObsDisplayCreationData,
    ) -> Self {
        use std::sync::atomic::Ordering;

        let (data, background_color, scale, pos) = data.build();

        let display = unsafe { libobs::obs_display_create(&data, background_color) };
        // Don't think we actually need to save the creation data

        let s = Self {
            display: WrappedObsDisplay(display),
            id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            width: data.cx,
            height: data.cy,
            scale,
            x: pos.0,
            y: pos.1,
            buffers: buffers.clone(),
            active_scene: active_scene.clone(),
        };

        unsafe {
            libobs::obs_display_add_draw_callback(
                display,
                Some(render_display),
                std::ptr::addr_of!(s) as *mut _,
            );
        }

        s
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

impl Drop for ObsDisplay {
    fn drop(&mut self) {
        unsafe {
            libobs::obs_display_destroy(self.display.0);
        }
    }
}
