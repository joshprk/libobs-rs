#[macro_export]
macro_rules! run_with_obs_impl {
    ($self:expr, $function:ident, $operation:expr) => {
        $crate::run_with_obs_impl!($self, $function, (), $operation)
    };
    ($self:expr, $function:ident, ($($var:ident),* $(,)*), $operation:expr) => {
        {
            $(let $var = $var.clone();)*
            $self.$function(move || {
                $(let $var = $var;)*
                let e = {
                    $(let $var = $var.0;)*
                    $operation
                };
                return e()
            })
        }
    };
    (SEPARATE_THREAD, $self:expr, $function:ident, ($($var:ident),* $(,)*), $operation:expr) => {
        {
            $(let $var = $var.clone();)*
            tokio::task::spawn_blocking(move || {
                $self.$function(move || {
                    $(let $var = $var;)*
                    let e = {
                        $(let $var = $var.0;)*
                        $operation
                    };
                    return e()
                }).unwrap()
            })
        }
    };
}

#[macro_export]
macro_rules! run_with_obs {
    ($self:expr, $operation:expr) => {
        {
            $crate::run_with_obs_impl!($self, run_with_obs_result, $operation)
                .map_err(|e| $crate::utils::ObsError::InvocationError(e.to_string()))
        }
    };
    ($self:expr, ($($var:ident),* $(,)*), $operation:expr) => {
        {
            $crate::run_with_obs_impl!($self, run_with_obs_result, ($($var),*), $operation)
                .map_err(|e| $crate::utils::ObsError::InvocationError(e.to_string()))
        }
    };
}

#[macro_export]
/// This function can only be called OUTSIDE of the OBS thread.
macro_rules! run_with_obs_blocking {
    ($self:expr, $operation:expr) => {
        $crate::run_with_obs_impl!($self, run_with_obs_result_blocking, (), $operation)
        .map_err(|e| $crate::utils::ObsError::InvocationError(e.to_string()))
    };
    ($self:expr, ($($var:ident),* $(,)*), $operation:expr) => {
        $crate::run_with_obs_impl!($self, run_with_obs_result_blocking, ($($var),*), $operation)
        .map_err(|e| $crate::utils::ObsError::InvocationError(e.to_string()))
    };
    (SEPARATE_THREAD, $self:expr, ($($var:ident),* $(,)*), $operation:expr) => {
        use futures_util::TryFutureExt;
        $crate::run_with_obs_impl!(SEPARATE_THREAD, $self, run_with_obs_result_blocking, ($($var),*), $operation)
            .map_err(|e| $crate::utils::ObsError::InvocationError(e.to_string()))
    };
}

#[macro_export]
macro_rules! impl_obs_drop {
    ($struct_name: ident, $operation:expr) => {
        $crate::impl_obs_drop!($struct_name, (), $operation);
    };
    ($struct_name: ident, ($($var:ident),* $(,)*), $operation:expr) => {
        impl Drop for $struct_name {
            fn drop(&mut self) {
                log::trace!("Dropping {}...", stringify!($struct_name));
                $(let $var = self.$var.clone();)*
                #[cfg(not(feature="no_blocking_drops"))]
                {
                    let r = $crate::run_with_obs!(self.runtime, ($($var),*), $operation);
                    if std::thread::panicking() {
                        return;
                    }

                    r.unwrap();
                }

                #[cfg(feature="no_blocking_drops")]
                {
                    let __runtime = self.runtime.clone();
                    $crate::run_with_obs_blocking!(SEPARATE_THREAD, __runtime, ($($var),*), $operation);
                }
            }
        }
    };
    (is_runtime, $struct_name: ident, ($($var:ident),* $(,)*), $operation:expr) => {
        impl Drop for $struct_name {
            fn drop(&mut self) {
                log::trace!("Dropping {}...", stringify!($struct_name));
                $(let $var = self.$var.clone();)*
                let r = $crate::run_with_obs_blocking!(self, ($($var),*), $operation);
                if std::thread::panicking() {
                    return;
                }

                r.unwrap();
            }
        }
    };
}
