
macro_rules! define_object_builder {
    ($struct_name:ident($obs_id:literal) for $updatable_name: ident, $($field_name:ident: $field_type:ty, $obs_property:meta),*) => {
        #[allow(dead_code)]
        /// This struct is just so the compiler isn't confused
        struct $struct_name {}

        paste::paste! {
            #[derive(Debug)]
            #[libobs_source_macro::obs_object_builder($obs_id)]
            pub struct [<$struct_name Builder>] {
                $(
                    #[$obs_property]
                    $field_name: $field_type,
                )*
            }

            #[libobs_source_macro::obs_object_updater($obs_id, $updatable_name)]
            pub struct [<$struct_name Updater>] {
                $(
                    #[$obs_property]
                    $field_name: $field_type,
                )*
            }
        }
    };
}

pub(crate) use define_object_builder;