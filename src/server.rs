use anyhow::Result;
use std::path::Path;
use tokio::fs;
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;
use tracing::{error, info};

use crate::config::Config;
use crate::csi_types::{
    controller_server::ControllerServer, identity_server::IdentityServer, node_server::NodeServer,
};
use crate::services::{ControllerService, IdentityService, NodeService};

pub struct CSIServer {
    identity_service: IdentityService,
    node_service: NodeService,
    controller_service: ControllerService,
}

impl CSIServer {
    pub fn new(config: Config) -> Result<Self> {
        info!("Creating CSI server with config: {:?}", config);

        let identity_service =
            IdentityService::new(config.driver.name.clone(), config.driver.version.clone());

        let node_service = NodeService::new(config.driver.node_id.clone());
        let controller_service = ControllerService::new();

        Ok(Self {
            identity_service,
            node_service,
            controller_service,
        })
    }

    pub async fn start(&self, endpoint: &str) -> Result<()> {
        let socket_path = Path::new(endpoint.trim_start_matches("unix://"));

        if socket_path.exists() {
            info!("Removing existing socket file: {}", socket_path.display());
            fs::remove_file(socket_path).await?;
        }

        if let Some(parent) = socket_path.parent().filter(|parent| !parent.exists()) {
            info!("Creating socket directory: {}", parent.display());
            fs::create_dir_all(parent).await?;
        }

        info!("Binding to Unix socket: {}", socket_path.display());
        let uds = UnixListener::bind(socket_path)?;
        let uds_stream = UnixListenerStream::new(uds);

        info!("CSI gRPC server listening on {}", socket_path.display());
        info!("Ready to accept CSI requests");

        Server::builder()
            .add_service(IdentityServer::new(self.identity_service.clone()))
            .add_service(NodeServer::new(self.node_service.clone()))
            .add_service(ControllerServer::new(self.controller_service.clone()))
            .serve_with_incoming(uds_stream)
            .await
            .map_err(|e| {
                error!("gRPC server error: {}", e);
                anyhow::anyhow!("Failed to start gRPC server: {}", e)
            })?;

        Ok(())
    }
}
