use std::pin::Pin;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, RwLock};

use libobs_sources::{
    windows::{MonitorCaptureSourceBuilder, MonitorCaptureSourceUpdater},
    ObsObjectUpdater,
};
use libobs_wrapper::context::ObsContextReturn;
use libobs_wrapper::data::video::ObsVideoInfo;
use libobs_wrapper::display::{ObsDisplayCreationData, ObsDisplayRef, WindowPositionTrait};
use libobs_wrapper::encoders::{ObsContextEncoders, ObsVideoEncoderType};
use libobs_wrapper::sources::ObsSourceRef;
use libobs_wrapper::unsafe_send::Sendable;
use libobs_wrapper::utils::traits::ObsUpdatable;
use libobs_wrapper::utils::{AudioEncoderInfo, OutputInfo};
use libobs_wrapper::{context::ObsContext, sources::ObsSourceBuilder, utils::StartupInfo};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winit::window::{Window, WindowId};

struct App {
    window: Arc<RwLock<Option<Sendable<Window>>>>,
    // Notice: Refs should never be stored in a struct, it could cause memory leaks or crashes, that's why
    // we are using a boolean here and fetching the display afterward
    display: Arc<RwLock<Option<Pin<Box<ObsDisplayRef>>>>>,
    context: Arc<RwLock<ObsContext>>,
    monitor_index: Arc<AtomicUsize>,
    source_ref: Arc<RwLock<ObsSourceRef>>,
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
        let d_rw = self.display.clone();
        let ctx = self.context.clone();
        let hwnd = hwnd;
        let data = ObsDisplayCreationData::new(hwnd.0.get(), 0, 0, width, height);

        let display = ctx.write().unwrap().display(data).unwrap();

        w.write().unwrap().replace(Sendable(window));
        d_rw.write().unwrap().replace(display);
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
                if let Some(display) = self.display.write().unwrap().clone() {
                    let ctx = self.context.clone();

                    ctx.write().unwrap().remove_display(&display);
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
                        .request_inner_size(PhysicalSize::new(width, height));
                }

                if let Some(display) = self.display.write().unwrap().clone() {
                    display.set_size(width, height).unwrap();
                }
            }
            WindowEvent::MouseInput { state, .. } => {
                if !matches!(state, ElementState::Pressed) {
                    return;
                }

                let tmp = self.source_ref.clone();
                let monitor_index = self.monitor_index.clone();

                let mut source = tmp.write().unwrap().clone();
                let monitors = MonitorCaptureSourceBuilder::get_monitors().unwrap();

                let monitor_index = monitor_index
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
                    % monitors.len();
                let monitor = &monitors[monitor_index];

                source
                    .create_updater::<MonitorCaptureSourceUpdater>()
                    .unwrap()
                    .set_monitor(monitor)
                    .update()
                    .unwrap();
            }
            _ => (),
        }
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let info = StartupInfo::new().set_video_info(ObsVideoInfo::default());

    let context = ObsContext::new(info)?;
    let mut context = match context {
        ObsContextReturn::Done(c) => c,
        ObsContextReturn::Restart => {
            return Ok(());
        }
    };

    // Set up output to ./recording.mp4
    let mut output_settings = context.data()?;
    output_settings.set_string("path", "recording.mp4")?;

    let output_info = OutputInfo::new("ffmpeg_muxer", "output", Some(output_settings), None);
    let mut output = context.output(output_info)?;

    // Register the video encoder
    let mut video_settings = context.data()?;
    video_settings
        .bulk_update()
        .set_int("bf", 0)
        .set_bool("psycho_aq", true)
        .set_bool("lookahead", true)
        .set_string("profile", "high")
        .set_string("preset", "fast")
        .set_string("rate_control", "cbr")
        .set_int("bitrate", 10000)
        .update()?;

    let encoders = context.available_video_encoders()?;

    println!("Available encoders: {:?}", encoders);
    let mut encoder = encoders
        .into_iter()
        .find(|e| {
            e.get_encoder_id() == &ObsVideoEncoderType::H264_TEXTURE_AMF
                || e.get_encoder_id() == &ObsVideoEncoderType::AV1_TEXTURE_AMF
        })
        .unwrap();

    encoder.set_settings(video_settings);

    println!("Using encoder {:?}", encoder.get_encoder_id());
    encoder.set_to_output(
        &mut output,
        "video_encoder"
    )?;

    // Register the audio encoder
    let mut audio_settings = context.data()?;
    audio_settings.set_int("bitrate", 160)?;

    let audio_info =
        AudioEncoderInfo::new("ffmpeg_aac", "audio_encoder", Some(audio_settings), None);

    let audio_handler = context.get_audio_ptr()?;
    output.audio_encoder(audio_info, 0, audio_handler)?;

    let mut scene = context.scene("Main Scene")?;

    let source = context
        .source_builder::<MonitorCaptureSourceBuilder, _>("Monitor Capture")?
        .set_monitor(&MonitorCaptureSourceBuilder::get_monitors()?[0])
        .add_to_scene(&mut scene)?;

    scene.set_to_channel(0)?;

    // Example for signals and events with libobs
    let tmp = source.clone();
    std::thread::spawn(move || {
        let signal_manager = tmp.signal_manager();
        let mut x = signal_manager.on_update().unwrap();

        println!("Listening for updates");
        while let Ok(_) = x.blocking_recv() {
            println!("Monitor Source has been updated!");
        }
    });

    let event_loop = EventLoop::new()?;
    let mut app = App {
        window: Arc::new(RwLock::new(None)),
        display: Arc::new(RwLock::new(None)),
        context: Arc::new(RwLock::new(context)),
        monitor_index: Arc::new(AtomicUsize::new(1)),
        source_ref: Arc::new(RwLock::new(source)),
    };

    event_loop.run_app(&mut app)?;

    println!("Done with mainloop.");
    Ok(())
}
