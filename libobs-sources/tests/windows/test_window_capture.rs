use std::{path::PathBuf, time::Duration};

use libobs_sources::windows::WindowCaptureSourceBuilder;
use libobs_window_helper::WindowSearchMode;
use libobs_wrapper::{sources::ObsSourceBuilder, utils::ObsPath};

use crate::common::{initialize_obs, test_video};

#[tokio::test]
// For this test to work, notepad must be open
pub async fn test_window_capture() {
    let rec_file = ObsPath::from_relative("window_capture.mp4").build();
    let path_out = PathBuf::from(rec_file.to_string());

    let (mut context, output) = initialize_obs(rec_file);
    let output = context.get_output(&output).unwrap();

    let windows = WindowCaptureSourceBuilder::get_windows(WindowSearchMode::ExcludeMinimized).unwrap();
    let window = windows.iter()
    .find(|w| w.title.as_ref().is_some_and(|e| e.to_lowercase().contains("notepad")))
    .unwrap();

    WindowCaptureSourceBuilder::new("test_capture")
        .set_window(window)
        .add_to_output(output, 0)
        .unwrap();

    output.start().unwrap();
    std::thread::sleep(Duration::from_secs(3));

    output.stop().unwrap();

    test_video(&path_out).await.unwrap();
}
