//! Sendable wrapper types for non-Send types
//!
//! This module provides wrapper types that allow non-Send types to be sent
//! across thread boundaries. Use with caution - these are unsafe by design.

#[cfg_attr(coverage_nightly, coverage(off))]
#[derive(Debug, Clone)]
pub struct Sendable<T>(pub T);

#[cfg_attr(coverage_nightly, coverage(off))]
unsafe impl<T> Send for Sendable<T> {}
#[cfg_attr(coverage_nightly, coverage(off))]
unsafe impl<T> Sync for Sendable<T> {}

#[cfg_attr(coverage_nightly, coverage(off))]
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct SendableComp<T>(pub T);

#[cfg_attr(coverage_nightly, coverage(off))]
unsafe impl<T: PartialEq> Send for SendableComp<T> {}
#[cfg_attr(coverage_nightly, coverage(off))]
unsafe impl<T: PartialEq> Sync for SendableComp<T> {}
