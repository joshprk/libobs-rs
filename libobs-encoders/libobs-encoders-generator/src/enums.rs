use num_derive::{FromPrimitive, ToPrimitive};

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum ObsPropertyType {
    Invalid = libobs::obs_property_type_OBS_PROPERTY_INVALID,
    Bool = libobs::obs_property_type_OBS_PROPERTY_BOOL,
    Int = libobs::obs_property_type_OBS_PROPERTY_INT,
    Float = libobs::obs_property_type_OBS_PROPERTY_FLOAT,
    Text = libobs::obs_property_type_OBS_PROPERTY_TEXT,
    Path = libobs::obs_property_type_OBS_PROPERTY_PATH,
    List = libobs::obs_property_type_OBS_PROPERTY_LIST,
    Color = libobs::obs_property_type_OBS_PROPERTY_COLOR,
    Button = libobs::obs_property_type_OBS_PROPERTY_BUTTON,
    Font = libobs::obs_property_type_OBS_PROPERTY_FONT,
    EditableList = libobs::obs_property_type_OBS_PROPERTY_EDITABLE_LIST,
    FrameRate = libobs::obs_property_type_OBS_PROPERTY_FRAME_RATE,
    Group = libobs::obs_property_type_OBS_PROPERTY_GROUP,
    ColorAlpha = libobs::obs_property_type_OBS_PROPERTY_COLOR_ALPHA,
}

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum ObsComboFormat {
    Invalid = libobs::obs_combo_format_OBS_COMBO_FORMAT_INVALID,
	Int = libobs::obs_combo_format_OBS_COMBO_FORMAT_INT,
	Float = libobs::obs_combo_format_OBS_COMBO_FORMAT_FLOAT,
	String = libobs::obs_combo_format_OBS_COMBO_FORMAT_STRING,
	Bool = libobs::obs_combo_format_OBS_COMBO_FORMAT_BOOL,
}