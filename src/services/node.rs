use crate::csi_types::{
    NodeExpandVolumeRequest, NodeExpandVolumeResponse, NodeGetCapabilitiesRequest,
    NodeGetCapabilitiesResponse, NodeGetInfoRequest, NodeGetInfoResponse,
    NodeGetVolumeStatsRequest, NodeGetVolumeStatsResponse, NodePublishVolumeRequest,
    NodePublishVolumeResponse, NodeServiceCapability, NodeStageVolumeRequest,
    NodeStageVolumeResponse, NodeUnpublishVolumeRequest, NodeUnpublishVolumeResponse,
    NodeUnstageVolumeRequest, NodeUnstageVolumeResponse, node_server::Node,
    node_service_capability,
};
use crate::lustre::{LustreClient, MountManager};
use tonic::{Request, Response, Status};
use tracing::{debug, error, info, instrument, warn};

#[derive(Debug, Clone)]
pub struct NodeService {
    node_id: String,
    mount_manager: MountManager,
    lustre_client: LustreClient,
}

impl NodeService {
    pub fn new(node_id: String) -> Self {
        info!("Creating Node service for node: {}", node_id);

        let lustre_client = LustreClient::new();

        // Check Lustre availability on startup
        if let Err(e) = lustre_client.ensure_lustre_loaded() {
            warn!("Failed to ensure Lustre is loaded: {}", e);
        }

        if let Ok(version) = lustre_client.get_lustre_version() {
            info!("Lustre version: {}", version);
        }

        Self {
            node_id,
            mount_manager: MountManager::new(),
            lustre_client,
        }
    }
}

#[tonic::async_trait]
impl Node for NodeService {
    #[instrument(skip(self))]
    async fn node_stage_volume(
        &self,
        _request: Request<NodeStageVolumeRequest>,
    ) -> Result<Response<NodeStageVolumeResponse>, Status> {
        debug!("NodeStageVolume called");
        // For Lustre, we don't need staging
        Ok(Response::new(NodeStageVolumeResponse {}))
    }

    #[instrument(skip(self))]
    async fn node_unstage_volume(
        &self,
        _request: Request<NodeUnstageVolumeRequest>,
    ) -> Result<Response<NodeUnstageVolumeResponse>, Status> {
        debug!("NodeUnstageVolume called");
        Ok(Response::new(NodeUnstageVolumeResponse {}))
    }

    #[instrument(skip(self, request))]
    async fn node_publish_volume(
        &self,
        request: Request<NodePublishVolumeRequest>,
    ) -> Result<Response<NodePublishVolumeResponse>, Status> {
        let req = request.into_inner();

        info!("NodePublishVolume called for volume: {}", req.volume_id);
        debug!("Target path: {}", req.target_path);

        // Validate request
        if req.volume_id.is_empty() {
            return Err(Status::invalid_argument("volume_id is required"));
        }
        if req.target_path.is_empty() {
            return Err(Status::invalid_argument("target_path is required"));
        }

        // Get volume context (contains Lustre-specific info)
        let volume_context = req.volume_context;
        let source = volume_context
            .get("source")
            .ok_or_else(|| Status::invalid_argument("source not found in volume_context"))?;

        // Validate source format
        if let Err(e) = self.lustre_client.validate_source(source) {
            return Err(Status::invalid_argument(format!(
                "Invalid Lustre source: {}",
                e
            )));
        }
        // Get mount options
        let mount_options = volume_context
            .get("mountOptions")
            .map(|s| s.split(',').map(String::from).collect())
            .unwrap_or_else(|| vec!["flock".to_string(), "user_xattr".to_string()]);

        info!("Mounting Lustre source: {} to {}", source, req.target_path);

        // Perform the mount
        if let Err(e) = self
            .mount_manager
            .mount(source, &req.target_path, &mount_options)
            .await
        {
            error!("Failed to mount volume: {}", e);
            return Err(Status::internal(format!("Mount failed: {}", e)));
        }

        info!("Successfully published volume {}", req.volume_id);
        Ok(Response::new(NodePublishVolumeResponse {}))
    }

    #[instrument(skip(self, request))]
    async fn node_unpublish_volume(
        &self,
        request: Request<NodeUnpublishVolumeRequest>,
    ) -> Result<Response<NodeUnpublishVolumeResponse>, Status> {
        let req = request.into_inner();

        info!("NodeUnpublishVolume called for volume: {}", req.volume_id);
        debug!("Target path: {}", req.target_path);

        // Validate request
        if req.volume_id.is_empty() {
            return Err(Status::invalid_argument("volume_id is required"));
        }
        if req.target_path.is_empty() {
            return Err(Status::invalid_argument("target_path is required"));
        }

        // Perform the unmount
        if let Err(e) = self.mount_manager.unmount(&req.target_path).await {
            warn!(
                "Failed to unmount volume (might already be unmounted): {}",
                e
            );
            // Don't fail - volume might already be unmounted
        }

        info!("Successfully unpublished volume {}", req.volume_id);
        Ok(Response::new(NodeUnpublishVolumeResponse {}))
    }

    #[instrument(skip(self))]
    async fn node_get_volume_stats(
        &self,
        _request: Request<NodeGetVolumeStatsRequest>,
    ) -> Result<Response<NodeGetVolumeStatsResponse>, Status> {
        debug!("NodeGetVolumeStats called");
        Err(Status::unimplemented(
            "NodeGetVolumeStats not yet implemented",
        ))
    }

    #[instrument(skip(self))]
    async fn node_expand_volume(
        &self,
        _request: Request<NodeExpandVolumeRequest>,
    ) -> Result<Response<NodeExpandVolumeResponse>, Status> {
        debug!("NodeExpandVolume called");
        Err(Status::unimplemented(
            "NodeExpandVolume not yet implemented",
        ))
    }

    #[instrument(skip(self))]
    async fn node_get_capabilities(
        &self,
        _request: Request<NodeGetCapabilitiesRequest>,
    ) -> Result<Response<NodeGetCapabilitiesResponse>, Status> {
        debug!("NodeGetCapabilities called");

        let response = NodeGetCapabilitiesResponse {
            capabilities: vec![NodeServiceCapability {
                r#type: Some(node_service_capability::Type::Rpc(
                    node_service_capability::Rpc {
                        r#type: node_service_capability::rpc::Type::StageUnstageVolume as i32,
                    },
                )),
            }],
        };

        Ok(Response::new(response))
    }

    #[instrument(skip(self))]
    async fn node_get_info(
        &self,
        _request: Request<NodeGetInfoRequest>,
    ) -> Result<Response<NodeGetInfoResponse>, Status> {
        debug!("NodeGetInfo called");

        let response = NodeGetInfoResponse {
            node_id: self.node_id.clone(),
            max_volumes_per_node: 0,
            accessible_topology: None,
        };

        info!("Returning node info for: {}", self.node_id);
        Ok(Response::new(response))
    }
}
