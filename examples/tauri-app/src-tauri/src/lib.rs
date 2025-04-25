use std::{pin::Pin, sync::Arc};

use bootstrap_status::ObsTauriStatusHandler;
use lazy_static::lazy_static;
use libobs_sources::{
    windows::{MonitorCaptureSourceBuilder, MonitorCaptureSourceUpdater},
    ObsObjectUpdater, ObsSourceBuilder,
};
use libobs_wrapper::{
    bootstrap::ObsBootstrapperOptions,
    context::{ObsContext, ObsContextReturn},
    display::{ObsDisplayCreationData, ObsDisplayRef, WindowPositionTrait},
    encoders::{ObsContextEncoders, ObsVideoEncoderType},
    sources::ObsSourceRef,
    utils::{traits::ObsUpdatable, AudioEncoderInfo, OutputInfo, VideoEncoderInfo},
};
use tauri::{async_runtime::RwLock, AppHandle, Emitter, Manager};
mod bootstrap_status;

pub struct CurrState {
    pub monitor_index: usize,
    pub is_bootstrapping: bool,
    pub obs_context: Option<ObsContext>,
    pub obs_source: Option<ObsSourceRef>,
    pub obs_display: Option<Pin<Box<ObsDisplayRef>>>,
}

impl CurrState {
    pub fn new() -> Self {
        CurrState {
            monitor_index: 1,
            is_bootstrapping: false,
            obs_context: None,
            obs_source: None,
            obs_display: None,
        }
    }
}

