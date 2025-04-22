use std::{path::PathBuf, time::Duration};

use libobs_sources::windows::{
    MonitorCaptureSourceBuilder, MonitorCaptureSourceUpdater, ObsDisplayCaptureMethod,
};
use libobs_wrapper::{
    data::ObsObjectUpdater,
    sources::ObsSourceBuilder,
    utils::{traits::ObsUpdatable, ObsPath},
};

use crate::common::{initialize_obs, test_video};

#[test]
pub fn monitor_list_check() {
    MonitorCaptureSourceBuilder::get_monitors().unwrap();
}

/// DXGI is not supported for now
const ENABLE_DXGI_TEST: bool = false;

#[tokio::test]
pub async fn monitor_test() {
    let rec_file = ObsPath::from_relative("monitor_capture.mp4").build();
    let path_out = PathBuf::from(rec_file.to_string());

    let (mut context, mut output) = initialize_obs("monitor_capture.mp4").await;
    let mut scene = context.scene("main").await.unwrap();

    let monitor = MonitorCaptureSourceBuilder::get_monitors().unwrap()[0].clone();
    println!("Using monitor {:?}", monitor);

    let mut capture_source = context
        .source_builder::<MonitorCaptureSourceBuilder>("monitor_capture")
        .await
        .unwrap()
        .set_monitor(&monitor)
        .add_to_scene(&mut scene)
        .await
        .unwrap();

    scene.add_and_set(0).await.unwrap();
    output.start().await.unwrap();

    println!("Recording started");
    std::thread::sleep(Duration::from_secs(5));
    if ENABLE_DXGI_TEST {
        println!("Testing DXGI capture method");
        capture_source
            .create_updater::<MonitorCaptureSourceUpdater>()
            .await
            .unwrap()
            .set_capture_method(ObsDisplayCaptureMethod::MethodDXGI)
            .update()
            .await
            .unwrap();

        std::thread::sleep(Duration::from_secs(5));
    }
    println!("Recording stop");

    output.stop().await.unwrap();

    test_video(&path_out, if ENABLE_DXGI_TEST { 2.0 } else { 1.0 })
        .await
        .unwrap();
}
