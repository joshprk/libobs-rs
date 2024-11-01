use crate::{data::{audio::ObsAudioInfo, video::ObsVideoInfo}, logger::{ConsoleLogger, ObsLogger}, utils::{ObsPath, ObsString}};



/// Contains information to start a libobs context.
/// This is passed to the creation of `ObsContext`.
#[derive(Debug)]
pub struct StartupInfo {
    pub(crate) startup_paths: StartupPaths,
    pub(crate) obs_video_info: ObsVideoInfo,
    pub(crate) obs_audio_info: ObsAudioInfo,
    // Option because logger is taken when creating
    pub(crate) logger: Option<Box<dyn ObsLogger>>
}

impl StartupInfo {
    pub fn new() -> StartupInfo {
        Self::default()
    }

    pub fn set_startup_paths(mut self, paths: StartupPaths) -> Self {
        self.startup_paths = paths;
        self
    }

    pub fn set_video_info(mut self, ovi: ObsVideoInfo) -> Self {
        self.obs_video_info = ovi;
        self
    }

    pub fn set_logger(mut self, logger: Box<dyn ObsLogger>) -> Self {
        self.logger = Some(logger);
        self
    }
}

impl Default for StartupInfo {
    fn default() -> StartupInfo {
        Self {
            startup_paths: StartupPaths::default(),
            obs_video_info: ObsVideoInfo::default(),
            obs_audio_info: ObsAudioInfo::default(),
            logger: Some(Box::new(ConsoleLogger::new())),
        }
    }
}

/// Contains the necessary paths for starting the
/// libobs context built from `ObsPath`.
///
/// Note that these strings are copied when parsed,
/// meaning that these can be freed immediately
/// after all three strings have been used.
#[derive(Clone, Debug)]
pub struct StartupPaths {
    libobs_data_path: ObsString,
    plugin_bin_path: ObsString,
    plugin_data_path: ObsString,
}

impl StartupPaths {
    pub fn new(
        libobs_data_path: ObsPath,
        plugin_bin_path: ObsPath,
        plugin_data_path: ObsPath,
    ) -> StartupPaths {
        Self {
            libobs_data_path: libobs_data_path.build(),
            plugin_bin_path: plugin_bin_path.build(),
            plugin_data_path: plugin_data_path.build(),
        }
    }

    pub fn libobs_data_path(&self) -> &ObsString {
        &(self.libobs_data_path)
    }

    pub fn plugin_bin_path(&self) -> &ObsString {
        &(self.plugin_bin_path)
    }

    pub fn plugin_data_path(&self) -> &ObsString {
        &(self.plugin_data_path)
    }
}

impl Default for StartupPaths {
    fn default() -> Self {
        StartupPathsBuilder::new().build()
    }
}

#[derive(Clone, Debug)]
pub struct StartupPathsBuilder {
    libobs_data_path: ObsPath,
    plugin_bin_path: ObsPath,
    plugin_data_path: ObsPath,
}

impl StartupPathsBuilder {
    fn new() -> Self {
        Self {
            libobs_data_path: ObsPath::from_relative("data/libobs"),
            plugin_bin_path: ObsPath::from_relative("obs-plugins/64bit"),
            plugin_data_path: ObsPath::from_relative("data/obs-plugins/%module%"),
        }
    }

    pub fn build(self) -> StartupPaths {
        StartupPaths {
            libobs_data_path: self.libobs_data_path.build(),
            plugin_bin_path: self.plugin_bin_path.build(),
            plugin_data_path: self.plugin_data_path.build(),
        }
    }

    pub fn libobs_data_path(mut self, value: ObsPath) -> Self {
        self.libobs_data_path = value;
        self
    }

    pub fn plugin_bin_path(mut self, value: ObsPath) -> Self {
        self.plugin_bin_path = value;
        self
    }

    pub fn plugin_data_path(mut self, value: ObsPath) -> Self {
        self.plugin_data_path = value;
        self
    }
}

impl Default for StartupPathsBuilder {
    fn default() -> StartupPathsBuilder {
        Self::new()
    }
}