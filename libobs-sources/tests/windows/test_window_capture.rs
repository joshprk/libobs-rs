use std::{
    cmp,
    io::{stdout, Write},
    path::PathBuf,
    process::Command,
    time::Duration,
};

use crate::common::{initialize_obs, assert_not_black};
use libobs_sources::windows::{
    ObsWindowCaptureMethod, WindowCaptureSourceBuilder, WindowCaptureSourceUpdater,
};
use libobs_window_helper::{WindowInfo, WindowSearchMode};
use libobs_wrapper::{data::ObsObjectUpdater, unsafe_send::Sendable};
use libobs_wrapper::{
    sources::ObsSourceBuilder,
    utils::{traits::ObsUpdatable, ObsPath},
};

fn find_notepad() -> Option<Sendable<WindowInfo>> {
    let windows =
        WindowCaptureSourceBuilder::get_windows(WindowSearchMode::ExcludeMinimized).unwrap();
    println!("{:?}", windows);
    windows.into_iter().find(|w| {
        w.0.class
            .as_ref()
            .is_some_and(|e| e.to_lowercase().contains("notepad"))
    })
}

#[test]
// For this test to work, notepad must be open
pub fn test_window_capture() {
    let rec_file = ObsPath::from_relative("window_capture.mp4").build();
    let path_out = PathBuf::from(rec_file.to_string());

    let mut window = find_notepad();
    if window.is_none() {
        Command::new("notepad.exe").spawn().unwrap();
        std::thread::sleep(Duration::from_millis(350));

        window = find_notepad();
    }

    let window = window.expect("Couldn't find notepad window");

    println!("Recording {:?}", window.0.obs_id);

    let (mut context, mut output) = initialize_obs(rec_file);
    let mut scene = context.scene("main").unwrap();
    scene.set_to_channel(0).unwrap();

    let source_name = "test_capture";
    let mut source = context
        .source_builder::<WindowCaptureSourceBuilder, _>(source_name)
        .unwrap()
        .set_capture_method(ObsWindowCaptureMethod::MethodAuto)
        .set_window(&window)
        .add_to_scene(&mut scene)
        .unwrap();

    output.start().unwrap();
    println!("Recording started");

    let windows = WindowCaptureSourceBuilder::get_windows(WindowSearchMode::ExcludeMinimized)
        .unwrap()
        .into_iter()
        .filter(|e| e.0.obs_id.to_lowercase().contains("code"))
        .collect::<Vec<_>>();
    for i in 0..cmp::min(5, windows.len()) {
        let w = windows.get(i).unwrap();
        println!("Setting to {:?}", w.0.obs_id);

        source
            .create_updater::<WindowCaptureSourceUpdater>()
            .unwrap()
            .set_window(w)
            .update()
            .unwrap();

        println!("Recording for {} seconds", i);
        stdout().flush().unwrap();
        std::thread::sleep(Duration::from_secs(5));
    }
    println!("Recording stop");

    output.stop().unwrap();

    assert_not_black(&path_out, 1.0);
}
