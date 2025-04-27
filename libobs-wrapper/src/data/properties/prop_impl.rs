use crate::{
    data::output::ObsOutputRef,
    runtime::ObsRuntime,
    sources::ObsSourceRef,
    unsafe_send::Sendable,
    utils::{ObsError, ObsString},
};

use super::{get_properties_inner, ObsProperty, ObsPropertyObject, ObsPropertyObjectPrivate};

#[cfg_attr(not(feature = "blocking"), async_trait::async_trait)]
impl ObsPropertyObject for ObsSourceRef {
    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn get_properties(&self) -> Result<Vec<ObsProperty>, ObsError> {
        let properties_raw = self.get_properties_raw().await?;
        get_properties_inner(properties_raw, self.runtime.clone()).await
    }
}

#[cfg_attr(not(feature = "blocking"), async_trait::async_trait)]
impl ObsPropertyObjectPrivate for ObsSourceRef {
    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn get_properties_raw(
        &self,
    ) -> Result<Sendable<*mut libobs::obs_properties_t>, ObsError> {
        let source_ptr = self.source.clone();
        self.runtime
            .run_with_obs_result(move || unsafe {
                let source_ptr = source_ptr;

                Sendable(libobs::obs_source_properties(source_ptr.0))
            })
            .await
            .map_err(|e| ObsError::InvocationError(e.to_string()))
    }
    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn get_properties_by_id_raw<T: Into<ObsString> + Sync + Send>(
        id: T,
        runtime: ObsRuntime,
    ) -> Result<Sendable<*mut libobs::obs_properties_t>, ObsError> {
        let id: ObsString = id.into();
        let id_ptr = id.as_ptr();
        runtime
            .run_with_obs_result(move || unsafe {
                let id_ptr = id_ptr;
                Sendable(libobs::obs_get_source_properties(id_ptr.0))
            })
            .await
            .map_err(|e| ObsError::InvocationError(e.to_string()))
    }
}

#[cfg_attr(not(feature = "blocking"), async_trait::async_trait)]
impl ObsPropertyObject for ObsOutputRef {
    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn get_properties(&self) -> Result<Vec<ObsProperty>, ObsError> {
        let properties_raw = self.get_properties_raw().await?;
        get_properties_inner(properties_raw, self.runtime.clone()).await
    }
}

#[cfg_attr(not(feature = "blocking"), async_trait::async_trait)]
impl ObsPropertyObjectPrivate for ObsOutputRef {
    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn get_properties_raw(
        &self,
    ) -> Result<Sendable<*mut libobs::obs_properties_t>, ObsError> {
        let output_ptr = self.output.clone();
        self.runtime
            .run_with_obs_result(move || unsafe {
                let output_ptr = output_ptr;

                Sendable(libobs::obs_output_properties(output_ptr.0))
            })
            .await
            .map_err(|e| ObsError::InvocationError(e.to_string()))
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn get_properties_by_id_raw<T: Into<ObsString> + Sync + Send>(
        id: T,
        runtime: ObsRuntime,
    ) -> Result<Sendable<*mut libobs::obs_properties_t>, ObsError> {
        let id: ObsString = id.into();
        let id_ptr = id.as_ptr();
        runtime
            .run_with_obs_result(move || unsafe {
                let id_ptr = id_ptr;

                Sendable(libobs::obs_get_output_properties(id_ptr.0))
            })
            .await
            .map_err(|e| ObsError::InvocationError(e.to_string()))
    }
}
