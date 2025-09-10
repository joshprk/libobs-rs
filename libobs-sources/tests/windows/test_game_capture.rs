use std::{path::PathBuf, time::Duration};

use libobs_sources::windows::{GameCaptureSourceBuilder, MonitorCaptureSourceBuilder, ObsGameCaptureMode};
use libobs_wrapper::{
    sources::ObsSourceBuilder,
    utils::ObsPath,
};

use crate::common::{initialize_obs, test_video};

#[tokio::test]
pub async fn game_test() {
    let rec_file = ObsPath::from_relative("game_capture.mp4").build();
    let path_out = PathBuf::from(rec_file.to_string());

    let (mut context, mut output) = initialize_obs(rec_file).await;
    let mut scene = context.scene("main").await.unwrap();


    let game = GameCaptureSourceBuilder::get_windows(libobs_window_helper::WindowSearchMode::ExcludeMinimized).unwrap();
    let game = game.iter().find(|e| e.title.is_some() && e.title.as_ref().unwrap().contains("Bloons")).unwrap();

    println!("Using window: {:?}", game);

    let capture_source = context
        .source_builder::<GameCaptureSourceBuilder, _>("game_capture")
        .await
        .unwrap()
        .set_capture_mode(ObsGameCaptureMode::Any)
        .add_to_scene(&mut scene)
        .await
        .unwrap();

    scene.set_to_channel(0).await.unwrap();
    output.start().await.unwrap();

    println!("Recording started");
    std::thread::sleep(Duration::from_secs(5));
    println!("Recording stop");

    let x = capture_source.id();
    output.stop().await.unwrap();

    test_video(&path_out, 1.0)
        .await
        .unwrap();
}
