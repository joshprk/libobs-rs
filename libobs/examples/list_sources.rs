// List all available OBS input sources
use std::ffi::{CStr, CString};
use std::{env::current_exe, ptr};

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
        
        // Get absolute paths
        let curr_exe = current_exe().unwrap();
        let curr_exe = curr_exe.parent().unwrap();
        
        // Try the .plugin bundle pattern
        let plugin_bin = curr_exe.join("../obs-plugins/%module%.plugin/Contents/MacOS");
        let plugin_data = curr_exe.join("../data/obs-plugins/%module%/");
        
        let plugin_bin_str = CString::new(plugin_bin.to_str().unwrap()).unwrap();
        let plugin_data_str = CString::new(plugin_data.to_str().unwrap()).unwrap();
        
        println!("Plugin bin path (with %%module%% pattern): {}", plugin_bin.display());
        println!("Plugin data path: {}", plugin_data.display());
        
        // Add module paths with .plugin bundle pattern
        libobs::obs_add_module_path(
            plugin_bin_str.as_ptr(),
            plugin_data_str.as_ptr(),
        );
        
        // Also add the root directory for .so files
        let plugin_root = curr_exe.join("../obs-plugins/");
        let plugin_root_str = CString::new(plugin_root.to_str().unwrap()).unwrap();
        let data_root_str = CString::new(curr_exe.join("../data/obs-plugins/%module%/").to_str().unwrap()).unwrap();
        libobs::obs_add_module_path(
            plugin_root_str.as_ptr(),
            data_root_str.as_ptr(),
        );
        
        // First, find modules to see what OBS is discovering
        println!("\n=== Module Discovery ===");
        extern "C" fn module_callback(_param: *mut std::ffi::c_void, info: *const libobs::obs_module_info2) {
            unsafe {
                if !info.is_null() {
                    let bin_path = CStr::from_ptr((*info).bin_path).to_str().unwrap_or("<invalid>");
                    let name = CStr::from_ptr((*info).name).to_str().unwrap_or("<invalid>");
                    println!("  Found: {} at {}", name, bin_path);
                }
            }
        }
        libobs::obs_find_modules2(Some(module_callback), ptr::null_mut());
        
        // Load modules
        println!("\nLoading modules...");
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

