#[macro_export]
macro_rules! run_with_obs_impl {
    ($self:expr, $function:ident, $operation:expr) => {
        {
            $self.$function(move || {
                let e = { $operation };
                return crate::unsafe_send::Sendable(e())
            })
        }
    };
    ($self:expr, $function:ident, ($($var:ident),* $(,)*), $operation:expr) => {
        {
            $(let $var = crate::unsafe_send::Sendable($var.clone());)*
            $self.$function(move || {
                $(let $var = $var.clone();)*
                let e = {
                    $(let $var = $var.0;)*
                    $operation
                };
                return crate::unsafe_send::Sendable(e())
            })
        }
    };
}

#[macro_export]
macro_rules! run_with_obs {
    ($self:expr, $operation:expr) => {
        $crate::run_with_obs_impl!($self, run_with_obs_result, $operation)
        .await
            .map_err(|e| crate::utils::ObsError::InvocationError(e.to_string()))
            .map(|x| x.0)
    };
    ($self:expr, ($($var:ident),* $(,)*), $operation:expr) => {
        $crate::run_with_obs_impl!($self, run_with_obs_result, ($($var),*), $operation)
        .await
            .map_err(|e| crate::utils::ObsError::InvocationError(e.to_string()))
            .map(|x| x.0)
    };
}

#[macro_export]
macro_rules! run_with_obs_blocking {
    ($self:expr, $operation:expr) => {
        $crate::run_with_obs_impl!($self, run_with_obs_result_blocking, $operation)
        .map_err(|e| crate::utils::ObsError::InvocationError(e.to_string()))
        .map(|x| x.0)
    };
    ($self:expr, ($($var:ident),* $(,)*), $operation:expr) => {
        $crate::run_with_obs_impl!($self, run_with_obs_result_blocking, ($($var),*), $operation)
        .map_err(|e| crate::utils::ObsError::InvocationError(e.to_string()))
        .map(|x| x.0)
    };
}

#[macro_export]
macro_rules! impl_obs_drop {
    ($struct_name: ident, ($($var:ident),* $(,)*), $operation:expr) => {
        crate::impl_obs_drop!($struct_name, self.runtime, ($($var),*), $operation);
    };
    ($struct_name: ident, $operation:expr) => {
        crate::impl_obs_drop!($struct_name, self.runtime, (), $operation);
    };
    ($struct_name: ident, $self:expr, ($($var:ident),* $(,)*), $operation:expr) => {
        impl Drop for $struct_name {
            fn drop(&mut self) {
                $(let $var = self.$var.clone();)*
                let r = crate::run_with_obs_blocking!($self, ($($var),*), $operation);
                if std::thread::panicking() {
                    return;
                }

                r.unwrap();
            }
        }
    };
}
