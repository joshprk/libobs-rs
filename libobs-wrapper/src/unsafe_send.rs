#[derive(Debug, Clone)]
pub struct Sendable<T>(pub T);

unsafe impl<T> Send for Sendable<T> {}
unsafe impl<T> Sync for Sendable<T> {}
