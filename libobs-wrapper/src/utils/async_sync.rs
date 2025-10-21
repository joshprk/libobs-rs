pub use parking_lot::{Mutex, RwLock};

#[macro_export]
macro_rules! rx_recv {
    ($rx:ident) => {
        $rx.blocking_recv()
    };
}

#[macro_export]
macro_rules! oneshot_rx_recv {
    ($rx:ident) => {
        $rx.blocking_recv()
    };
}

#[macro_export]
macro_rules! mutex_blocking_lock {
    ($lock:ident) => {
        $lock.lock()
    };
}


#[macro_export]
macro_rules! rw_lock_blocking_read {
    ($lock:expr) => {
        $lock.read()
    };
}



#[macro_export]
macro_rules! rw_lock_blocking_write {
    ($lock:expr) => {
        $lock.write()
    };
}