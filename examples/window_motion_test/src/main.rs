use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct AnimatedWindow {
    window: Option<Window>,
    pixels: Option<Pixels>,
    start_time: Instant,
}

impl AnimatedWindow {
    fn new() -> Self {
        Self {
            window: None,
            pixels: None,
            start_time: Instant::now(),
        }
    }

    fn draw(&mut self) {
        if let Some(pixels) = &mut self.pixels {
            let elapsed = self.start_time.elapsed().as_secs_f32();
            
            // Create a simple animation - a moving gradient and rectangle
            let frame = pixels.frame_mut();
            
            // Background gradient that shifts over time
            let time_offset = (elapsed * 30.0) as i32;
            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                let x = (i % WIDTH as usize) as i32;
                let y = (i / WIDTH as usize) as i32;
                
                // Shifting gradient
                let r = ((x + time_offset) % 256) as u8;
                let g = ((y + time_offset / 2) % 256) as u8;
                let b = ((x + y + time_offset) % 256) as u8;
                
                pixel[0] = r; // R
                pixel[1] = g; // G
                pixel[2] = b; // B
                pixel[3] = 0xff; // A
            }
            
            // Moving rectangle
            let rect_x = ((elapsed * 100.0) % WIDTH as f32) as i32;
            let rect_y = ((elapsed * 80.0) % HEIGHT as f32) as i32;
            let rect_width = 100;
            let rect_height = 80;
            
            for y in rect_y.max(0)..((rect_y + rect_height).min(HEIGHT as i32)) {
                for x in rect_x.max(0)..((rect_x + rect_width).min(WIDTH as i32)) {
                    let idx = ((y * WIDTH as i32 + x) * 4) as usize;
                    if idx + 3 < frame.len() {
                        frame[idx] = 255;     // R
                        frame[idx + 1] = 255; // G
                        frame[idx + 2] = 255; // B
                        frame[idx + 3] = 255; // A
                    }
                }
            }
            
            if let Err(e) = pixels.render() {
                eprintln!("pixels.render() failed: {}", e);
            }
        }
    }
}

impl ApplicationHandler for AnimatedWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window_attributes = Window::default_attributes()
            .with_title("OBS Motion Test Window")
            .with_inner_size(LogicalSize::new(WIDTH, HEIGHT));

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
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn main() -> Result<()> {
    println!("Starting OBS Motion Test Window...");
    
    let event_loop = EventLoop::new()?;
    let mut app = AnimatedWindow::new();
    
    event_loop.run_app(&mut app)?;
    
    Ok(())
}
