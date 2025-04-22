use crate::{
    data::output::ObsOutputRef, runtime::ObsRuntime, sources::ObsSourceRef, unsafe_send::Sendable, utils::{ObsError, ObsString}
};

use super::{get_properties_inner, ObsProperty, ObsPropertyObject, ObsPropertyObjectPrivate};

#[async_trait::async_trait]
impl ObsPropertyObject for ObsSourceRef {
    async fn get_properties(&self) -> Result<Vec<ObsProperty>, ObsError> {
        let properties_raw = self.get_properties_raw().await?;
        get_properties_inner(properties_raw, self.runtime.clone()).await
    }
}

#[async_trait::async_trait]
impl ObsPropertyObjectPrivate for ObsSourceRef {
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
    async fn get_properties_by_id_raw(
        id: ObsString,
        runtime: ObsRuntime
    ) -> Result<Sendable<*mut libobs::obs_properties_t>, ObsError> {
        let id_ptr = Sendable(id.as_ptr());
        runtime
            .run_with_obs_result(move || unsafe {
                let id_ptr = id_ptr;
                Sendable(libobs::obs_get_source_properties(id_ptr.0))
            })
            .await
            .map_err(|e| ObsError::InvocationError(e.to_string()))
    }
}

#[async_trait::async_trait]
impl ObsPropertyObject for ObsOutputRef {
    async fn get_properties(&self) -> Result<Vec<ObsProperty>, ObsError> {
        let properties_raw = self.get_properties_raw().await?;
        get_properties_inner(properties_raw, self.runtime.clone()).await
    }
}

#[async_trait::async_trait]
impl ObsPropertyObjectPrivate for ObsOutputRef {
    async fn get_properties_raw(&self) -> Result<Sendable<*mut libobs::obs_properties_t>, ObsError> {
        let output_ptr = self.output.clone();
        self.runtime
            .run_with_obs_result(move || unsafe {
                let output_ptr = output_ptr;

                Sendable(libobs::obs_output_properties(output_ptr.0))
            })
            .await
            .map_err(|e| ObsError::InvocationError(e.to_string()))
    }

    async fn get_properties_by_id_raw(id: ObsString, runtime: ObsRuntime) -> Result<Sendable<*mut libobs::obs_properties_t>, ObsError> {
        let id_ptr = Sendable(id.as_ptr());
        runtime
            .run_with_obs_result(move || unsafe {
                let id_ptr = id_ptr;

                Sendable(libobs::obs_get_output_properties(id_ptr.0))
            })
            .await
            .map_err(|e| ObsError::InvocationError(e.to_string()))
    }
}
