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
}

#[macro_export]
macro_rules! run_with_obs {
    ($self:expr, $operation:expr) => {
        {
            #[cfg(not(feature="blocking"))]
            use futures_util::TryFutureExt;

            $crate::run_with_obs_impl!($self, run_with_obs_result, $operation)
                .map_err(|e| crate::utils::ObsError::InvocationError(e.to_string()))
        }
    };
    ($self:expr, ($($var:ident),* $(,)*), $operation:expr) => {
        {
            #[cfg(not(feature="blocking"))]
            use futures_util::TryFutureExt;

            $crate::run_with_obs_impl!($self, run_with_obs_result, ($($var),*), $operation)
                .map_err(|e| crate::utils::ObsError::InvocationError(e.to_string()))
        }
    };
}

#[macro_export]
/// This function can only be called OUTSIDE of the OBS thread.
macro_rules! run_with_obs_blocking {
    ($self:expr, $operation:expr) => {
        $crate::run_with_obs_impl!($self, run_with_obs_result_blocking, (), $operation)
        .map_err(|e| crate::utils::ObsError::InvocationError(e.to_string()))
    };
    ($self:expr, ($($var:ident),* $(,)*), $operation:expr) => {
        $crate::run_with_obs_impl!($self, run_with_obs_result_blocking, ($($var),*), $operation)
        .map_err(|e| crate::utils::ObsError::InvocationError(e.to_string()))
    };
}

#[macro_export]
macro_rules! impl_obs_drop {
    ($struct_name: ident, $operation:expr) => {
        crate::impl_obs_drop!($struct_name, (), $operation);
    };
    ($struct_name: ident, ($($var:ident),* $(,)*), $operation:expr) => {
        impl Drop for $struct_name {
            fn drop(&mut self) {
                //TODO Make sure that we are not blocking when dropping the object
                $(let $var = self.$var.clone();)*
                #[cfg(not(feature="blocking"))]
                let r = futures::executor::block_on(async {
                    return crate::run_with_obs!(self.runtime, ($($var),*), $operation).await
                });

                #[cfg(feature="blocking")]
                let r = crate::run_with_obs!(self.runtime, ($($var),*), $operation);
                if std::thread::panicking() {
                    return;
                }

                r.unwrap();
            }
        }
    };
    (is_runtime, $struct_name: ident, ($($var:ident),* $(,)*), $operation:expr) => {
        impl Drop for $struct_name {
            fn drop(&mut self) {
                $(let $var = self.$var.clone();)*
                let r = crate::run_with_obs_blocking!(self, ($($var),*), $operation);
                if std::thread::panicking() {
                    return;
                }

                r.unwrap();
            }
        }
    };
}
