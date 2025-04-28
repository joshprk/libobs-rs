use crate::utils::ObsString;

#[macro_export]
macro_rules! impl_signal_manager {
    ($objName: literal, $name: ident for $ref: ident<$ptr: ty>, [
        $($signal_name: literal: $fn_name: expr => $return_type: ty),* $(,)*
    ]) => {
        paste::paste! {
            lazy_static::lazy_static! {
                $(static ref [<$signal_name:snake:upper _SENDERS>]: std::sync::Arc<$crate::utils::async_sync::RwLock<std::collections::HashMap<$crate::unsafe_send::SendableComp<$ptr>, tokio::sync::broadcast::Sender<$return_type>>>> = std::sync::Arc::new($crate::utils::async_sync::RwLock::new(std::collections::HashMap::new()));)*
            }

            $(
            extern "C" fn [< $signal_name:snake _handler>](obj_ptr: *mut std::ffi::c_void, cd: *mut libobs::calldata_t) {
                #[allow(unused_unsafe)]
                let res = unsafe { $fn_name(cd) };
                if res.is_err() {
                    log::warn!("Error processing signal {}: {:?}", stringify!($signal_name), res.err());
                    return;
                }

                let res = res.unwrap();
                let senders = crate::rw_lock_blocking_read!([<$signal_name:snake:upper _SENDERS>]);
                let senders = senders.get(&$crate::unsafe_send::SendableComp(obj_ptr as $ptr));
                if senders.is_none() {
                    log::warn!("No sender found for signal {}", stringify!($signal_name));
                    return;
                }

                let senders = senders.unwrap();
                let res = senders.send(res);
                if let Err(e) = res {
                    log::warn!("Error sending signal {}: {:?}", stringify!($signal_name), e);
                };
            })*
            #[derive(Debug)]
            pub struct $name {
                pointer: $crate::unsafe_send::SendableComp<$ptr>,
                runtime: $crate::runtime::ObsRuntime,
                $(
                    #[allow(dead_code)]
                    [< $signal_name:snake _rx>]: tokio::sync::broadcast::Receiver<$return_type>,
                )*
            }

            impl $name {
                #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
                pub(crate) async fn new(ptr: &Sendable<$ptr>, runtime: $crate::runtime::ObsRuntime) -> Result<Self, crate::utils::ObsError> {
                    use crate::{utils::ObsString, unsafe_send::SendableComp};
                    let pointer =  SendableComp(ptr.0);

                    $(
                        let senders = [<$signal_name:snake:upper _SENDERS>].clone();
                        let mut senders = senders.write().await;
                        let (tx, [< $signal_name:snake _rx>]) = tokio::sync::broadcast::channel(16);
                        senders.insert(pointer.clone(), tx);
                    )*

                    crate::run_with_obs!(runtime, (pointer), move || unsafe {
                            let handler = libobs::[< obs_ $objName:snake _get_signal_handler>](pointer);
                            $(
                                let signal = ObsString::new($signal_name);
                                libobs::signal_handler_connect(
                                    handler,
                                    signal.as_ptr().0,
                                    Some([< $signal_name:snake _handler>]),
                                    pointer as *mut std::ffi::c_void,
                                );
                            )*
                    }).await?;

                    Ok(Self {
                        pointer,
                        runtime,
                        $(
                            [< $signal_name:snake _rx>],
                        )*
                    })
                }

                $(
                    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
                    pub async fn [<get_ $signal_name:snake _receiver>](&self) -> Result<tokio::sync::broadcast::Receiver<$return_type>, crate::utils::ObsError> {
                        let handlers = crate::rw_lock_blocking_read!([<$signal_name:snake:upper _SENDERS>]);
                        let rx = handlers.get(&self.pointer)
                            .ok_or_else(|| crate::utils::ObsError::NoSenderError)?
                            .subscribe();

                        Ok(rx)
                    }
                )*
            }

            impl Drop for $name {
                fn drop(&mut self) {
                    let ptr = self.pointer.clone();
                    let runtime = self.runtime.clone();

                    let future = crate::run_with_obs!(runtime, (ptr), move || unsafe {
                        let handler = libobs::[< obs_ $objName:snake _get_signal_handler>](ptr);
                        $(
                            let signal = crate::utils::ObsString::new($signal_name);
                            libobs::signal_handler_disconnect(
                                handler,
                                signal.as_ptr().0,
                                Some([< $signal_name:snake _handler>]),
                                ptr as *mut std::ffi::c_void,
                            );
                        )*
                    });

                    let tmp_ptr = self.pointer.clone();
                    #[cfg(not(feature="blocking"))]
                    let r = futures::executor::block_on(async move {
                        $(
                            let mut handlers = [<$signal_name:snake:upper _SENDERS>].write().await;
                            handlers.remove(&tmp_ptr);
                        )*

                        future.await
                    });

                    #[cfg(feature="blocking")]
                    let r = {
                        $(
                            let mut handlers = [<$signal_name:snake:upper _SENDERS>].blocking_write();
                            handlers.remove(&self.pointer);
                        )*

                        future
                    };

                    if std::thread::panicking() {
                        return;
                    }

                    r.unwrap();
                }
            }
        }
    };
}

pub unsafe fn process_no_op(
    _cd: *mut libobs::calldata_t,
) -> anyhow::Result<()> {
    Ok(())
}

pub fn get_boolean_processor(
    name: &str,
) -> impl Fn(*mut libobs::calldata_t) -> anyhow::Result<bool> {
    let name = ObsString::new(name);
    move |cd| unsafe {
        let mut res = false;
        let success = libobs::calldata_get_data(
            cd,
            name.as_ptr().0,
            &mut res as *mut _ as *mut std::ffi::c_void,
            size_of::<bool>()
        );

        if !success {
            return Err(anyhow::anyhow!("Failed to get boolean from calldata"));
        }

        Ok(res)
    }
}