// List screen_capture source properties to understand what settings are available
use std::ffi::{CStr, CString};
use std::{env::current_exe, ptr};

fn main() {
    unsafe {
        libobs::obs_startup("en-US\0".as_ptr() as *const i8, ptr::null(), ptr::null_mut());
        
        let curr_exe = current_exe().unwrap();
        let curr_exe = curr_exe.parent().unwrap();
        
        let module_bin = CString::new(curr_exe.join("../obs-plugins/%module%.plugin/Contents/MacOS").to_str().unwrap()).unwrap();
        let module_data = CString::new(curr_exe.join("../data/obs-plugins/%module%/").to_str().unwrap()).unwrap();
        libobs::obs_add_module_path(module_bin.as_ptr(), module_data.as_ptr());
        
        let mut audio_info: libobs::obs_audio_info = std::mem::zeroed();
        audio_info.samples_per_sec = 44100;
        audio_info.speakers = libobs::speaker_layout_SPEAKERS_STEREO;
        libobs::obs_reset_audio(&audio_info);
        
        libobs::obs_load_all_modules();
        libobs::obs_post_load_modules();
        
        // Get source properties
        let source_id = CString::new("screen_capture").unwrap();
        let properties = libobs::obs_get_source_properties(source_id.as_ptr());
        
        if properties.is_null() {
            println!("✗ Could not get properties for screen_capture");
        } else {
            println!("=== screen_capture Properties ===\n");
            
            let mut prop = libobs::obs_properties_first(properties);
            while !prop.is_null() {
                let name_ptr = libobs::obs_property_name(prop);
                let desc_ptr = libobs::obs_property_description(prop);
                let prop_type = libobs::obs_property_get_type(prop);
                
                let name = if !name_ptr.is_null() {
                    CStr::from_ptr(name_ptr).to_str().unwrap_or("<invalid>")
                } else {
                    "<null>"
                };
                
                let desc = if !desc_ptr.is_null() {
                    CStr::from_ptr(desc_ptr).to_str().unwrap_or("<invalid>")
                } else {
                    "<null>"
                };
                
                println!("  {} ({}) - Type: {}", name, desc, prop_type);
                
                // If it's a list, show the options
                if prop_type == libobs::obs_property_type_OBS_PROPERTY_LIST {
                    let count = libobs::obs_property_list_item_count(prop);
                    for i in 0..count {
                        let item_name = libobs::obs_property_list_item_name(prop, i);
                        if !item_name.is_null() {
                            let item_str = CStr::from_ptr(item_name).to_str().unwrap_or("<invalid>");
                            println!("      - Option {}: {}", i, item_str);
                        }
                    }
                }
                
                if !libobs::obs_property_next(&mut prop) {
                    break;
                }
            }
            
            libobs::obs_properties_destroy(properties);
        }
        
        libobs::obs_shutdown();
    }
}

