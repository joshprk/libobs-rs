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

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum ObsComboType {
    Invalid = libobs::obs_combo_type_OBS_COMBO_TYPE_INVALID,
    Editable = libobs::obs_combo_type_OBS_COMBO_TYPE_EDITABLE,
    List = libobs::obs_combo_type_OBS_COMBO_TYPE_LIST,
    Radio = libobs::obs_combo_type_OBS_COMBO_TYPE_RADIO,
}

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum ObsTextType {
    Default = libobs::obs_text_type_OBS_TEXT_DEFAULT,
    Password = libobs::obs_text_type_OBS_TEXT_PASSWORD,
    Multiline = libobs::obs_text_type_OBS_TEXT_MULTILINE,
    Info = libobs::obs_text_type_OBS_TEXT_INFO,
}

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum ObsTextInfoType {
    Normal = libobs::obs_text_info_type_OBS_TEXT_INFO_NORMAL,
    Warning = libobs::obs_text_info_type_OBS_TEXT_INFO_WARNING,
    Error = libobs::obs_text_info_type_OBS_TEXT_INFO_ERROR
}

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum ObsNumberType {
    Scroller = libobs::obs_number_type_OBS_NUMBER_SCROLLER,
    Slider = libobs::obs_number_type_OBS_NUMBER_SLIDER,
}

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum ObsPathType {
    File = libobs::obs_path_type_OBS_PATH_FILE,
    FileSave = libobs::obs_path_type_OBS_PATH_FILE_SAVE,
    Directory = libobs::obs_path_type_OBS_PATH_DIRECTORY,
}

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum ObsEditableListType {
    Strings = libobs::obs_editable_list_type_OBS_EDITABLE_LIST_TYPE_STRINGS,
    Files = libobs::obs_editable_list_type_OBS_EDITABLE_LIST_TYPE_FILES,
    FilesAndUrls = libobs::obs_editable_list_type_OBS_EDITABLE_LIST_TYPE_FILES_AND_URLS,
}

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum ObsGroupType {
    Invalid = libobs::obs_group_type_OBS_COMBO_INVALID,
    Normal = libobs::obs_group_type_OBS_GROUP_NORMAL,
    Checkable = libobs::obs_group_type_OBS_GROUP_CHECKABLE,
}

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum ObsButtonType {
    Default = libobs::obs_button_type_OBS_BUTTON_DEFAULT,
    Url = libobs::obs_button_type_OBS_BUTTON_URL,
}