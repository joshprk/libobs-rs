// Example demonstrating useful flows using libobs-wrapper

use std::{thread::sleep, time::Duration};

use libobs_sources::{windows::{MonitorCaptureSourceBuilder, ObsDisplayCaptureMethod}, ObsSourceBuilder};
use libobs_wrapper::{
    context::ObsContext, data::properties::{types::ObsListItemValue, ObsProperty, ObsPropertyObject}, sources::ObsSourceRef, utils::{
        AudioEncoderInfo, ObsPath, OutputInfo, StartupInfo, VideoEncoderInfo,
    }
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let startup_info = StartupInfo::new();
    let context = ObsContext::new(startup_info).await?;
    let mut context = match context {
        libobs_wrapper::context::ObsContextReturn::Done(c) => c,
        libobs_wrapper::context::ObsContextReturn::Restart => {
            return Ok(());
        }
    };

    // Create a new main scene
    let mut scene = context.scene("MAIN").await?;
    // Set the scene as main channel for video and audio output
    scene.add_and_set(0).await?;
    scene.add_and_set(1).await?;

    // Add a ffmpeg_muxer output
    let mut output = context
        .output(OutputInfo::new("ffmpeg_muxer", "MAIN", None, None))
        .await?;

    // Read all the properties of source type or encoders
    let source = context
        .source_builder::<MonitorCaptureSourceBuilder, _>("Display name")
        .await?
        .add_to_scene(&mut scene)
        .await?;

    let properties = ObsSourceRef::get_properties_by_id("monitor_capture", context.runtime()).await?;
    println!("Properties: {:?}", properties);

    // Property name as key and its value can be passed as settings to the encoder while creating or updating the encoder

    // Adding a default video and audio encoder
    let vid_ptr = context.get_video_ptr().await?;
    let audio_ptr = context.get_audio_ptr().await?;

    output
        .video_encoder(
            VideoEncoderInfo::new("obs_x264", "video_encoder", None, None),
            vid_ptr,
        )
        .await?;
    output
        .audio_encoder(
            AudioEncoderInfo::new("ffmpeg_aac", "audio_encoder", None, None),
            0,
            audio_ptr,
        )
        .await?;

    // In case we already have the encoder and we want to use the same encoder for multiple outputs, use:
    // output.set_video_encoder(ObsVideoEncoder)?;
    // output.set_audio_encoder(ObsAudioEncoder)?;

    // Can update the output path to record to a different location
    let mut settings = context.data().await?;
    settings.set_string("path", ObsPath::from_relative("obs_output.mp4")).await?;

    // Update path
    context.update_output("MAIN", settings).await?;
    // To get the list of all monitors
    // It has a loop hole though, somehow the monitor_id returned in property is same if we have multiple monitor of exactly same model (exactly same monitor), use `libobs-window-helper` lib for fix
    let properties = source.get_properties().await?;
    let mut builder: MonitorCaptureSourceBuilder = context.source_builder("Display name").await?;

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
                    builder = builder.set_monitor_id_raw(value.to_string());
                }
            }
        }
    }

    // method 2 is WGC
    builder.
        set_capture_method(ObsDisplayCaptureMethod::MethodWgc)
        .add_to_scene(&mut scene)
        .await?;

    output.start().await?;

    sleep(Duration::from_secs(15));

    // Stop the recording
    output.stop().await?;


    // Remove the source from the scene
    scene.remove_source(&source).await?;

    Ok(())
}
