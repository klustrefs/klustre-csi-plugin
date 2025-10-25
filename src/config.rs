use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main configuration for the klustrefs CSI driver
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Driver information
    pub driver: DriverConfig,

    /// Lustre-specific configuration
    pub lustre: LustreConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverConfig {
    /// CSI driver name
    pub name: String,

    /// Driver version
    pub version: String,

    /// Node ID where this driver instance is running
    pub node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LustreConfig {
    /// Default mount options for Lustre filesystems
    pub default_mount_options: Vec<String>,

    /// Mapping of filesystem names to MGS addresses
    pub filesystem_mapping: HashMap<String, String>,
}

impl Config {
    /// Create a new configuration with sensible defaults
    pub fn new(driver_name: String, node_id: String) -> Self {
        Self {
            driver: DriverConfig {
                name: driver_name,
                version: env!("CARGO_PKG_VERSION").to_string(),
                node_id,
            },
            lustre: LustreConfig {
                default_mount_options: vec!["flock".to_string(), "user_xattr".to_string()],
                filesystem_mapping: HashMap::new(),
            },
        }
    }
}
