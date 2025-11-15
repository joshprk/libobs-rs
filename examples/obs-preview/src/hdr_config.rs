use libobs_wrapper::{
    data::video::ObsVideoInfoBuilder,
    display::{GsColorFormat, ObsDisplayCreationData},
    enums::{ObsColorspace, ObsVideoFormat, ObsVideoRange},
};

/// Configuration options for HDR support in the preview example
pub struct HdrConfig {
    pub enable_hdr: bool,
    pub use_pq_colorspace: bool, // true for PQ, false for HLG
}

impl Default for HdrConfig {
    fn default() -> Self {
        Self {
            enable_hdr: true,
            use_pq_colorspace: true,
        }
    }
}

impl HdrConfig {
    pub fn configure_video_info(&self, builder: ObsVideoInfoBuilder) -> ObsVideoInfoBuilder {
        if self.enable_hdr {
            let colorspace = if self.use_pq_colorspace {
                ObsColorspace::CS2100PQ
            } else {
                ObsColorspace::CS2100HLG
            };

            builder
                .output_format(ObsVideoFormat::P010) // 10-bit format for HDR
                .colorspace(colorspace)
                .range(ObsVideoRange::Full) // Full range for HDR
                .gpu_conversion(true) // Essential for HDR processing
        } else {
            // Standard SDR configuration
            builder
                .output_format(ObsVideoFormat::NV12)
                .colorspace(ObsColorspace::CS709)
                .range(ObsVideoRange::Default)
                .gpu_conversion(true)
        }
    }

    pub fn configure_display_data(&self, data: ObsDisplayCreationData) -> ObsDisplayCreationData {
        if self.enable_hdr {
            data.set_format(GsColorFormat::RGBA16F) // 16-bit float for HDR
                .set_backbuffers(2) // More backbuffers for smoother rendering
        } else {
            // Standard SDR configuration
            data.set_format(GsColorFormat::BGRA)
                .set_backbuffers(1)
        }
    }
}