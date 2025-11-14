// List all available OBS input sources
use std::ffi::CStr;
use std::ptr;

fn main() {
    unsafe {
        // Initialize OBS
        libobs::obs_startup("en-US\0".as_ptr() as *const i8, ptr::null(), ptr::null_mut());
        
        println!("macOS detected - loading modules");
        
        // Reset audio
        let mut audio_info: libobs::obs_audio_info = std::mem::zeroed();
        audio_info.samples_per_sec = 44100;
        audio_info.speakers = libobs::speaker_layout_SPEAKERS_STEREO;
        libobs::obs_reset_audio(&audio_info);
        
        // Add module paths (relative to examples directory, go up one level)
        libobs::obs_add_module_path(
            "../obs-plugins/\0".as_ptr() as *const i8,
            "../data/obs-plugins/%module%/\0".as_ptr() as *const i8,
        );
        
        // Load modules
        println!("Loading modules...");
        libobs::obs_load_all_modules();
        libobs::obs_post_load_modules();
        
        // Log loaded modules
        libobs::obs_log_loaded_modules();
        
        println!("\n=== Available Input Sources ===\n");
        let mut idx = 0;
        loop {
            let mut source_id_ptr: *const i8 = ptr::null();
            let found = libobs::obs_enum_input_types(idx, &mut source_id_ptr as *mut *const i8);
            if !found || source_id_ptr.is_null() {
                break;
            }
            
            let id_cstr = CStr::from_ptr(source_id_ptr);
            let id_str = id_cstr.to_str().unwrap_or("<invalid>");
            
            // Get capabilities
            let caps = libobs::obs_get_source_output_flags(source_id_ptr);
            let is_video = (caps & libobs::OBS_SOURCE_VIDEO) != 0;
            let is_audio = (caps & libobs::OBS_SOURCE_AUDIO) != 0;
            
            if id_str.contains("mac") || id_str.contains("screen") || id_str.contains("display") || id_str.contains("capture") {
                println!("  • {} [video: {}, audio: {}]", id_str, is_video, is_audio);
            }
            
            idx += 1;
        }
        
        libobs::obs_shutdown();
    }
}

