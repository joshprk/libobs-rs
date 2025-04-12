// Example demonstrating useful flows using libobs-wrapper

use std::{thread::sleep, time::Duration};

use libobs_wrapper::{context::ObsContext, data::{properties::{types::ObsListItemValue, ObsProperty, ObsPropertyObject}, ObsData}, sources::ObsSourceRef, utils::{AudioEncoderInfo, ObsPath, ObsString, OutputInfo, SourceInfo, StartupInfo, VideoEncoderInfo}};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let startup_info = StartupInfo::new();
    let mut context = ObsContext::new(startup_info)?;

    // Create a new main scene
    let scene = context.scene(ObsString::new("MAIN"));
    // Set the scene as main channel for video and audio output
    scene.add_and_set(0);
    scene.add_and_set(1);
    // Add a ffmpeg_muxer output
    let mut output = context.output(OutputInfo::new(ObsString::new("ffmpeg_muxer"), ObsString::new("MAIN"), None, None))?;

    // Read all the properties of source type or encoders
    let properties = ObsSourceRef::get_properties_by_id(ObsString::new("window_capture"))?;
    println!("Properties: {:?}", properties);

    // Property name as key and its value can be passed as settings to the encoder while creating or updating the encoder

    // Adding a default video and audio encoder
    output.video_encoder(VideoEncoderInfo::new("obs_x264", "video_encoder", None, None), ObsContext::get_video_ptr()?)?;
    output.audio_encoder(AudioEncoderInfo::new("ffmpeg_aac", "audio_encoder", None, None), 0, ObsContext::get_audio_ptr()?)?;

    // In case we already have the encoder and we want to use the same encoder for multiple outputs, use:
    // output.set_video_encoder(ObsVideoEncoder)?;
    // output.set_audio_encoder(ObsAudioEncoder)?;

    // Can update the output path to record to a different location
    let mut settings = ObsData::new();
    settings.set_string("path", ObsPath::from_relative("obs_output.mp4"));

    // Update path
    context.update_output("MAIN", settings)?;

    // To get the list of all monitors
    // It has a loop hole though, somehow the monitor_id returned in property is same if we have multiple monitor of exactly same model (exactly same monitor), use `libobs-window-helper` lib for fix
    let properties = ObsSourceRef::get_properties_by_id(ObsString::new("monitor_capture"))?;

    let mut source_settings = ObsData::new();

    // Read the monitor_id from the property
    if let Some(prop) = properties.iter().find(|p| {
        if let ObsProperty::List(list) = p {
            list.name().eq("monitor_id")
        } else {
            false
        }
    }) {
        if let ObsProperty::List(list) = prop {
            if list.items().len() > 0 {
                if let ObsListItemValue::String(value) = list.items()[0].value() {
                    source_settings.set_string("monitor_id", value.to_string());
                }
            }
        }
    }

    // method 2 is WGC
    source_settings.set_int("method", 2);

    // Prepare the source info
    let source_info = SourceInfo::new("monitor_capture", "Display name", Some(source_settings), None);

    // Get the main scene from the context (Even if we have it as we have set it here only, this can be useful when we want to fetch with the help of context)
    let mut scene = context.get_scene("MAIN").unwrap();
    scene.add_source(source_info)?;

    // Start the recording
    // For the same reason, fetching output from context, can be used if we already have the previous context of output
    let mut scene = context.get_output("MAIN").unwrap();
    scene.start()?;

    sleep(Duration::from_secs(15));

    // Stop the recording
    scene.stop()?;

    // Remove the source by its name, if it is needed, in case we want to keep running the main context and want to drop not required sources
    let mut scene = context.get_scene("MAIN").unwrap();
    scene.remove_source(ObsString::new("Display name"))?;

    Ok(())
}
