use libobs::{obs_audio_info, obs_audio_info2};

use crate::enums::{ObsSamplesPerSecond, ObsSpeakerLayout};



/// Information passed to libobs when attempting to
/// reset the audio context using `obs_reset_audio`.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObsAudioInfo {
    samples_per_sec: ObsSamplesPerSecond,
    speakers: ObsSpeakerLayout,
}

impl ObsAudioInfo {
    pub fn new(samples_per_second: ObsSamplesPerSecond, speakers: ObsSpeakerLayout) -> Self {
        Self {
            samples_per_sec: samples_per_second,
            speakers,
        }
    }

    pub fn as_ptr(&self) -> *const obs_audio_info {
        self as *const Self as *const obs_audio_info
    }
}

impl Default for ObsAudioInfo {
    fn default() -> Self {
        Self {
            samples_per_sec: ObsSamplesPerSecond::F44100,
            speakers: ObsSpeakerLayout::Stereo,
        }
    }
}



/// Information passed to libobs when attempting to
/// reset the audio context using the newer, more
/// detailed function `obs_reset_audio2`.
pub type ObsAudioInfo2 = obs_audio_info2;