use std::{cmp, io::{stdout, Write}, path::PathBuf, process::Command, time::Duration};

use libobs_sources::windows::{WindowCaptureSourceBuilder, WindowCaptureSourceUpdater};
use libobs_window_helper::{WindowInfo, WindowSearchMode};
use libobs_wrapper::{data::ObsObjectBuilder, sources::ObsSourceBuilder, utils::{traits::ObsUpdatable, ObsPath}};
use libobs_wrapper::data::ObsObjectUpdater;
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

    let mut context = initialize_obs(rec_file);
    let mut scene = context.scene("main");
    scene.add_and_set(0);

    let source_name = "test_capture";
    WindowCaptureSourceBuilder::new(source_name)
        .set_window(&window)
        .add_to_scene(&mut scene)
        .unwrap();

    let mut output = context.outputs().borrow()[0].clone();
    output.start().unwrap();
    println!("Recording started");

    let windows =
        WindowCaptureSourceBuilder::get_windows(WindowSearchMode::ExcludeMinimized).unwrap()
        .into_iter()
        .filter(|e| e.obs_id.to_lowercase().contains("code"))
        .collect::<Vec<_>>();
    for i in 0..cmp::min(5, windows.len()) {
        let mut source = context.scenes_mut().borrow_mut().get_mut(0).unwrap().get_source_by_index(0).unwrap();
        let w = windows.get(i).unwrap();
        println!("Setting to {:?}", w.obs_id);
        source.create_updater::<WindowCaptureSourceUpdater>()
            .set_window(w)
            .update();

        println!("Recording for {} seconds", i);
        stdout().flush().unwrap();
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
    println!("Recording stop");

    output.stop().unwrap();

    test_video(&path_out, 1.0).await.unwrap();
}
