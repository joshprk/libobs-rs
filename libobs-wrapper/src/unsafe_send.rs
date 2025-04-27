#[derive(Debug, Clone)]
pub struct Sendable<T>(pub T);

unsafe impl<T> Send for Sendable<T> {}
unsafe impl<T> Sync for Sendable<T> {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendableComp<T>(pub T);

unsafe impl<T: PartialEq> Send for SendableComp<T> {}
unsafe impl<T: PartialEq> Sync for SendableComp<T> {}
