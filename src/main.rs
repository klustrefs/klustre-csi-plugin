use anyhow::Result;
use clap::Parser;
use std::io::IsTerminal;
use tracing::{error, info};
use tracing_subscriber::layer::Layer;
mod config;
mod csi_types;
mod lustre;
mod server;
mod services;
mod utils;

#[derive(Parser, Debug)]
#[command(name = "klustrefs-csi-plugin", author, version, about, long_about = None)]
struct Args {
    /// CSI driver name
    #[arg(long, default_value = "lustre.csi.klustrefs.io", env = "DRIVER_NAME")]
    driver_name: String,

    /// Node ID for this instance
    #[arg(long, env = "KUBE_NODE_NAME")]
    node_id: String,

    /// Unix socket endpoint for CSI communication
    #[arg(
        long,
        default_value = "/var/lib/kubelet/plugins/lustre.csi.klustrefs.io/csi.sock",
        env = "CSI_ENDPOINT"
    )]
    endpoint: String,

    /// Log level (trace, debug, info, warn, error)
    #[arg(long, default_value = "info", env = "LOG_LEVEL")]
    log_level: String,

    #[arg(long, default_value = "plain", env = "LOG_FORMAT")]
    log_format: String,

    /// read RUST_LOG if present
    #[arg(long, default_value = "", env = "RUST_LOG")]
    _ignored_rust_log: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize tracing subscriber
    setup_tracing(&args.log_level, &args.log_format)?;

    // Log startup information
    info!(
        "Starting klustrefs-csi-plugin v{}",
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

pub fn setup_tracing(log_level: &str, log_format: &str) -> Result<()> {
    use std::io;
    use tracing_error::ErrorLayer;
    use tracing_subscriber::{
        EnvFilter, Registry, fmt, layer::SubscriberExt, util::SubscriberInitExt,
    };

    // Build filter from RUST_LOG/LOG_LEVEL; fallback to provided level.
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(format!("{}{}", "info,", log_level)))
        .unwrap_or_else(|_| EnvFilter::new("info"));

    // Write logs to stdout
    let is_tty = io::stdout().is_terminal();

    // Common formatter options (writer, metadata, timestamp).
    let base = fmt::layer()
        .with_writer(io::stdout)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(true)
        .with_ansi(is_tty) // no ANSI in non-TTY (pods/log shippers)
        .with_timer(fmt::time::UtcTime::rfc_3339());

    let fmt_layer = if log_format.eq_ignore_ascii_case("json") {
        base.json()
            .flatten_event(true)
            .with_current_span(true)
            .with_span_list(true)
            .boxed()
    } else {
        base.compact().boxed() // single-line compact text
    };

    // Compose registry + filters + formatter + error contexts.
    Registry::default()
        .with(filter)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();

    Ok(())
}
