mod common;

use anyhow::Result;
use libobs_wrapper::context::ObsContext;
use libobs_wrapper::encoders::ObsContextEncoders;
use libobs_wrapper::enums::ObsLogLevel;
use libobs_wrapper::logger::ObsLogger;
use libobs_wrapper::utils::{AudioEncoderInfo, ObsPath, OutputInfo, SourceInfo, StartupInfo};
use pixels::{Pixels, SurfaceTexture};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const RECORDING_DURATION_SECS: u64 = 5;

#[derive(Debug)]
struct TestLogger;

impl ObsLogger for TestLogger {
    fn log(&mut self, level: ObsLogLevel, msg: String) {
        println!("[{:?}] {}", level, msg);
    }
}

struct AnimatedTestWindow {
    window: Option<Window>,
    pixels: Option<Pixels>,
    start_time: Instant,
    should_close: Arc<AtomicBool>,
}

impl AnimatedTestWindow {
    fn new(should_close: Arc<AtomicBool>) -> Self {
        Self {
            window: None,
            pixels: None,
            start_time: Instant::now(),
            should_close,
        }
    }

    fn draw(&mut self) {
        if let Some(pixels) = &mut self.pixels {
            let elapsed = self.start_time.elapsed().as_secs_f32();

            // Create animated content - moving gradient and rectangle
            let frame = pixels.frame_mut();

            // Animated background gradient
            let time_offset = (elapsed * 30.0) as i32;
            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                let x = (i % WIDTH as usize) as i32;
                let y = (i / WIDTH as usize) as i32;

                let r = ((x + time_offset) % 256) as u8;
                let g = ((y + time_offset / 2) % 256) as u8;
                let b = ((x + y + time_offset) % 256) as u8;

                pixel[0] = r;
                pixel[1] = g;
                pixel[2] = b;
                pixel[3] = 0xff;
            }

            // Moving white rectangle
            let rect_x = ((elapsed * 100.0) % WIDTH as f32) as i32;
            let rect_y = ((elapsed * 80.0) % HEIGHT as f32) as i32;
            let rect_width = 100;
            let rect_height = 80;

            for y in rect_y.max(0)..((rect_y + rect_height).min(HEIGHT as i32)) {
                for x in rect_x.max(0)..((rect_x + rect_width).min(WIDTH as i32)) {
                    let idx = ((y * WIDTH as i32 + x) * 4) as usize;
                    if idx + 3 < frame.len() {
                        frame[idx] = 255;
                        frame[idx + 1] = 255;
                        frame[idx + 2] = 255;
                        frame[idx + 3] = 255;
                    }
                }
            }

            if let Err(e) = pixels.render() {
                eprintln!("pixels.render() failed: {}", e);
            }
        }
    }
}

