
#[cfg(target_family = "windows")]
mod window_capture;

#[cfg(target_family = "windows")]
pub use window_capture::*;



#[cfg(test)]
mod test {
    use libobs::wrapper::sources::ObsSourceBuilder;

    use super::WindowCaptureSourceBuilder;

    #[test]
    pub fn test_capture() {
        let mut hi = WindowCaptureSourceBuilder::new("test")
        .set_window("test-window");
    }
}