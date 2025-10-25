use crate::csi_types::plugin_capability::service::Type;
use crate::csi_types::{
    GetPluginCapabilitiesRequest, GetPluginCapabilitiesResponse, GetPluginInfoRequest,
    GetPluginInfoResponse, PluginCapability, ProbeRequest, ProbeResponse,
    identity_server::Identity, plugin_capability,
};
use tonic::{Request, Response, Status};
use tracing::{debug, info, instrument};

#[derive(Debug, Clone)]
pub struct IdentityService {
    driver_name: String,
    driver_version: String,
}

impl IdentityService {
    pub fn new(driver_name: String, driver_version: String) -> Self {
        info!("Creating Identity service for driver: {}", driver_name);
        Self {
            driver_name,
            driver_version,
        }
    }
}

#[tonic::async_trait]
impl Identity for IdentityService {
    #[instrument(skip(self))]
    async fn get_plugin_info(
        &self,
        _request: Request<GetPluginInfoRequest>,
    ) -> Result<Response<GetPluginInfoResponse>, Status> {
        debug!("Handling GetPluginInfo request");

        let response = GetPluginInfoResponse {
            name: self.driver_name.clone(),
            vendor_version: self.driver_version.clone(),
            manifest: std::collections::HashMap::new(),
        };

        info!(
            "Returning plugin info: {} v{}",
            self.driver_name, self.driver_version
        );
        Ok(Response::new(response))
    }

    #[instrument(skip(self))]
    async fn get_plugin_capabilities(
        &self,
        _request: Request<GetPluginCapabilitiesRequest>,
    ) -> Result<Response<GetPluginCapabilitiesResponse>, Status> {
        debug!("Handling GetPluginCapabilities request");

        let response = GetPluginCapabilitiesResponse {
            capabilities: vec![
                // Advertise that we support volume accessibility constraints
                PluginCapability {
                    r#type: Some(plugin_capability::Type::Service(
                        plugin_capability::Service {
                            r#type: Type::VolumeAccessibilityConstraints as i32,
                        },
                    )),
                },
            ],
        };

        Ok(Response::new(response))
    }

    #[instrument(skip(self))]
    async fn probe(
        &self,
        _request: Request<ProbeRequest>,
    ) -> Result<Response<ProbeResponse>, Status> {
        debug!("Handling Probe request");

        let response = ProbeResponse { ready: Some(true) };

        info!("Driver health check: OK");
        Ok(Response::new(response))
    }
}
