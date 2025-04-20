use std::fmt::Display;

use crate::enums::ObsResetVideoStatus;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ObsBootstrapError {
    GeneralError(String),
    DownloadError(String),
    ExtractError(String),
}

/// Error type for OBS function calls.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ObsError {
    /// The `obs_startup` function failed on libobs.
    Failure,
    /// Failed to lock mutex describing whether there is a
    /// thread using libobs or not. Report to crate maintainer.
    MutexFailure,
    /// Some or no thread is already using libobs. This is a bug!
    ThreadFailure,
    /// Unable to reset video.
    ResetVideoFailure(ObsResetVideoStatus),
    /// Unable to bootstrap OBS for downloading and installing
    BootstrapperFailure(ObsBootstrapError),
    /// Unable to reset video because the program attempted to
    /// change the graphics module. This is a bug!
    ResetVideoFailureGraphicsModule,
    /// The function returned a null pointer, often indicating
    /// an error with creating the object of the requested
    /// pointer.
    NullPointer,
    OutputAlreadyActive,
    OutputStartFailure(Option<String>),
    OutputStopFailure(Option<String>),
    OutputNotFound,
    SourceNotFound,

    /// Native error from the Windows API when creating a display
    DisplayCreationError(String),

    OutputSaveBufferFailure(String),

    /// The obs thread couldn't be called
    InvocationError(String),
}

impl Display for ObsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OBS Error: ")?;

        match self {
            ObsError::Failure => write!(f, "`obs-startup` function failed on libobs"),
            ObsError::MutexFailure => write!(f, "Failed to lock mutex describing whether there is a thread using libobs or not. Report to crate maintainer."),
            ObsError::ThreadFailure => write!(f, "Some or no thread is already using libobs. This is a bug!"),
            ObsError::ResetVideoFailure(status) => write!(f, "Could not reset obs video. Status: {:?}", status),
            ObsError::ResetVideoFailureGraphicsModule => write!(f, "Unable to reset video because the program attempted to change the graphics module. This is a bug!"),
            ObsError::NullPointer => write!(f, "The function returned a null pointer, often indicating an error with creating the object of the requested pointer."),
            ObsError::OutputAlreadyActive => write!(f, "Output is already active."),
            ObsError::OutputStartFailure(s) => write!(f, "Output failed to start. Error is {:?}", s),
            ObsError::OutputStopFailure(s) => write!(f, "Output failed to stop. Error is {:?}", s),
            ObsError::OutputNotFound => write!(f, "Output not found."),
            ObsError::DisplayCreationError(e) => write!(f, "Native error from the Windows API when creating a display: {:?}", e),
            ObsError::OutputSaveBufferFailure(e) => write!(f, "Couldn't save output buffer: {:?}", e),
            ObsError::SourceNotFound => write!(f, "Source not found."),
            ObsError::BootstrapperFailure(error) => match error {
                ObsBootstrapError::GeneralError(e) => write!(f, "Bootstrapper error: {:?}", e),
                ObsBootstrapError::DownloadError(e) => write!(f, "Bootstrapper download error: {:?}", e),
                ObsBootstrapError::ExtractError(e) => write!(f, "Bootstrapper extract error: {:?}", e),
            },
            ObsError::InvocationError(e) => write!(f, "The obs thread couldn't be called: {:?}", e),
        }
    }
}

impl std::error::Error for ObsError {}
