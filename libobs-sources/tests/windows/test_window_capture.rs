use std::{path::PathBuf, process::Command, time::Duration};

use libobs_sources::windows::WindowCaptureSourceBuilder;
use libobs_window_helper::{WindowInfo, WindowSearchMode};
use libobs_wrapper::{data::ObsObjectBuilder, sources::ObsSourceBuilder, utils::ObsPath};

use crate::common::{initialize_obs, test_video};

fn find_notepad() -> Option<WindowInfo> {
    let windows =
        WindowCaptureSourceBuilder::get_windows(WindowSearchMode::ExcludeMinimized).unwrap();
    println!("{:?}", windows);
    windows.into_iter().find(|w| {
        w.class
            .as_ref()
            .is_some_and(|e| e.to_lowercase().contains("notepad"))
    })
}

#[tokio::test]
// For this test to work, notepad must be open
pub async fn test_window_capture() {
    let rec_file = ObsPath::from_relative("window_capture.mp4").build();
    let path_out = PathBuf::from(rec_file.to_string());

    let mut window = find_notepad();
    if window.is_none() {
        Command::new("notepad.exe").spawn().unwrap();
        std::thread::sleep(Duration::from_millis(350));

        window = find_notepad();
    }

    let window = window.expect("Couldn't find notepad window");

    println!("Recording {:?}", window);

    let (mut context, output) = initialize_obs(rec_file);
    let scene = context.scene("main");

    WindowCaptureSourceBuilder::new("test_capture")
        .set_window(&window)
        .add_to_scene(scene)
        .unwrap();

    scene.add_and_set(0);

    let output = context.get_output(&output).unwrap();
    output.start().unwrap();
    println!("Recording started");
    std::thread::sleep(Duration::from_secs(5));
    println!("Recording stop");

    output.stop().unwrap();

    test_video(&path_out).await.unwrap();
}
