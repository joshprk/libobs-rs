use std::sync::{Arc, Mutex};

use libobs::{
    gs_render_save, gs_render_start, gs_vertex2f, gs_vertexbuffer_destroy,
    obs_enter_graphics, obs_leave_graphics,
};

use crate::unsafe_send::WrappedGsVertexBuffer;

#[derive(Debug, Clone)]
pub struct VertexBuffers {
    pub box_buffer: Arc<Mutex<WrappedGsVertexBuffer>>,
}

impl VertexBuffers {
    pub unsafe fn initialize() -> Self {
        obs_enter_graphics();

        gs_render_start(true);
        gs_vertex2f(0.0, 0.0);
        gs_vertex2f(0.0, 1.0);
        gs_vertex2f(1.0, 0.0);
        gs_vertex2f(1.0, 1.0);
        let b = gs_render_save();

        obs_leave_graphics();

        Self {
            box_buffer: Arc::new(Mutex::new(WrappedGsVertexBuffer(b))),
        }
    }
}

impl Drop for VertexBuffers {
    fn drop(&mut self) {
        unsafe {
            obs_enter_graphics();
            gs_vertexbuffer_destroy(self.box_buffer.lock().unwrap().0);
            obs_leave_graphics();
        }
    }
}
