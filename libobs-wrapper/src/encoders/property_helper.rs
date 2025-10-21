use std::{collections::HashMap, str::FromStr, sync::Arc};

use duplicate::duplicate_item;

use crate::{
    context::ObsContext,
    data::{
        output::ObsOutputRef,
        properties::{
            get_properties_inner, ObsProperty, ObsPropertyObject, ObsPropertyObjectPrivate,
        },
        ObsData,
    },
    run_with_obs,
    runtime::ObsRuntime,
    unsafe_send::Sendable,
    utils::{ObjectInfo, ObsError, ObsString},
};

use super::{audio::ObsAudioEncoder, video::ObsVideoEncoder, ObsAudioEncoderType, ObsVideoEncoderType};

#[duplicate_item(
    StructName EncoderType;
    [ObsAudioEncoderBuilder] [ObsAudioEncoderType];
    [ObsVideoEncoderBuilder] [ObsVideoEncoderType]
)]
#[derive(Debug)]
pub struct StructName {
    encoder_id: EncoderType,
    runtime: ObsRuntime,
    context: ObsContext,
    settings: Option<ObsData>,
    hotkey_data: Option<ObsData>,
}

#[duplicate_item(
    StructName EncoderType;
    [ObsAudioEncoderBuilder] [ObsAudioEncoderType];
    [ObsVideoEncoderBuilder] [ObsVideoEncoderType]
)]
impl StructName {
    pub fn new(context: ObsContext, encoder_id: &str) -> Self {
        Self {
            encoder_id: EncoderType::from_str(encoder_id).unwrap(),
            runtime: context.runtime().clone(),
            context,
            settings: None,
            hotkey_data: None
        }
    }

    pub fn get_encoder_id(&self) -> &EncoderType {
        &self.encoder_id
    }

    pub fn set_settings(&mut self, settings: ObsData) -> &mut Self {
        self.settings = Some(settings);
        self
    }

    pub fn set_hotkey_data(&mut self, hotkey_data: ObsData) -> &mut Self {
        self.hotkey_data = Some(hotkey_data);
        self
    }

    pub fn get_settings(&self) -> Option<&ObsData> {
        self.settings.as_ref()
    }

    pub fn get_hotkey_data(&self) -> Option<&ObsData> {
        self.hotkey_data.as_ref()
    }

    pub fn get_settings_mut(&mut self) -> Option<&mut ObsData> {
        self.settings.as_mut()
    }

    pub fn get_hotkey_data_mut(&mut self) -> Option<&mut ObsData> {
        self.hotkey_data.as_mut()
    }
}

impl ObsAudioEncoderBuilder {
    pub fn apply_to_context(
        self,
        output: &mut ObsOutputRef,
        name: &str,
        settings: Option<ObsData>,
        hotkey_data: Option<ObsData>,
        mixer_idx: usize,
    ) -> Result<Arc<ObsAudioEncoder>, ObsError> {
        let e_id: ObsString = self.encoder_id.into();
        let info = ObjectInfo::new(
            e_id,
            ObsString::new(name),
            settings,
            hotkey_data,
        );

        let audio_handler = self.context.get_audio_ptr()?;
        output.audio_encoder(info, mixer_idx, audio_handler)
    }
}


impl ObsVideoEncoderBuilder {
    pub fn set_to_output(
        self,
        output: &mut ObsOutputRef,
        name: &str
    ) -> Result<Arc<ObsVideoEncoder>, ObsError> {
        let e_id: ObsString = self.encoder_id.into();
        let info = ObjectInfo::new(
            e_id,
            ObsString::new(name),
            self.settings,
            self.hotkey_data,
        );

        let video_handler = self.context.get_video_ptr()?;
        output.video_encoder(info, video_handler)
    }
}

#[duplicate_item(
    StructName;
    [ObsAudioEncoderBuilder];
    [ObsVideoEncoderBuilder]
)]
impl ObsPropertyObject for StructName {
    fn get_properties(&self) -> Result<HashMap<String, ObsProperty>, ObsError> {
        let properties_raw = self.get_properties_raw()?;
        get_properties_inner(properties_raw, self.runtime.clone())
    }
}

#[duplicate_item(
    StructName;
    [ObsAudioEncoderBuilder];
    [ObsVideoEncoderBuilder]
)]
impl ObsPropertyObjectPrivate for StructName {
    fn get_properties_raw(
        &self,
    ) -> Result<Sendable<*mut libobs::obs_properties_t>, ObsError> {
        let encoder_name: ObsString = self.encoder_id.clone().into();
        let encoder_name_ptr = encoder_name.as_ptr();

        run_with_obs!(self.runtime, (encoder_name_ptr), move || unsafe {
            Sendable(libobs::obs_get_encoder_properties(encoder_name_ptr))
        })
    }

    fn get_properties_by_id_raw<T: Into<ObsString> + Sync + Send>(
        id: T,
        runtime: ObsRuntime,
    ) -> Result<Sendable<*mut libobs::obs_properties_t>, ObsError> {
        let id: ObsString = id.into();
        let id_ptr = id.as_ptr();
        run_with_obs!(runtime, (id_ptr), move || unsafe {
            Sendable(libobs::obs_get_encoder_properties(id_ptr))
        })
    }
}
