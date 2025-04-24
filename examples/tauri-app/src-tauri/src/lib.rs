use std::{
    pin::Pin,
    sync::{atomic::AtomicBool, Arc, RwLock},
};

use bootstrap_status::ObsTauriStatusHandler;
use lazy_static::lazy_static;
use libobs_sources::{windows::MonitorCaptureSourceBuilder, ObsSourceBuilder};
use libobs_wrapper::{
    bootstrap::ObsBootstrapperOptions, context::{ObsContext, ObsContextReturn}, display::{ObsDisplayCreationData, ObsDisplayRef, WindowPositionTrait}, encoders::{ObsContextEncoders, ObsVideoEncoderType}, utils::{AudioEncoderInfo, OutputInfo, VideoEncoderInfo}
};
use tauri::{AppHandle, Emitter, Manager};
mod bootstrap_status;

lazy_static! {
    pub static ref IS_BOOTSTRAPPING: AtomicBool = AtomicBool::new(false);
    pub static ref OBS_CONTEXT: Arc<RwLock<Option<ObsContext>>> = Arc::new(RwLock::new(None));
    pub static ref OBS_DISPLAY: Arc<RwLock<Option<Pin<Box<ObsDisplayRef>>>>> =
        Arc::new(RwLock::new(None));
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn add_preview(handle: AppHandle, x: u32, y: u32, width: u32, height: u32) -> String {
    // Get a clone of the runtime to avoid holding the lock across the await
    let ctx = {
        let ctx = OBS_CONTEXT.read().unwrap();
        ctx.as_ref().map(|r| r.clone())
    };

    // Use the cloned runtime, which is now outside the lock scope
    if let Some(mut ctx) = ctx {
        if OBS_DISPLAY.read().unwrap().is_some() {
            return "Display already exists".to_string();
        }

        let window = handle.get_webview_window("main").unwrap();
        let handle = window.hwnd().unwrap().0 as isize;

        let opt = ObsDisplayCreationData::new(handle, x, y, width, height);
        let display = ctx.display(opt).await.unwrap();
        OBS_DISPLAY.write().unwrap().replace(display);

        format!("Display created at ({}, {}, {}, {})", x, y, width, height)
    } else {
        "OBS runtime is not initialized".to_string()
    }
}

#[tauri::command]
async fn resize_preview(x: u32, y: u32, width: u32, height: u32) -> String {
    let display = OBS_DISPLAY.read().unwrap().as_ref().cloned();
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
    let display = OBS_DISPLAY.write().unwrap().take();
    let ctx = OBS_CONTEXT.read().unwrap().as_ref().cloned();
    if ctx.is_none() {
        return "OBS runtime is not initialized".to_string();
    }

    let mut ctx = ctx.unwrap();
    if let Some(display) = display {
        ctx.remove_display(&display).await;

        OBS_DISPLAY.write().unwrap().take();
        return "Display closed".to_string();
    }
    "Display is not initialized".to_string()
}

#[tauri::command]
async fn bootstrap(handle: AppHandle) -> String {
    if OBS_CONTEXT.read().unwrap().is_some() {
        return "OBS is already running.".to_string();
    }

    if IS_BOOTSTRAPPING.load(std::sync::atomic::Ordering::SeqCst) {
        return "OBS is already starting.".to_string();
    }

    IS_BOOTSTRAPPING.store(true, std::sync::atomic::Ordering::SeqCst);
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
            output_settings.set_string("path", "recording.mp4").await.unwrap();

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

            println!("Storing context");
            let mut scene = context.scene("Test Scene").await.unwrap();
            scene.add_and_set(0).await.unwrap();

            context
                .source_builder::<MonitorCaptureSourceBuilder, _>("Monitor")
                .await
                .unwrap()
                .set_monitor(&MonitorCaptureSourceBuilder::get_monitors().unwrap()[0])
                .add_to_scene(&mut scene)
                .await
                .unwrap();

            OBS_CONTEXT.write().unwrap().replace(context);
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

    IS_BOOTSTRAPPING.store(false, std::sync::atomic::Ordering::SeqCst);
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
            close_preview
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
