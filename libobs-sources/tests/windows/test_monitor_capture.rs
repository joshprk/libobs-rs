use std::{path::PathBuf, time::Duration};

use libobs_sources::windows::MonitorCaptureSourceBuilder;
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
    let scene = context.scene("main");

    MonitorCaptureSourceBuilder::new("monitor_test")
        //.set_monitor(&MonitorCaptureSourceBuilder::get_monitors().unwrap()[0])
        .set_monitor_id_raw("\\\\?\\DISPLAY#Default_Monitor#1&1f0c3c2f&0&UID256#{e6f07b5f-ee97-4a90-b076-33f57bf4eaa7}")
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