impl ApplicationHandler for AnimatedTestWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window_attributes = Window::default_attributes()
            .with_title("OBS Motion Test Window")
            .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
            .with_visible(true);

        let window = event_loop
            .create_window(window_attributes)
            .expect("Failed to create window");

        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)
            .expect("Failed to create pixels buffer");

        self.window = Some(window);
        self.pixels = Some(pixels);

        event_loop.set_control_flow(ControlFlow::Poll);
        
        println!("Test window created and visible");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Window close requested");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.draw();
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::Resized(size) => {
                if let Some(pixels) = &mut self.pixels {
                    if let Err(e) = pixels.resize_surface(size.width, size.height) {
                        eprintln!("pixels.resize_surface() failed: {}", e);
                    }
                }
            }
            _ => {}
        }

        // Check if we should close
        if self.should_close.load(Ordering::Relaxed) {
            println!("Closing test window");
            event_loop.exit();
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

#[tokio::test]
async fn test_motion_recording() -> Result<()> {
    println!("=== Starting motion recording test ===");

    let rec_file = ObsPath::from_relative("motion_test.mp4").build();
    let path_out = PathBuf::from(rec_file.to_string());

    // Clean up any existing file
    if path_out.exists() {
        std::fs::remove_file(&path_out)?;
    }

    // Shared flag to close the window
    let should_close = Arc::new(AtomicBool::new(false));
    let should_close_clone = should_close.clone();

    // Spawn the animated window in a separate thread
    let window_thread = thread::spawn(move || {
        println!("Starting animated window thread...");
        let event_loop = EventLoop::new().expect("Failed to create event loop");
        let mut app = AnimatedTestWindow::new(should_close_clone);
        event_loop.run_app(&mut app).expect("Event loop failed");
        println!("Window thread finished");
    });

    // Give the window time to appear and render some frames
    println!("Waiting for window to initialize...");
    tokio::time::sleep(Duration::from_secs(2)).await;

    println!("Setting up OBS recording...");

    // Start the OBS context
    let startup_info = StartupInfo::default().set_logger(Box::new(TestLogger {}));
    let mut context = ObsContext::new(startup_info)?;
    let mut scene = context.scene("main")?;

    // Create game capture source to capture the test window
    let mut video_source_data = context.data()?;
    video_source_data
        .bulk_update()
        .set_string("capture_mode", "window")
        .set_string("window", "OBS Motion Test Window")
        .set_bool("capture_cursor", false)
        .update()?;

    let video_source_info = SourceInfo::new(
        "game_capture",
        "motion_test_source",
        Some(video_source_data),
        None,
    );

    scene.add_source(video_source_info)?;
    scene.set_to_channel(0)?;

    // Set up output
    let mut output_settings = context.data()?;
    output_settings.set_string("path", rec_file)?;

    let output_info = OutputInfo::new("ffmpeg_muxer", "output", Some(output_settings), None);
    let mut output = context.output(output_info)?;

    // Set up video encoder
    let mut video_encoder = context.best_video_encoder()?;
    let mut video_settings = context.data()?;
    video_settings
        .bulk_update()
        .set_int("bf", 2)
        .set_bool("psycho_aq", true)
        .set_bool("lookahead", true)
        .set_string("profile", "high")
        .set_string("preset", "fast")
        .set_string("rate_control", "cbr")
        .set_int("bitrate", 10000)
        .update()?;

    video_encoder.set_settings(video_settings);
    video_encoder.set_to_output(&mut output, "video_encoder")?;

    // Set up audio encoder
    let mut audio_settings = context.data()?;
    audio_settings.set_int("bitrate", 160)?;

    let audio_info = AudioEncoderInfo::new("ffmpeg_aac", "audio_encoder", Some(audio_settings), None);
    let audio_handler = context.get_audio_ptr()?;
    output.audio_encoder(audio_info, 0, audio_handler)?;

    // Start recording
    println!("Starting recording for {} seconds...", RECORDING_DURATION_SECS);
    output.start()?;

    // Record for specified duration
    tokio::time::sleep(Duration::from_secs(RECORDING_DURATION_SECS)).await;

    // Stop recording
    println!("Stopping recording...");
    output.stop()?;

    // Signal window to close
    println!("Signaling window to close...");
    should_close.store(true, Ordering::Relaxed);

    // Wait for window thread to finish
    println!("Waiting for window thread to finish...");
    window_thread.join().expect("Window thread panicked");

    println!("Recording complete. Validating video...");

    // Wait a moment for file to be fully written
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Verify the video file exists
    assert!(path_out.exists(), "Recording file was not created");
    
    let metadata = std::fs::metadata(&path_out)?;
    println!("Recording file size: {} bytes", metadata.len());
    assert!(metadata.len() > 1000, "Recording file is too small");

    // Validate video content
    println!("Running ffprobe validation tests...");

    // Test 1: Video should not be black
    println!("Test 1: Checking video is not black...");
    common::test_video_not_black(&path_out, 1.0).await?;
    println!("✓ Video is not black");

    // Test 2: Video should have motion
    println!("Test 2: Checking video has motion...");
    common::test_video_has_motion(&path_out).await?;
    println!("✓ Video has motion");

    // Test 3: Check frame count
    println!("Test 3: Checking frame count...");
    let frame_count = common::get_frame_count(&path_out).await?;
    println!("Frame count: {}", frame_count);
    assert!(frame_count > 30, "Not enough frames recorded");
    println!("✓ Sufficient frames recorded");

    println!("=== Motion recording test passed! ===");

    // Clean up
    if path_out.exists() {
        std::fs::remove_file(&path_out)?;
    }

    Ok(())
}
