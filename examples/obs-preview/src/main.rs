use std::cell::RefCell;
use std::rc::Rc;

use libobs_sources::windows::MonitorCaptureSourceBuilder;
use libobs_wrapper::data::video::ObsVideoInfo;
use libobs_wrapper::data::{ObsData, ObsObjectBuilder};
use libobs_wrapper::display::{ObsDisplayCreationData, WindowPositionTrait};
use libobs_wrapper::encoders::{ObsContextEncoders, ObsVideoEncoderType};
use libobs_wrapper::utils::{AudioEncoderInfo, OutputInfo, VideoEncoderInfo};
use libobs_wrapper::{context::ObsContext, sources::ObsSourceBuilder, utils::StartupInfo};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winit::window::{Window, WindowId};

struct App {
    window: Option<Window>,
    // Notice: Refs should never be stored in a struct, it could cause memory leaks or crashes, thats why
    // we are using a boolean here and fetching the display afterwards
    display_id: Option<usize>,
    context: Rc<RefCell<ObsContext>>,
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

        let display_id = self
            .context
            .borrow_mut()
            .display(ObsDisplayCreationData::new(hwnd.get(), 0, 0, width, height))
            .unwrap();

        self.display_id = Some(display_id);
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                println!("Stopping output...");
                if let Some(display_id) = self.display_id {
                    self.context.borrow_mut().remove_display_by_id(display_id);
                }

                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
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
                        .as_ref()
                        .unwrap()
                        .request_inner_size(LogicalSize::new(width, height));
                }

                if let Some(display_id) = self.display_id {
                    let display = self
                        .context
                        .borrow_mut()
                        .get_display_by_id(display_id)
                        .unwrap();
                    // A real application would probably want to check the aspect ratio of the output
                    display.set_size(width, height).unwrap();
                }
            }
            _ => (),
        }
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let info = StartupInfo::new().set_video_info(ObsVideoInfo::default());

    let context = Rc::new(RefCell::new(ObsContext::new(info)?));

    // Set up output to ./recording.mp4
    let mut output_settings = ObsData::new();
    output_settings.set_string("path", "recording.mp4");

    let output_name = "output";
    let output_info = OutputInfo::new("ffmpeg_muxer", output_name, Some(output_settings), None);

    let mut output = context.borrow_mut().output(output_info)?;

    // Register the video encoder
    let mut video_settings = ObsData::new();
    video_settings
        .set_int("bf", 0)
        .set_bool("psycho_aq", true)
        .set_bool("lookahead", true)
        .set_string("profile", "high")
        .set_string("preset", "fast")
        .set_string("rate_control", "cbr")
        .set_int("bitrate", 10000);

    let encoders = ObsContext::get_available_video_encoders();

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

    let video_handler = ObsContext::get_video_ptr()?;
    output.video_encoder(video_info, video_handler)?;

    // Register the audio encoder
    let mut audio_settings = ObsData::new();
    audio_settings.set_int("bitrate", 160);

    let audio_info =
        AudioEncoderInfo::new("ffmpeg_aac", "audio_encoder", Some(audio_settings), None);

    let audio_handler = ObsContext::get_audio_ptr()?;
    output.audio_encoder(audio_info, 0, audio_handler)?;

    let mut scene = context.borrow_mut().scene("Main Scene");

    MonitorCaptureSourceBuilder::new("Monitor Capture")
        .set_monitor(&MonitorCaptureSourceBuilder::get_monitors()?[1])
        .add_to_scene(&mut scene)?;

    scene.add_and_set(0);

    let event_loop = EventLoop::new()?;
    let mut app = App {
        window: None,
        display_id: None,
        context: context.clone(),
    };

    event_loop.run_app(&mut app)?;

    println!("Done with mainloop.");
    Ok(())
}
