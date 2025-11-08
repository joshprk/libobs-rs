#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obs_video_format_equality() {
        assert_eq!(ObsVideoFormat::I420, ObsVideoFormat::I420);
        assert_ne!(ObsVideoFormat::I420, ObsVideoFormat::NV12);
    }

    #[test]
    fn test_obs_video_format_clone() {
        let format = ObsVideoFormat::RGBA;
        let cloned = format.clone();
        assert_eq!(format, cloned);
    }

    #[test]
    fn test_obs_video_format_debug() {
        let format = ObsVideoFormat::BGRA;
        let debug_str = format!("{:?}", format);
        assert!(debug_str.contains("BGRA"));
    }

    #[test]
    fn test_obs_colorspace_equality() {
        assert_eq!(ObsColorspace::CS709, ObsColorspace::CS709);
        assert_ne!(ObsColorspace::CS709, ObsColorspace::CS601);
    }

    #[test]
    fn test_obs_colorspace_clone() {
        let colorspace = ObsColorspace::Default;
        let cloned = colorspace.clone();
        assert_eq!(colorspace, cloned);
    }

    #[test]
    fn test_obs_video_range_equality() {
        assert_eq!(ObsVideoRange::Full, ObsVideoRange::Full);
        assert_ne!(ObsVideoRange::Full, ObsVideoRange::Partial);
    }

    #[test]
    fn test_obs_video_range_clone() {
        let range = ObsVideoRange::Default;
        let cloned = range.clone();
        assert_eq!(range, cloned);
    }

    #[test]
    fn test_obs_scale_type_equality() {
        assert_eq!(ObsScaleType::Bilinear, ObsScaleType::Bilinear);
        assert_ne!(ObsScaleType::Bilinear, ObsScaleType::Bicubic);
    }

    #[test]
    fn test_obs_scale_type_clone() {
        let scale = ObsScaleType::Lanczos;
        let cloned = scale.clone();
        assert_eq!(scale, cloned);
    }

    #[test]
    fn test_obs_graphics_module_equality() {
        assert_eq!(ObsGraphicsModule::DirectX11, ObsGraphicsModule::DirectX11);
        assert_ne!(ObsGraphicsModule::DirectX11, ObsGraphicsModule::OpenGL);
    }

    #[test]
    fn test_obs_graphics_module_clone() {
        let module = ObsGraphicsModule::OpenGL;
        let cloned = module.clone();
        assert_eq!(module, cloned);
    }

    #[test]
    fn test_obs_reset_video_status_equality() {
        assert_eq!(ObsResetVideoStatus::Success, ObsResetVideoStatus::Success);
        assert_ne!(ObsResetVideoStatus::Success, ObsResetVideoStatus::Fail);
    }

    #[test]
    fn test_obs_reset_video_status_clone() {
        let status = ObsResetVideoStatus::Success;
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_obs_reset_video_status_debug() {
        let status = ObsResetVideoStatus::Success;
        let debug_str = format!("{:?}", status);
        assert!(debug_str.contains("Success"));
    }

    #[test]
    fn test_all_enums_are_copy() {
        let format = ObsVideoFormat::I420;
        let _copy1 = format;
        let _copy2 = format; // Should work because it's Copy

        let colorspace = ObsColorspace::CS709;
        let _copy1 = colorspace;
        let _copy2 = colorspace;
    }

    #[test]
    fn test_obs_graphics_module_display() {
        let module = ObsGraphicsModule::DirectX11;
        let display_str = format!("{}", module);
        assert!(display_str.contains("direct3d11") || display_str.contains("d3d11"));
    }

    #[test]
    fn test_obs_video_format_display() {
        let format = ObsVideoFormat::I420;
        let display_str = format!("{}", format);
        assert!(!display_str.is_empty());
    }
}