lazy_static! {
    pub static ref CURR_STATE: Arc<RwLock<CurrState>> = Arc::new(RwLock::new(CurrState::new()));
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn add_preview(handle: AppHandle, x: u32, y: u32, width: u32, height: u32) -> String {
    let state = CURR_STATE.read().await;
    if state.obs_display.is_some() {
        return "Display already exists".to_string();
    }

    if state.obs_context.is_none() {
        return "OBS runtime is not initialized".to_string();
    }

    drop(state);
    let mut state = CURR_STATE.write().await;

    // Get a clone of the runtime to avoid holding the lock across the await
    let ctx = { state.obs_context.as_ref().map(|r| r.clone()) };

    // Use the cloned runtime, which is now outside the lock scope
    let r = if let Some(mut ctx) = ctx {
        let window = handle.get_webview_window("main").unwrap();
        let handle = window.hwnd().unwrap().0 as isize;

        let opt = ObsDisplayCreationData::new(handle, x, y, width, height);
        let display = ctx.display(opt).await.unwrap();

        state.obs_display = Some(display);
        format!("Display created at ({}, {}, {}, {})", x, y, width, height)
    } else {
        "OBS runtime is not initialized".to_string()
    };

    r
}

#[tauri::command]
async fn resize_preview(x: u32, y: u32, width: u32, height: u32) -> String {
    let display = CURR_STATE
        .read()
        .await
        .obs_display
        .as_ref()
        .map(|d| d.clone());

    if let Some(display) = display {
        display.set_size(width, height).await.unwrap();
        display.set_pos(x as i32, y as i32).await.unwrap();

        format!("Display resized to ({}, {}, {}, {})", x, y, width, height)
    } else {
        "Display is not initialized".to_string()
    }
}

#[tauri::command]
async fn close_preview() -> String {
    let state = CURR_STATE.read().await;
    if state.obs_display.is_none() {
        return "Display is not initialized".to_string();
    }

    drop(state);
    let mut state = CURR_STATE.write().await;
    let display = state.obs_display.take().unwrap();
    let ctx = state.obs_context.clone();

    if ctx.is_none() {
        return "OBS runtime is not initialized".to_string();
    }

    let mut ctx = ctx.unwrap();
    ctx.remove_display(&display).await;

    "Display closed".to_string()
}

#[tauri::command]
async fn switch_monitor() -> String {
    let state = CURR_STATE.read().await;
    if state.obs_source.is_none() {
        return "Source is not initialized".to_string();
    }

    let ctx = state.obs_context.clone();
    if ctx.is_none() {
        return "OBS runtime is not initialized".to_string();
    }

    drop(state);
    let mut state = CURR_STATE.write().await;
    let monitors = MonitorCaptureSourceBuilder::get_monitors().unwrap();

    state.monitor_index = (state.monitor_index + 1) % monitors.len();
    let monitor = &monitors[state.monitor_index];

    let mut source = state.obs_source.as_ref().unwrap().clone();
    source
        .create_updater::<MonitorCaptureSourceUpdater>()
        .await
        .unwrap()
        .set_monitor(monitor)
        .update()
        .await
        .unwrap();

    format!("Monitor switched to {:?}", monitor)
}

#[tauri::command]
async fn bootstrap(handle: AppHandle) -> String {
    let state = CURR_STATE.read().await;
    if state.obs_context.is_some() {
        return "OBS is already running.".to_string();
    }

    println!("Starting OBS bootstrapper");
    drop(state);
    let state = CURR_STATE.try_write();
    if state.is_err() {
        println!("Already bootstrapping");
        return "Already bootstrapping".to_string();
    }

    let mut state = state.unwrap();
    println!("State locked");
    let mut options = ObsBootstrapperOptions::default();

    // Don't restart in debug mode to avoid issues with the tauri dev server
    // and the bootstrapper trying to restart the app.
    // The restart itself should be handled by tauri
    if cfg!(debug_assertions) {
        options = options.set_no_restart();
    }

    let f = ObsContext::builder()
        .enable_bootstrapper(
            ObsTauriStatusHandler {
                handle: handle.clone(),
            },
            options,
        )
        .start();

    let r = f.await.expect("Failed to start OBS context");
    match r {
        ObsContextReturn::Done(mut context) => {
            // Set up output to ./recording.mp4
            let mut output_settings = context.data().await.unwrap();
            output_settings
                .set_string("path", "recording.mp4")
                .await
                .unwrap();

            let output_name = "output";
            let output_info =
                OutputInfo::new("ffmpeg_muxer", output_name, Some(output_settings), None);

            let mut output = context.output(output_info).await.unwrap();

            // Register the video encoder
            let mut video_settings = context.data().await.unwrap();
            video_settings
                .bulk_update()
                .set_int("bf", 0)
                .set_bool("psycho_aq", true)
                .set_bool("lookahead", true)
                .set_string("profile", "high")
                .set_string("preset", "fast")
                .set_string("rate_control", "cbr")
                .set_int("bitrate", 10000)
                .update()
                .await
                .unwrap();

            let encoders = context.get_available_video_encoders().await.unwrap();

            println!("Available encoders: {:?}", encoders);
            let encoder = encoders
                .iter()
                .find(|e| {
                    **e == ObsVideoEncoderType::H264_TEXTURE_AMF
                        || **e == ObsVideoEncoderType::AV1_TEXTURE_AMF
                })
                .unwrap();

            println!("Using encoder {:?}", encoder);
            let video_info =
                VideoEncoderInfo::new(encoder.clone(), "video_encoder", Some(video_settings), None);

            let video_handler = context.get_video_ptr().await.unwrap();
            output
                .video_encoder(video_info, video_handler)
                .await
                .unwrap();

            // Register the audio encoder
            let mut audio_settings = context.data().await.unwrap();
            audio_settings.set_int("bitrate", 160).await.unwrap();

            let audio_info =
                AudioEncoderInfo::new("ffmpeg_aac", "audio_encoder", Some(audio_settings), None);

            let audio_handler = context.get_audio_ptr().await.unwrap();
            output
                .audio_encoder(audio_info, 0, audio_handler)
                .await
                .unwrap();

            let mut scene = context.scene("Test Scene").await.unwrap();
            scene.set_to_channel(0).await.unwrap();

            let source = context
                .source_builder::<MonitorCaptureSourceBuilder, _>("Monitor")
                .await
                .unwrap()
                .set_monitor(&MonitorCaptureSourceBuilder::get_monitors().unwrap()[0])
                .add_to_scene(&mut scene)
                .await
                .unwrap();

            state.obs_source = Some(source.clone());
            state.obs_context = Some(context.clone());
        }
        ObsContextReturn::Restart => {
            println!("Restarting OBS context");
            // Unclean exit so the app gets restarted by tauri if in debug mode
            if cfg!(debug_assertions) {
                println!("Restarting in debug mode. You'll need to do it manually.");
            }

            handle.exit(0);
        }
    }

    println!("Runtime done");
    handle.emit("bootstrap_done", ()).unwrap();

    "Done.".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            add_preview,
            bootstrap,
            resize_preview,
            close_preview,
            switch_monitor
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
