use std::ffi::CStr;

use libobs::obs_get_encoder_properties;
use libobs_wrapper::{context::ObsContext, data::properties::{ObsComboFormat, ObsPropertyType}, encoders::ObsContextEncoders, utils::ObsString};
use num_traits::FromPrimitive;

fn main() -> anyhow::Result<()> {
    let _context = ObsContext::new(Default::default())?;

    for encoder in ObsContext::get_available_video_encoders() {
        let encoder_name: ObsString = encoder.into();
        let properties = unsafe { obs_get_encoder_properties(encoder_name.as_ptr()) };

        if properties.is_null() {
            println!(
                "Failed to get properties for encoder: {}",
                encoder_name.to_string()
            );
            continue;
        }

        println!("=========== {} ===========", encoder_name.to_string());

        let mut property = unsafe { libobs::obs_properties_first(properties) };
        while !property.is_null() {
            let name = unsafe { libobs::obs_property_name(property) };
            let name_str = unsafe { CStr::from_ptr(name as _) };
            let description = unsafe { libobs::obs_property_description(property) };
            let description_str = unsafe { CStr::from_ptr(description as _) };
            let type_ = unsafe { libobs::obs_property_get_type(property) };
            let type_ = ObsPropertyType::from_i32(type_).unwrap();

            if matches!(type_, ObsPropertyType::List) {
                let item_count = unsafe { libobs::obs_property_list_item_count(property) };
                let combo_format = unsafe { libobs::obs_property_list_format(property) };
                let combo_format = ObsComboFormat::from_i32(combo_format).unwrap();

                println!("Format is {:?}", combo_format);
                for i in 0..item_count {
                    let item_name = unsafe { libobs::obs_property_list_item_name(property, i) };
                    let item_name_str = unsafe { CStr::from_ptr(item_name as _) };

                    println!("  List Item: {}", item_name_str.to_string_lossy());
                }
            }

            println!(
                "Property: {} ({}), Type: {:?}",
                name_str.to_string_lossy(),
                description_str.to_string_lossy(),
                type_
            );

            // Move to the next property
            unsafe { libobs::obs_property_next(&mut property) };
        }

        unsafe { libobs::obs_properties_destroy(properties) };
        println!("=============================");
    }

    Ok(())
}
