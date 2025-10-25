use anyhow::Result;
use clap::Parser;
use tracing::{Level, error, info};
use tracing_subscriber::FmtSubscriber;

mod config;
mod csi_types;
mod lustre;
mod server;
mod services;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// CSI driver name
    #[arg(long, default_value = "klustrefs.csi.k8s.io", env = "DRIVER_NAME")]
    driver_name: String,

    /// Node ID for this instance
    #[arg(long, env = "KUBE_NODE_NAME")]
    node_id: String,

    /// Unix socket endpoint for CSI communication
    #[arg(
        long,
        default_value = "/var/lib/kubelet/plugins/klustrefs.csi.k8s.io/csi.sock",
        env = "CSI_ENDPOINT"
    )]
    endpoint: String,

    /// Log level (trace, debug, info, warn, error)
    #[arg(long, default_value = "info", env = "LOG_LEVEL")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize tracing subscriber
    setup_tracing(&args.log_level)?;

    // Log startup information
    info!(
        "Starting klustrefs CSI driver v{}",
        env!("CARGO_PKG_VERSION")
    );
    info!("Driver name: {}", args.driver_name);
    info!("Node ID: {}", args.node_id);
    info!("Endpoint: {}", args.endpoint);

    // Create configuration
    let config = config::Config::new(args.driver_name.clone(), args.node_id.clone());

    // Start the CSI gRPC server
    info!("Initializing CSI gRPC server...");
    let server = server::CSIServer::new(config)?;

    info!("Server starting on {}", args.endpoint);
    if let Err(e) = server.start(&args.endpoint).await {
        error!("Failed to start server: {}", e);
        return Err(e);
    }

    Ok(())
}

fn setup_tracing(log_level: &str) -> Result<()> {
    // Parse log level
    let level = match log_level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    // Build the subscriber
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .finish();

    // Set the global subscriber
    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
