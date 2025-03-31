use std::{path::PathBuf, time::Duration};

use libobs_sources::windows::{MonitorCaptureSourceBuilder, ObsDisplayCaptureMethod};
use libobs_wrapper::{data::ObsObjectBuilder, utils::ObsPath, sources::ObsSourceBuilder};

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
    MonitorCaptureSourceBuilder::new("monitor_test")
        .set_monitor(&monitor)
        // Set the method as WGC (based on hit and trial)
        .set_capture_method(ObsDisplayCaptureMethod::MethodWgc)
        .add_to_scene(&mut scene)
        .unwrap();

    scene.add_and_set(0);
    let mut output = context.get_output(&output).unwrap();
    output.start().unwrap();
    println!("Recording started");
    std::thread::sleep(Duration::from_secs(5));
    println!("Recording stop");

    output.stop().unwrap();

    test_video(&path_out).await.unwrap();
}
