#[cfg(not(feature="blocking"))]
pub use tokio::sync::{Mutex, RwLock};

#[cfg(feature="blocking")]
pub use parking_lot::{Mutex, RwLock};

#[macro_export]
#[cfg(not(feature="blocking"))]
macro_rules! oneshot_rx_recv {
    ($rx:ident) => {
        $rx.await
    };
}

#[macro_export]
#[cfg(not(feature="blocking"))]
macro_rules! rx_recv {
    ($rx:ident) => {
        $rx.recv().await
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
#[cfg(feature="blocking")]
macro_rules! oneshot_rx_recv {
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


#[macro_export]
#[cfg(not(feature="blocking"))]
macro_rules! rw_lock_blocking_write {
    ($lock:expr) => {
        $lock.blocking_write()
    };
}


#[macro_export]
#[cfg(feature="blocking")]
macro_rules! rw_lock_blocking_write {
    ($lock:expr) => {
        $lock.write()
    };
}

#[macro_export]
#[cfg(not(feature="blocking"))]
macro_rules! wrap_with_spawn_blocking {
    ($operation:expr) => {
        tokio::task::spawn_blocking(move || {
            $operation
        })
    };
}

#[macro_export]
#[cfg(feature="blocking")]
macro_rules! wrap_with_spawn_blocking {
    ($operation:expr) => {
        {
            $operation
        }
    };
}
