use std::sync::{Arc, RwLock};

use libobs_sources::windows::MonitorCaptureSourceBuilder;
use libobs_wrapper::context::ObsContextReturn;
use libobs_wrapper::data::video::ObsVideoInfo;
use libobs_wrapper::display::{ObsDisplayCreationData, WindowPositionTrait};
use libobs_wrapper::encoders::{ObsContextEncoders, ObsVideoEncoderType};
use libobs_wrapper::unsafe_send::Sendable;
use libobs_wrapper::utils::{AudioEncoderInfo, OutputInfo, VideoEncoderInfo};
use libobs_wrapper::{context::ObsContext, sources::ObsSourceBuilder, utils::StartupInfo};
use tokio::task;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winit::window::{Window, WindowId};

struct App {
    window: Arc<RwLock<Option<Sendable<Window>>>>,
    // Notice: Refs should never be stored in a struct, it could cause memory leaks or crashes, thats why
    // we are using a boolean here and fetching the display afterwards
    display_id: Arc<RwLock<Option<usize>>>,
    context: Arc<tokio::sync::RwLock<ObsContext>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(
                Window::default_attributes().with_inner_size(LogicalSize::new(1920 / 2, 1080 / 2)),
            )
            .unwrap();

        let size = window.inner_size();
        let width = size.width;
        let height = size.height;

        let hwnd = window.window_handle().unwrap().as_raw();
        let hwnd = if let RawWindowHandle::Win32(hwnd) = hwnd {
            hwnd.hwnd
        } else {
            panic!("Expected a Win32 window handle");
        };

        let hwnd = Sendable(hwnd);
        let w = self.window.clone();
        let display = self.display_id.clone();
        let ctx = self.context.clone();
        task::spawn(async move {
            let hwnd = hwnd;
            let data = ObsDisplayCreationData::new(
                hwnd.0.get(),
                0,
                0,
                width,
                height,
            );

            let display_id = ctx
                .write()
                .await
                .display(data)
                .await
                .unwrap();

            w.write().unwrap().replace(Sendable(window));
            display.write().unwrap().replace(display_id);
        });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let window = self.window.read().unwrap();
        if window.is_none() {
            return;
        }

        let window = window.as_ref().unwrap();
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                println!("Stopping output...");
                if let Some(display_id) = *self.display_id.read().unwrap() {
                    let ctx = self.context.clone();

                    task::spawn(async move {
                        ctx.write().await.remove_display_by_id(display_id).await;
                    });
                }

                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                window.0.request_redraw();
            }
            WindowEvent::Resized(size) => {
                let width = size.width;
                let orig_height = size.height;
                let aspect_ratio = 16.0 / 9.0;
                let height = (width as f32 / aspect_ratio) as u32;

                if orig_height != height {
                    // Keeping the aspect ratio of 16 / 9
                    let _ = self
                        .window
                        .read()
                        .unwrap()
                        .as_ref()
                        .unwrap()
                        .0
                        .request_inner_size(LogicalSize::new(width, height));
                }

                if let Some(display_id) = *self.display_id.read().unwrap() {
                    let ctx = self.context.clone();
                    task::spawn(async move {
                        let display = ctx
                            .write()
                            .await
                            .get_display_by_id(display_id)
                            .await
                            .unwrap();

                        // A real application would probably want to check the aspect ratio of the output
                        display.set_size(width, height).await.unwrap();
                    });
                }
            }
            _ => (),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let info = StartupInfo::new().set_video_info(ObsVideoInfo::default());

    let context = ObsContext::new(info).await?;
    let mut context = match context {
        ObsContextReturn::Done(c) => c,
        ObsContextReturn::Restart => {
            return Ok(());
        }
    };

    // Set up output to ./recording.mp4
    let mut output_settings = context.data().await?;
    output_settings.set_string("path", "recording.mp4").await?;

    let output_name = "output";
    let output_info = OutputInfo::new("ffmpeg_muxer", output_name, Some(output_settings), None);

    let mut output = context.output(output_info).await?;

    // Register the video encoder
    let mut video_settings = context.data().await?;
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
        .await?;

    let encoders = context.get_available_video_encoders().await?;

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

    let video_handler = context.get_video_ptr().await?;
    output.video_encoder(video_info, video_handler).await?;

    // Register the audio encoder
    let mut audio_settings = context.data().await?;
    audio_settings.set_int("bitrate", 160).await?;

    let audio_info =
        AudioEncoderInfo::new("ffmpeg_aac", "audio_encoder", Some(audio_settings), None);

    let audio_handler = context.get_audio_ptr().await?;
    output.audio_encoder(audio_info, 0, audio_handler).await?;

    let mut scene = context.scene("Main Scene").await?;

    context
        .source_builder::<MonitorCaptureSourceBuilder, _>("Monitor Capture")
        .await?
        .set_monitor(&MonitorCaptureSourceBuilder::get_monitors()?[1])
        .add_to_scene(&mut scene)
        .await?;

    scene.add_and_set(0).await?;

    let event_loop = EventLoop::new()?;
    let mut app = App {
        window: Arc::new(RwLock::new(None)),
        display_id: Arc::new(RwLock::new(None)),
        context: Arc::new(tokio::sync::RwLock::new(context)),
    };

    event_loop.run_app(&mut app)?;

    println!("Done with mainloop.");
    Ok(())
}
