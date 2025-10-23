use libobs_source_macro::obs_object_builder;

macro_rules! new_output_builder {
    ($builder:ident, $output_type:literal) => {
        #[obs_object_builder($output_type)]
        pub struct $builder {
            #[obs_property(type_t = "string")]
            /// The path the recording should be saved to
            path: String,

            #[obs_property(type_t = "int")]
            bitrate: i32,

            #[obs_property(type_t = "int")]
            codec_type: i32,

            #[obs_property(type_t = "string")]
            // Custom Arguments for the muxer to use
            muxer_settings: String,

            #[obs_property(type_t = "int")]
            // The maximum time in seconds that the recording should hold
            max_time_sec: i32,

            #[obs_property(type_t = "int")]
            // The maximum size in megabytes that the recording should hold
            max_size_mb: i32,

            #[obs_property(type_t = "bool")]
            /// Whether the recording should be split into multiple files and merged later
            split_file: bool,

            #[obs_property(type_t = "bool")]
            /// Whether it should be permitted to overwrite the old file
            allow_overwrite: bool,

            #[obs_property(type_t = "string")]
            /// The directory the recording should be saved to
            directory: String,

            #[obs_property(type_t = "string")]
            /// The format to use for the file name of the recording.
            /// e.g. "%CCYY-%MM-%DD %hh-%mm-%ss"
            /// Code for formatting can be found [here](https://github.com/obsproject/obs-studio/blob/5854f3b9e5861246ea57dd4a26d3d847a8552c4b/libobs/util/platform.c#L715)
            format: String,

            #[obs_property(type_t = "string")]
            /// The extension to use for the file name of the recording (without the dot, e.g. "mpr")
            extension: String,

            #[obs_property(type_t = "bool")]
            /// Whether spaces are allowed in the file name
            allow_spaces: bool,
        }
    };
}

new_output_builder!(FFmpegMuxerOutput, "ffmpeg_muxer");
new_output_builder!(ReplayBufferOutput, "replay_buffer");
