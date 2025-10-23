use std::{convert::Infallible, str::FromStr};

use crate::utils::ObsString;

macro_rules! encoder_enum {
    ($name:ident, [ $($(#[$attr:meta])* $variant:ident,)* ]) => { paste::paste! {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        #[allow(non_camel_case_types)]
        pub enum $name {
            $($(#[$attr])* [<$variant:upper>],)*
            Other(String),
        }

        impl FromStr for $name {
            type Err = Infallible;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                #[allow(deprecated)]
                return Ok(match s {
                    $( stringify!($variant) => Self::[<$variant:upper>], )*
                    e => Self::Other(e.to_string()),
                });
            }
        }
        impl Into<ObsString> for $name {
            fn into(self) -> ObsString {
                #[allow(deprecated)]
                return match self {
                    $( Self:: [<$variant:upper>] => ObsString::new(stringify!($variant)), )*
                    Self::Other(e) => ObsString::new(&e),
                };
            }
        }
    } };
}

// These lists were produced using
//   rg "struct obs_encoder_info [^\*].*=" -A 4
// on the OBS codebase, which lists all of the encoders and their types, as well as their
// capabilities (including whether they're deprecated or not).

encoder_enum!(
    ObsVideoEncoderType,
    [
        //
        // obs-ffmpeg
        //
        /// from `obs-ffmpeg`
        h264_texture_amf,
        /// from `obs-ffmpeg`
        h265_texture_amf,
        /// from `obs-ffmpeg`
        av1_texture_amf,
        /// from `obs-ffmpeg`
        ffmpeg_vaapi,
        /// from `obs-ffmpeg`
        ffmpeg_vaapi_tex,
        /// from `obs-ffmpeg`
        av1_ffmpeg_vaapi,
        /// from `obs-ffmpeg`
        av1_ffmpeg_vaapi_tex,
        /// from `obs-ffmpeg`
        hevc_ffmpeg_vaapi,
        /// from `obs-ffmpeg`
        hevc_ffmpeg_vaapi_tex,
        /// from `obs-ffmpeg`
        ffmpeg_openh264,
        /// from `obs-ffmpeg`
        #[deprecated]
        ffmpeg_nvenc,
        /// from `obs-ffmpeg`
        #[deprecated]
        ffmpeg_hevc_nvenc,
        /// from `obs-ffmpeg`
        ffmpeg_svt_av1,
        /// from `obs-ffmpeg`
        ffmpeg_aom_av1,
        //
        // obs-nvenc
        //
        /// from 'obs-nvenc'
        #[deprecated]
        obs_nvenc_h264_cuda,
        /// from 'obs-nvenc'
        #[deprecated]
        obs_nvenc_hevc_cuda,
        /// from 'obs-nvenc'
        #[deprecated]
        obs_nvenc_av1_cuda,
        /// from 'obs-nvenc'
        obs_nvenc_h264_tex,
        /// from 'obs-nvenc'
        obs_nvenc_hevc_tex,
        /// from 'obs-nvenc'
        obs_nvenc_av1_tex,
        /// from 'obs-nvenc'
        #[deprecated]
        jim_nvenc,
        /// from 'obs-nvenc'
        #[deprecated]
        jim_hevc_nvenc,
        /// from 'obs-nvenc'
        #[deprecated]
        jim_av1_nvenc,
        /// from 'obs-nvenc'
        obs_nvenc_h264_soft,
        /// from 'obs-nvenc'
        obs_nvenc_hevc_soft,
        /// from 'obs-nvenc'
        obs_nvenc_av1_soft,
        //
        // obs-qsv11
        //
        /// from 'obs-qsv11'
        obs_qsv11,
        /// from 'obs-qsv11'
        obs_qsv11_soft,
        /// from 'obs-qsv11'
        obs_qsv11_v2,
        /// from 'obs-qsv11'
        obs_qsv11_soft_v2,
        /// from 'obs-qsv11'
        obs_qsv11_av1,
        /// from 'obs-qsv11'
        obs_qsv11_av1_soft,
        /// from 'obs-qsv11'
        obs_qsv11_hevc,
        /// from 'obs-qsv11'
        obs_qsv11_hevc_soft,
        //
        // obs-x264
        //
        /// from 'obs-x264'
        obs_x264,
    ]
);

encoder_enum!(
    ObsAudioEncoderType,
    [
        //
        // obs-ffmpeg
        //
        /// from `obs-ffmpeg`
        ffmpeg_aac,
        /// from `obs-ffmpeg`
        ffmpeg_opus,
        /// from `obs-ffmpeg`
        ffmpeg_pcm_s16le,
        /// from `obs-ffmpeg`
        ffmpeg_pcm_s24le,
        /// from `obs-ffmpeg`
        ffmpeg_pcm_f32le,
        /// from `obs-ffmpeg`
        ffmpeg_alac,
        /// from `obs-ffmpeg`
        ffmpeg_flac,
        //
        // obs-libfdk
        //
        /// from 'obs-libfdk'
        libfdk_aac,
    ]
);
