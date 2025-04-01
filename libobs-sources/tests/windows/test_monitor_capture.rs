use std::{path::PathBuf, time::Duration};

use libobs_sources::windows::{
    MonitorCaptureSourceBuilder, MonitorCaptureSourceUpdater, ObsDisplayCaptureMethod,
};
use libobs_wrapper::{
    data::{ObsObjectBuilder, ObsObjectUpdater},
    sources::ObsSourceBuilder,
    utils::ObsPath,
};

use crate::common::{initialize_obs_with_log, test_video};

#[test]
pub fn monitor_list_check() {
    MonitorCaptureSourceBuilder::get_monitors().unwrap();
}

#[tokio::test]
pub async fn monitor_test() {
    let rec_file = ObsPath::from_relative("monitor_capture.mp4").build();
    let path_out = PathBuf::from(rec_file.to_string());

    let (mut context, output) = initialize_obs_with_log(rec_file, true);
    let mut scene = context.scene("main");

    let monitor = MonitorCaptureSourceBuilder::get_monitors().unwrap()[1].clone();
    println!("Using monitor {:?}", monitor);
    let mut capture_source = MonitorCaptureSourceBuilder::new("monitor_test")
        .set_monitor(&monitor)
        .add_to_scene(&mut scene)
        .unwrap();

    scene.add_and_set(0);
    let mut output = context.get_output(&output).unwrap();
    output.start().unwrap();

    println!("Recording started");
    std::thread::sleep(Duration::from_secs(5));
    println!("Testing DXGI capture method");
    MonitorCaptureSourceUpdater::create_update(&mut capture_source)
        .set_capture_method(ObsDisplayCaptureMethod::MethodDXGI)
        .update();
    std::thread::sleep(Duration::from_secs(5));

    println!("Recording stop");

    output.stop().unwrap();

    test_video(&path_out, 2.0).await.unwrap();
}
