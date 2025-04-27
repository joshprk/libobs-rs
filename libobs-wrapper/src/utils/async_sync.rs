#[cfg(not(feature="blocking"))]
pub use tokio::sync::{Mutex, RwLock};

#[cfg(feature="blocking")]
pub use parking_lot::{Mutex, RwLock};

#[macro_export]
#[cfg(not(feature="blocking"))]
macro_rules! rx_recv {
    ($rx:ident) => {
        $rx.await
    };
}

#[macro_export]
#[cfg(feature="blocking")]
macro_rules! rx_recv {
    ($rx:ident) => {
        $rx.blocking_recv()
    };
}

#[macro_export]
#[cfg(not(feature="blocking"))]
macro_rules! mutex_blocking_lock {
    ($lock:ident) => {
        $lock.blocking_lock()
    };
}

#[macro_export]
#[cfg(feature="blocking")]
macro_rules! mutex_blocking_lock {
    ($lock:ident) => {
        $lock.lock()
    };
}

#[macro_export]
#[cfg(not(feature="blocking"))]
macro_rules! rw_lock_blocking_read {
    ($lock:expr) => {
        $lock.blocking_read()
    };
}


#[macro_export]
#[cfg(feature="blocking")]
macro_rules! rw_lock_blocking_read {
    ($lock:expr) => {
        $lock.read()
    };
}