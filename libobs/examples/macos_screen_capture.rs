// macOS screen capture example using OBS's screen_capture source
use std::ffi::CString;
use std::{env::current_exe, ptr, thread, time::Duration};

fn main() {
    unsafe {
        println!("=== macOS Screen Capture Example ===\n");
        
        // Initialize OBS
        let locale = CString::new("en-US").unwrap();
        libobs::obs_startup(locale.as_ptr(), ptr::null(), ptr::null_mut());
        
        let curr_exe = current_exe().unwrap();
        let curr_exe = curr_exe.parent().unwrap();
        
        // Add module paths with .plugin bundle pattern
        let module_bin = CString::new(curr_exe.join("../obs-plugins/%module%.plugin/Contents/MacOS").to_str().unwrap()).unwrap();
        let module_data = CString::new(curr_exe.join("../data/obs-plugins/%module%/").to_str().unwrap()).unwrap();
        libobs::obs_add_module_path(module_bin.as_ptr(), module_data.as_ptr());
        
        // Add data path for libobs
        let data_path = CString::new(curr_exe.join("../data/libobs/").to_str().unwrap()).unwrap();
        libobs::obs_add_data_path(data_path.as_ptr());
        
        // Reset audio
        let mut audio_info: libobs::obs_audio_info = std::mem::zeroed();
        audio_info.samples_per_sec = 44100;
        audio_info.speakers = libobs::speaker_layout_SPEAKERS_STEREO;
        libobs::obs_reset_audio(&audio_info);
        
        // Reset video (required for graphics)
        let mut video_info: libobs::obs_video_info = std::mem::zeroed();
        video_info.fps_num = 30;
        video_info.fps_den = 1;
        video_info.graphics_module = "libobs-opengl.so\0".as_ptr() as *const i8;
        video_info.base_width = 1920;
        video_info.base_height = 1080;
        video_info.output_width = 1920;
        video_info.output_height = 1080;
        video_info.output_format = libobs::video_format_VIDEO_FORMAT_NV12;
        video_info.adapter = 0;
        video_info.gpu_conversion = true;
        video_info.colorspace = libobs::video_colorspace_VIDEO_CS_709;
        video_info.range = libobs::video_range_type_VIDEO_RANGE_PARTIAL;
        video_info.scale_type = libobs::obs_scale_type_OBS_SCALE_BILINEAR;
        
        let video_result = libobs::obs_reset_video(&mut video_info);
        if video_result != 0 {
            println!("✗ Failed to reset video: code {}", video_result);
            libobs::obs_shutdown();
            return;
        }
        println!("✓ Video initialized");
        
        // Load modules
        println!("Loading modules...");
        libobs::obs_load_all_modules();
        libobs::obs_post_load_modules();
        libobs::obs_log_loaded_modules();
        
        // Create a scene
        let scene_name = CString::new("Main Scene").unwrap();
        let scene = libobs::obs_scene_create(scene_name.as_ptr());
        if scene.is_null() {
            println!("✗ Failed to create scene");
            libobs::obs_shutdown();
            return;
        }
        println!("✓ Scene created");
        
        // Create screen capture source
        let source_name = CString::new("Screen Capture").unwrap();
        let source_id = CString::new("screen_capture").unwrap();
        
        // Create settings (use NULL/default settings initially)
        let settings = libobs::obs_data_create();
        
        let source = libobs::obs_source_create(
            source_id.as_ptr(),
            source_name.as_ptr(),
            settings,
            ptr::null_mut(),
        );
        
        libobs::obs_data_release(settings);
        
        if source.is_null() {
            println!("✗ Failed to create screen capture source");
            libobs::obs_scene_release(scene);
            libobs::obs_shutdown();
            return;
        }
        println!("✓ Screen capture source created");
        
        // Add source to scene
        let scene_item = libobs::obs_scene_add(scene, source);
        if scene_item.is_null() {
            println!("✗ Failed to add source to scene");
        } else {
            println!("✓ Source added to scene");
        }
        
        println!("\n✓ Screen capture setup complete!");
        println!("  Source is now capturing the main display");
        println!("  (In a real app, you would connect this to an output/encoder)");
        
        // Keep running for a bit to allow capture to initialize
        thread::sleep(Duration::from_secs(2));
        
        // Cleanup - proper order to avoid segfaults
        // 1. Get and release scene source
        let scene_source = libobs::obs_scene_get_source(scene);
        libobs::obs_source_release(scene_source);
        
        // 2. Release scene (which will release scene items)
        libobs::obs_scene_release(scene);
        
        // 3. Release capture source
        libobs::obs_source_release(source);
        
        // 4. Clear output channels
        libobs::obs_set_output_source(0, ptr::null_mut());
        libobs::obs_set_output_source(1, ptr::null_mut());
        
        // 5. Shutdown OBS
        libobs::obs_shutdown();
        
        println!("\n✓ Example completed successfully!");
    }
}

