#[cfg(test)]
mod tests {
    use super::super::ObsError;
    use crate::enums::ObsResetVideoStatus;

    #[test]
    fn test_error_failure_display() {
        let error = ObsError::Failure;
        let display_str = format!("{}", error);
        assert!(display_str.contains("OBS Error"));
        assert!(display_str.contains("obs-startup"));
    }

    #[test]
    fn test_error_mutex_failure_display() {
        let error = ObsError::MutexFailure;
        let display_str = format!("{}", error);
        assert!(display_str.contains("mutex"));
    }

    #[test]
    fn test_error_thread_failure_display() {
        let error = ObsError::ThreadFailure;
        let display_str = format!("{}", error);
        assert!(display_str.contains("thread"));
    }

    #[test]
    fn test_error_reset_video_failure_display() {
        let error = ObsError::ResetVideoFailure(ObsResetVideoStatus::Fail);
        let display_str = format!("{}", error);
        assert!(display_str.contains("reset"));
        assert!(display_str.contains("video"));
    }

    #[test]
    fn test_error_reset_video_failure_graphics_module() {
        let error = ObsError::ResetVideoFailureGraphicsModule;
        let display_str = format!("{}", error);
        assert!(display_str.contains("graphics module"));
    }

    #[test]
    fn test_error_reset_video_failure_output_active() {
        let error = ObsError::ResetVideoFailureOutputActive;
        let display_str = format!("{}", error);
        assert!(display_str.contains("outputs were still active"));
    }

    #[test]
    fn test_error_null_pointer_display() {
        let error = ObsError::NullPointer;
        let display_str = format!("{}", error);
        assert!(display_str.contains("null pointer"));
    }

    #[test]
    fn test_error_output_already_active() {
        let error = ObsError::OutputAlreadyActive;
        let display_str = format!("{}", error);
        assert!(display_str.contains("already active"));
    }

    #[test]
    fn test_error_output_start_failure() {
        let error = ObsError::OutputStartFailure(Some("test reason".to_string()));
        let display_str = format!("{}", error);
        assert!(display_str.contains("start"));
        assert!(display_str.contains("test reason"));
    }

    #[test]
    fn test_error_output_start_failure_none() {
        let error = ObsError::OutputStartFailure(None);
        let display_str = format!("{}", error);
        assert!(display_str.contains("start"));
    }

    #[test]
    fn test_error_output_stop_failure() {
        let error = ObsError::OutputStopFailure(Some("stop error".to_string()));
        let display_str = format!("{}", error);
        assert!(display_str.contains("stop"));
        assert!(display_str.contains("stop error"));
    }

    #[test]
    fn test_error_output_pause_failure() {
        let error = ObsError::OutputPauseFailure(Some("pause error".to_string()));
        let display_str = format!("{}", error);
        assert!(display_str.contains("pause"));
        assert!(display_str.contains("pause error"));
    }

    #[test]
    fn test_error_output_not_found() {
        let error = ObsError::OutputNotFound;
        let display_str = format!("{}", error);
        assert!(display_str.contains("Output not found"));
    }

    #[test]
    fn test_error_source_not_found() {
        let error = ObsError::SourceNotFound;
        let display_str = format!("{}", error);
        assert!(display_str.contains("Source not found"));
    }

    #[test]
    fn test_error_display_creation_error() {
        let error = ObsError::DisplayCreationError("creation failed".to_string());
        let display_str = format!("{}", error);
        assert!(display_str.contains("Display"));
        assert!(display_str.contains("creation failed"));
    }

    #[test]
    fn test_error_output_save_buffer_failure() {
        let error = ObsError::OutputSaveBufferFailure("save failed".to_string());
        let display_str = format!("{}", error);
        assert!(display_str.contains("save"));
        assert!(display_str.contains("save failed"));
    }

    #[test]
    fn test_error_invocation_error() {
        let error = ObsError::InvocationError("thread error".to_string());
        let display_str = format!("{}", error);
        assert!(display_str.contains("thread"));
        assert!(display_str.contains("thread error"));
    }

    #[test]
    fn test_error_json_parse_error() {
        let error = ObsError::JsonParseError;
        let display_str = format!("{}", error);
        assert!(display_str.contains("JSON"));
    }

    #[test]
    fn test_error_no_sender_error() {
        let error = ObsError::NoSenderError;
        let display_str = format!("{}", error);
        assert!(display_str.contains("sender"));
    }

    #[test]
    fn test_error_no_available_encoders() {
        let error = ObsError::NoAvailableEncoders;
        let display_str = format!("{}", error);
        assert!(display_str.contains("encoders"));
    }

    #[test]
    fn test_error_lock_error() {
        let error = ObsError::LockError("mutex locked".to_string());
        let display_str = format!("{}", error);
        assert!(display_str.contains("mutex locked"));
    }

    #[test]
    fn test_error_unexpected() {
        let error = ObsError::Unexpected("unexpected condition".to_string());
        let display_str = format!("{}", error);
        assert!(display_str.contains("unexpected"));
        assert!(display_str.contains("unexpected condition"));
    }

    #[test]
    fn test_error_encoder_active() {
        let error = ObsError::EncoderActive;
        let display_str = format!("{}", error);
        assert!(display_str.contains("Encoder is still active"));
    }

    #[test]
    fn test_error_clone() {
        let error1 = ObsError::Failure;
        let error2 = error1.clone();
        assert_eq!(error1, error2);
    }

    #[test]
    fn test_error_eq() {
        let error1 = ObsError::NullPointer;
        let error2 = ObsError::NullPointer;
        assert_eq!(error1, error2);
    }

    #[test]
    fn test_error_ne() {
        let error1 = ObsError::Failure;
        let error2 = ObsError::NullPointer;
        assert_ne!(error1, error2);
    }

    #[test]
    fn test_error_debug() {
        let error = ObsError::Failure;
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("Failure"));
    }

    #[test]
    fn test_error_is_std_error() {
        let error = ObsError::Failure;
        let _: &dyn std::error::Error = &error;
    }
}
