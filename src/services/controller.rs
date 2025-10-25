use crate::csi_types::{
    ControllerExpandVolumeRequest, ControllerExpandVolumeResponse,
    ControllerGetCapabilitiesRequest, ControllerGetCapabilitiesResponse,
    ControllerGetVolumeRequest, ControllerGetVolumeResponse, ControllerModifyVolumeRequest,
    ControllerModifyVolumeResponse, ControllerPublishVolumeRequest,
    ControllerPublishVolumeResponse, ControllerUnpublishVolumeRequest,
    ControllerUnpublishVolumeResponse, CreateSnapshotRequest, CreateSnapshotResponse,
    CreateVolumeRequest, CreateVolumeResponse, DeleteSnapshotRequest, DeleteSnapshotResponse,
    DeleteVolumeRequest, DeleteVolumeResponse, GetCapacityRequest, GetCapacityResponse,
    GetSnapshotRequest, GetSnapshotResponse, ListSnapshotsRequest, ListSnapshotsResponse,
    ListVolumesRequest, ListVolumesResponse, ValidateVolumeCapabilitiesRequest,
    ValidateVolumeCapabilitiesResponse, controller_server::Controller,
};
use tonic::{Request, Response, Status};
use tracing::{debug, info, instrument};

#[derive(Debug, Clone, Default)]
pub struct ControllerService;

impl ControllerService {
    pub fn new() -> Self {
        info!("Creating Controller service");
        Self
    }
}

#[tonic::async_trait]
impl Controller for ControllerService {
    #[instrument(skip(self))]
    async fn create_volume(
        &self,
        _request: Request<CreateVolumeRequest>,
    ) -> Result<Response<CreateVolumeResponse>, Status> {
        debug!("CreateVolume called");
        Err(Status::unimplemented("CreateVolume not yet implemented"))
    }

    #[instrument(skip(self))]
    async fn delete_volume(
        &self,
        _request: Request<DeleteVolumeRequest>,
    ) -> Result<Response<DeleteVolumeResponse>, Status> {
        debug!("DeleteVolume called");
        Err(Status::unimplemented("DeleteVolume not yet implemented"))
    }

    #[instrument(skip(self))]
    async fn controller_publish_volume(
        &self,
        _request: Request<ControllerPublishVolumeRequest>,
    ) -> Result<Response<ControllerPublishVolumeResponse>, Status> {
        debug!("ControllerPublishVolume called");
        Err(Status::unimplemented(
            "ControllerPublishVolume not yet implemented",
        ))
    }

    #[instrument(skip(self))]
    async fn controller_unpublish_volume(
        &self,
        _request: Request<ControllerUnpublishVolumeRequest>,
    ) -> Result<Response<ControllerUnpublishVolumeResponse>, Status> {
        debug!("ControllerUnpublishVolume called");
        Err(Status::unimplemented(
            "ControllerUnpublishVolume not yet implemented",
        ))
    }

    // Validate that we support the requested capabilities
    async fn validate_volume_capabilities(
        &self,
        request: Request<ValidateVolumeCapabilitiesRequest>,
    ) -> Result<Response<ValidateVolumeCapabilitiesResponse>, Status> {
        let req = request.into_inner();
        debug!("ValidateVolumeCapabilities for volume: {}", req.volume_id);

        // We support ReadWriteMany for Lustre
        Ok(Response::new(ValidateVolumeCapabilitiesResponse {
            confirmed: Some(
                crate::csi_types::validate_volume_capabilities_response::Confirmed {
                    volume_capabilities: req.volume_capabilities,
                    volume_context: req.volume_context,
                    parameters: req.parameters,
                    mutable_parameters: Default::default(),
                },
            ),
            message: "Lustre supports all access modes".to_string(),
        }))
    }

    #[instrument(skip(self))]
    async fn list_volumes(
        &self,
        _request: Request<ListVolumesRequest>,
    ) -> Result<Response<ListVolumesResponse>, Status> {
        debug!("ListVolumes called");
        Err(Status::unimplemented("ListVolumes not yet implemented"))
    }

    #[instrument(skip(self))]
    async fn get_capacity(
        &self,
        _request: Request<GetCapacityRequest>,
    ) -> Result<Response<GetCapacityResponse>, Status> {
        debug!("GetCapacity called");
        Err(Status::unimplemented("GetCapacity not yet implemented"))
    }

    async fn controller_get_capabilities(
        &self,
        _request: Request<ControllerGetCapabilitiesRequest>,
    ) -> Result<Response<ControllerGetCapabilitiesResponse>, Status> {
        debug!("ControllerGetCapabilities called");

        Ok(Response::new(ControllerGetCapabilitiesResponse {
            capabilities: vec![], // Empty - we don't dynamically provision
        }))
    }

    #[instrument(skip(self))]
    async fn create_snapshot(
        &self,
        _request: Request<CreateSnapshotRequest>,
    ) -> Result<Response<CreateSnapshotResponse>, Status> {
        debug!("CreateSnapshot called");
        Err(Status::unimplemented("CreateSnapshot not yet implemented"))
    }

    #[instrument(skip(self))]
    async fn delete_snapshot(
        &self,
        _request: Request<DeleteSnapshotRequest>,
    ) -> Result<Response<DeleteSnapshotResponse>, Status> {
        debug!("DeleteSnapshot called");
        Err(Status::unimplemented("DeleteSnapshot not yet implemented"))
    }

    #[instrument(skip(self))]
    async fn list_snapshots(
        &self,
        _request: Request<ListSnapshotsRequest>,
    ) -> Result<Response<ListSnapshotsResponse>, Status> {
        debug!("ListSnapshots called");
        Err(Status::unimplemented("ListSnapshots not yet implemented"))
    }

    #[instrument(skip(self))]
    async fn get_snapshot(
        &self,
        _request: Request<GetSnapshotRequest>,
    ) -> Result<Response<GetSnapshotResponse>, Status> {
        debug!("GetSnapshot called");
        Err(Status::unimplemented("GetSnapshot not yet implemented"))
    }

    #[instrument(skip(self))]
    async fn controller_expand_volume(
        &self,
        _request: Request<ControllerExpandVolumeRequest>,
    ) -> Result<Response<ControllerExpandVolumeResponse>, Status> {
        debug!("ControllerExpandVolume called");
        Err(Status::unimplemented(
            "ControllerExpandVolume not yet implemented",
        ))
    }

    #[instrument(skip(self))]
    async fn controller_get_volume(
        &self,
        _request: Request<ControllerGetVolumeRequest>,
    ) -> Result<Response<ControllerGetVolumeResponse>, Status> {
        debug!("ControllerGetVolume called");
        Err(Status::unimplemented(
            "ControllerGetVolume not yet implemented",
        ))
    }

    #[instrument(skip(self))]
    async fn controller_modify_volume(
        &self,
        _request: Request<ControllerModifyVolumeRequest>,
    ) -> Result<Response<ControllerModifyVolumeResponse>, Status> {
        debug!("ControllerModifyVolume called");
        Err(Status::unimplemented(
            "ControllerModifyVolume not yet implemented",
        ))
    }
}
