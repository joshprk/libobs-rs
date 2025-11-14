// Complete macOS screen recording example - records 5 seconds to desktop
use std::ffi::{CStr, CString};
use std::{env::current_exe, ptr, thread, time::Duration};

fn main() {
    unsafe {
        println!("=== macOS Screen Recording Test ===\n");
        
        // Initialize OBS
        let locale = CString::new("en-US").unwrap();
        libobs::obs_startup(locale.as_ptr(), ptr::null(), ptr::null_mut());
        
        let curr_exe = current_exe().unwrap();
        let curr_exe = curr_exe.parent().unwrap();
        
        // Setup paths
        let module_bin = CString::new(curr_exe.join("../obs-plugins/%module%.plugin/Contents/MacOS").to_str().unwrap()).unwrap();
        let module_data = CString::new(curr_exe.join("../data/obs-plugins/%module%/").to_str().unwrap()).unwrap();
        libobs::obs_add_module_path(module_bin.as_ptr(), module_data.as_ptr());
        
        let data_path = CString::new(curr_exe.join("../data/libobs/").to_str().unwrap()).unwrap();
        libobs::obs_add_data_path(data_path.as_ptr());
        
        // Reset audio
        let mut audio_info: libobs::obs_audio_info = std::mem::zeroed();
        audio_info.samples_per_sec = 44100;
        audio_info.speakers = libobs::speaker_layout_SPEAKERS_STEREO;
        libobs::obs_reset_audio(&audio_info);
        
        // Reset video
        let mut video_info: libobs::obs_video_info = std::mem::zeroed();
        video_info.fps_num = 30;
        video_info.fps_den = 1;
        video_info.graphics_module = "libobs-opengl.so\0".as_ptr() as *const i8;
        video_info.base_width = 3840;
        video_info.base_height = 2160;
        video_info.output_width = 1920;
        video_info.output_height = 1080;
        video_info.output_format = libobs::video_format_VIDEO_FORMAT_NV12;
        video_info.adapter = 0;
        video_info.gpu_conversion = true;
        video_info.colorspace = libobs::video_colorspace_VIDEO_CS_709;
        video_info.range = libobs::video_range_type_VIDEO_RANGE_PARTIAL;
        video_info.scale_type = libobs::obs_scale_type_OBS_SCALE_BILINEAR;
        
        if libobs::obs_reset_video(&mut video_info) != 0 {
            println!("✗ Failed to initialize video");
            libobs::obs_shutdown();
            return;
        }
        println!("✓ Video initialized (1920x1080 @ 30fps)");
        
        // Load modules
        libobs::obs_load_all_modules();
        libobs::obs_post_load_modules();
        println!("✓ Modules loaded");
        
        // Create scene
        let scene_name = CString::new("Recording Scene").unwrap();
        let scene = libobs::obs_scene_create(scene_name.as_ptr());
        let scene_source = libobs::obs_scene_get_source(scene);
        
        // Create screen capture source
        let source_name = CString::new("Screen Capture").unwrap();
        let source_id = CString::new("screen_capture").unwrap();
        let settings = libobs::obs_data_create();
        libobs::obs_data_set_int(settings, "display\0".as_ptr() as *const i8, 0); // Main display
        libobs::obs_data_set_bool(settings, "show_cursor\0".as_ptr() as *const i8, true);
        
        let capture_source = libobs::obs_source_create(
            source_id.as_ptr(),
            source_name.as_ptr(),
            settings,
            ptr::null_mut(),
        );
        libobs::obs_data_release(settings);
        
        if capture_source.is_null() {
            println!("✗ Failed to create screen capture source");
            libobs::obs_scene_release(scene);
            libobs::obs_shutdown();
            return;
        }
        println!("✓ Screen capture source created");
        
        // Add to scene
        libobs::obs_scene_add(scene, capture_source);
        
        // Set as video output
        libobs::obs_set_output_source(0, scene_source);
        
        // Wait for screen capture to initialize (async)
        println!("Waiting for capture source to initialize...");
        thread::sleep(Duration::from_millis(1000));
        
        // Check if source is showing
        let width = libobs::obs_source_get_width(capture_source);
        let height = libobs::obs_source_get_height(capture_source);
        println!("Source dimensions: {}x{}", width, height);
        
        if width == 0 || height == 0 {
            println!("✗ Screen capture not initialized - check permissions");
            libobs::obs_source_release(capture_source);
            libobs::obs_scene_release(scene);
            libobs::obs_shutdown();
            return;
        }
        
        // Create H.264 encoder
        let encoder_id = CString::new("obs_x264").unwrap();
        let encoder_name = CString::new("h264_encoder").unwrap();
        let encoder_settings = libobs::obs_data_create();
        libobs::obs_data_set_string(encoder_settings, "rate_control\0".as_ptr() as *const i8, "CBR\0".as_ptr() as *const i8);
        libobs::obs_data_set_int(encoder_settings, "bitrate\0".as_ptr() as *const i8, 2500);
        libobs::obs_data_set_string(encoder_settings, "preset\0".as_ptr() as *const i8, "veryfast\0".as_ptr() as *const i8);
        
        let encoder = libobs::obs_video_encoder_create(
            encoder_id.as_ptr(),
            encoder_name.as_ptr(),
            encoder_settings,
            ptr::null_mut(),
        );
        
        // Update encoder with current video settings (before releasing)
        libobs::obs_encoder_update(encoder, encoder_settings);
        libobs::obs_data_release(encoder_settings);
        
        if encoder.is_null() {
            println!("✗ Failed to create encoder");
            libobs::obs_source_release(capture_source);
            libobs::obs_scene_release(scene);
            libobs::obs_shutdown();
            return;
        }
        println!("✓ H.264 encoder created (2500 kbps)");
        
        // Create output to file
        let output_id = CString::new("ffmpeg_muxer").unwrap();
        let output_name = CString::new("file_output").unwrap();
        let output_settings = libobs::obs_data_create();
        
        // Save to desktop
        let desktop_path = std::env::var("HOME").unwrap() + "/Desktop/obs_test_recording.mp4";
        let file_path = CString::new(desktop_path.clone()).unwrap();
        libobs::obs_data_set_string(output_settings, "path\0".as_ptr() as *const i8, file_path.as_ptr());
        
        let output = libobs::obs_output_create(
            output_id.as_ptr(),
            output_name.as_ptr(),
            output_settings,
            ptr::null_mut(),
        );
        libobs::obs_data_release(output_settings);
        
        if output.is_null() {
            println!("✗ Failed to create output");
            libobs::obs_encoder_release(encoder);
            libobs::obs_source_release(capture_source);
            libobs::obs_scene_release(scene);
            libobs::obs_shutdown();
            return;
        }
        println!("✓ Output created: {}", desktop_path);
        
        // Create audio encoder (required for ffmpeg_muxer)
        let audio_encoder_id = CString::new("ffmpeg_aac").unwrap();
        let audio_encoder_name = CString::new("aac_encoder").unwrap();
        let audio_encoder = libobs::obs_audio_encoder_create(
            audio_encoder_id.as_ptr(),
            audio_encoder_name.as_ptr(),
            ptr::null_mut(),
            0,
            ptr::null_mut(),
        );
        libobs::obs_encoder_set_audio(audio_encoder, libobs::obs_get_audio());
        println!("✓ AAC audio encoder created");
        
        // Connect encoders to output
        libobs::obs_encoder_set_video(encoder, libobs::obs_get_video());
        libobs::obs_output_set_video_encoder(output, encoder);
        libobs::obs_output_set_audio_encoder(output, audio_encoder, 0);
        
        // Start recording
        println!("\n🔴 Starting recording...");
        if !libobs::obs_output_start(output) {
            println!("✗ Failed to start output");
            let error = libobs::obs_output_get_last_error(output);
            if !error.is_null() {
                let error_str = CStr::from_ptr(error).to_str().unwrap_or("<invalid>");
                println!("  Error: {}", error_str);
            }
        } else {
            println!("✓ Recording started!");
            println!("  Recording for 5 seconds...");
            
            // Record for 5 seconds
            for i in 1..=5 {
                thread::sleep(Duration::from_secs(1));
                print!("  {}... ", i);
                std::io::Write::flush(&mut std::io::stdout()).ok();
            }
            println!("\n");
            
            // Stop recording
            libobs::obs_output_stop(output);
            
            // Wait for output to finish
            thread::sleep(Duration::from_millis(500));
            
            println!("⏹️  Recording stopped");
        }
        
        // Cleanup
        libobs::obs_output_release(output);
        libobs::obs_encoder_release(encoder);
        libobs::obs_encoder_release(audio_encoder);
        libobs::obs_source_release(capture_source);
        libobs::obs_source_release(scene_source);
        libobs::obs_scene_release(scene);
        libobs::obs_set_output_source(0, ptr::null_mut());
        libobs::obs_set_output_source(1, ptr::null_mut());
        libobs::obs_shutdown();
        
        println!("\n✅ Recording complete!");
        println!("📁 Check: {}", desktop_path);
    }
}

