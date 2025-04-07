use std::{path::PathBuf, time::Duration};

use libobs_sources::windows::{
    MonitorCaptureSourceBuilder, MonitorCaptureSourceUpdater, ObsDisplayCaptureMethod,
};
use libobs_wrapper::{
    data::{ObsObjectBuilder, ObsObjectUpdater},
    sources::ObsSourceBuilder,
    utils::ObsPath,
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

    let mut context = initialize_obs("monitor_capture.mp4");
    let mut scene = context.scene("main");

    let monitor = MonitorCaptureSourceBuilder::get_monitors().unwrap()[1].clone();
    println!("Using monitor {:?}", monitor);
    let mut capture_source = MonitorCaptureSourceBuilder::new("monitor_test")
        .set_monitor(&monitor)
        .add_to_scene(&mut scene)
        .unwrap();

    scene.add_and_set(0);
    let mut output = context.outputs().borrow()[0].clone();
    output.start().unwrap();

    println!("Recording started");
    std::thread::sleep(Duration::from_secs(5));
    if ENABLE_DXGI_TEST {
        println!("Testing DXGI capture method");
        MonitorCaptureSourceUpdater::create_update(&mut capture_source)
            .set_capture_method(ObsDisplayCaptureMethod::MethodDXGI)
            .update();
        std::thread::sleep(Duration::from_secs(5));
    }
    println!("Recording stop");

    output.stop().unwrap();

    test_video(&path_out, if ENABLE_DXGI_TEST { 2.0 } else { 1.0 })
        .await
        .unwrap();
}
