#[macro_export]
macro_rules! define_object_manager {
    ($(#[$parent_meta:meta])* struct $struct_name:ident($obs_id:literal) for $updatable_name:ident {
        $(
            $(#[$meta:meta])*
            $field:ident: $ty:ty,
        )*
    }) => {
        paste::paste! {
            #[libobs_simple_macro::obs_object_builder($obs_id)]
            $(#[$parent_meta])*
            pub struct [<$struct_name Builder>] {
                $(
                    $(#[$meta])*
                    $field: $ty,
                )*
            }

            #[libobs_simple_macro::obs_object_updater($obs_id, $updatable_name)]
            /// Used to update the source this updater was created from. For more details look
            /// at docs for the corresponding builder.
            pub struct [<$struct_name Updater>] {
                $(
                    $(#[$meta])*
                    $field: $ty,
                )*
            }
        }
    };
}
