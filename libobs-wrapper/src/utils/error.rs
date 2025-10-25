use std::fmt::Display;

use crate::enums::ObsResetVideoStatus;

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
    OutputPauseFailure(Option<String>),
    OutputNotFound,
    SourceNotFound,

    /// Native error from the Windows API when creating a display
    DisplayCreationError(String),

    OutputSaveBufferFailure(String),

    /// The obs thread couldn't be called
    InvocationError(String),

    JsonParseError,
    /// Couldn't get the sender of the signal
    NoSenderError,
    NoAvailableEncoders,
    /// Error locking a mutex or RwLock
    LockError(String),
    Unexpected(String),
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
            ObsError::InvocationError(e) => write!(f, "The obs thread couldn't be called: {:?}", e),
            ObsError::JsonParseError => write!(f, "Failed to parse JSON data."),
            ObsError::NoSenderError => write!(f, "Couldn't get the sender of the signal."),
            ObsError::NoAvailableEncoders => write!(f, "No available encoders found."),
            ObsError::OutputPauseFailure(s) => write!(f, "Output failed to pause. Error is {:?}", s),
            ObsError::LockError(e) => write!(f, "Error locking a mutex or RwLock: {:?}", e),
            ObsError::Unexpected(e) => write!(f, "Unexpected error: {:?}", e),
        }
    }
}

impl std::error::Error for ObsError {}
