use libobs::{obs_source, obs_data, obs_output, obs_display_t, obs_scene_t, obs_encoder, gs_vertex_buffer, obs_video_info};
use windows::Win32::Foundation::HWND;

macro_rules! impl_send_sync {
    ($n:ident, $t:ty) => {
        #[derive(Debug)]
        pub struct $n(pub $t);
    
        #[cfg(feature="unsafe-send")]
        unsafe impl Send for $n {}
        #[cfg(feature="unsafe-send")]
        unsafe impl Sync for $n {}
    };
}

impl_send_sync! { WrappedObsData, *mut obs_data}
impl_send_sync! { WrappedObsOutput, *mut obs_output}
impl_send_sync! { WrappedObsDisplay, *mut obs_display_t}
impl_send_sync! { WrappedObsScene, *mut obs_scene_t}
impl_send_sync! { WrappedObsEncoders, *mut obs_encoder}
impl_send_sync! { WrappedGsVertexBuffer, *mut gs_vertex_buffer}
impl_send_sync! { WrappedObsVideoInfo, obs_video_info}
impl_send_sync! { WrappedObsSource, *mut obs_source}
impl_send_sync! { WrappedHWND, HWND }

impl Clone for WrappedObsVideoInfo {
    fn clone(&self) -> Self {
        WrappedObsVideoInfo(self.0.clone())
    }
}

impl PartialEq for WrappedObsVideoInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for WrappedObsVideoInfo {}