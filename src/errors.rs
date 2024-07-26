#[derive(Clone, Copy, Debug)]
pub enum ContextError {
    StartupFailure,
    ContextExists,
    MutexPoisoned,
}

#[derive(Clone, Copy, Debug)]
pub enum ResetVideoError {
    GraphicsModuleError
}