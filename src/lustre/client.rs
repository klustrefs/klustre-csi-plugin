#![allow(dead_code)]
use anyhow::{Context, Result};
use std::process::Command;
use tracing::{debug, info, warn};

/// Lustre client utilities and health checks
#[derive(Debug, Clone)]
pub struct LustreClient;

impl LustreClient {
    pub fn new() -> Self {
        Self
    }

    /// Check if Lustre client kernel module is loaded
    pub fn is_lustre_available(&self) -> Result<bool> {
        debug!("Checking if Lustre kernel module is loaded");

        let output = Command::new("lsmod")
            .output()
            .context("Failed to execute lsmod")?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        let is_loaded = output_str.contains("lustre");

        if is_loaded {
            info!("Lustre kernel module is loaded");
        } else {
            warn!("Lustre kernel module is NOT loaded");
        }

        Ok(is_loaded)
    }

    /// Load Lustre kernel module if not already loaded
    pub fn ensure_lustre_loaded(&self) -> Result<()> {
        if self.is_lustre_available()? {
            return Ok(());
        }

        info!("Attempting to load Lustre kernel module");

        let output = Command::new("modprobe")
            .arg("lustre")
            .output()
            .context("Failed to execute modprobe")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to load Lustre module: {}", stderr);
        }

        info!("Lustre kernel module loaded successfully");
        Ok(())
    }

    /// Get Lustre filesystem info using lfs df
    pub fn get_fs_info(&self, mount_point: &str) -> Result<LustreFilesystemInfo> {
        debug!("Getting filesystem info for: {}", mount_point);

        let output = Command::new("lfs")
            .args(["df", "-h", mount_point])
            .output()
            .context("Failed to execute lfs df")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("lfs df failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        self.parse_lfs_df_output(&stdout)
    }

    /// Parse output from lfs df
    fn parse_lfs_df_output(&self, output: &str) -> Result<LustreFilesystemInfo> {
        let mut total_bytes = 0u64;
        let mut used_bytes = 0u64;

        for line in output.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                // Try to parse size columns (rough estimation)
                if let Some(size_str) = parts.get(1) {
                    if let Some(bytes) = parse_size_string(size_str) {
                        total_bytes += bytes;
                    }
                }
                if let Some(used_str) = parts.get(2) {
                    if let Some(bytes) = parse_size_string(used_str) {
                        used_bytes += bytes;
                    }
                }
            }
        }

        Ok(LustreFilesystemInfo {
            total_bytes,
            used_bytes,
            available_bytes: total_bytes.saturating_sub(used_bytes),
        })
    }

    /// Check if a Lustre mount is healthy
    pub fn check_mount_health(&self, mount_point: &str) -> Result<bool> {
        debug!("Checking mount health: {}", mount_point);

        // Try to stat the mount point
        let output = Command::new("stat")
            .arg(mount_point)
            .output()
            .context("Failed to stat mount point")?;

        if !output.status.success() {
            warn!("Mount point {} is not accessible", mount_point);
            return Ok(false);
        }

        // Try to list directory (basic read test)
        let output = Command::new("ls")
            .arg("-la")
            .arg(mount_point)
            .output()
            .context("Failed to list mount point")?;

        if !output.status.success() {
            warn!("Cannot list mount point {}", mount_point);
            return Ok(false);
        }

        info!("Mount point {} is healthy", mount_point);
        Ok(true)
    }

    /// Get Lustre version
    pub fn get_lustre_version(&self) -> Result<String> {
        debug!("Getting Lustre version");

        let output = Command::new("lfs")
            .arg("--version")
            .output()
            .context("Failed to get Lustre version")?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("Failed to get Lustre version"));
        }

        let version = String::from_utf8_lossy(&output.stdout)
            .lines()
            .next()
            .unwrap_or("unknown")
            .to_string();

        info!("Lustre version: {}", version);
        Ok(version)
    }

    /// Validate Lustre source format (MGS@network:/fsname)
    pub fn validate_source(&self, source: &str) -> Result<()> {
        debug!("Validating Lustre source: {}", source);

        if !source.contains('@') {
            anyhow::bail!(
                "Invalid Lustre source: missing '@' (expected format: mgs@network:/fsname)"
            );
        }

        if !source.contains(":/") {
            anyhow::bail!(
                "Invalid Lustre source: missing ':/' (expected format: mgs@network:/fsname)"
            );
        }

        // Split and validate parts
        let parts: Vec<&str> = source.split(":/").collect();
        if parts.len() != 2 {
            anyhow::bail!("Invalid Lustre source format");
        }

        let mgs_part = parts[0];
        let fs_name = parts[1];

        if mgs_part.is_empty() || fs_name.is_empty() {
            anyhow::bail!("Invalid Lustre source: empty MGS or filesystem name");
        }

        info!("Lustre source validated: {}", source);
        Ok(())
    }
}

/// Lustre filesystem information
#[derive(Debug, Clone)]
pub struct LustreFilesystemInfo {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
}

impl LustreFilesystemInfo {
    pub fn usage_percent(&self) -> f64 {
        if self.total_bytes == 0 {
            return 0.0;
        }
        (self.used_bytes as f64 / self.total_bytes as f64) * 100.0
    }
}

/// Parse size string like "96.0G", "1.0M", etc. to bytes
fn parse_size_string(size_str: &str) -> Option<u64> {
    let size_str = size_str.trim();

    // Find where the number ends and unit begins
    let (num_part, unit_part) = size_str
        .char_indices()
        .find(|(_, c)| c.is_alphabetic())
        .map(|(idx, _)| size_str.split_at(idx))
        .unwrap_or((size_str, ""));

    let number: f64 = num_part.parse().ok()?;

    let multiplier: u64 = match unit_part.to_uppercase().as_str() {
        "K" | "KB" => 1024,
        "M" | "MB" => 1024 * 1024,
        "G" | "GB" => 1024 * 1024 * 1024,
        "T" | "TB" => 1024 * 1024 * 1024 * 1024, // Use u64 type above
        "" => 1,                                 // Bytes
        _ => return None,
    };

    Some((number * multiplier as f64) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size_string() {
        assert_eq!(parse_size_string("1024"), Some(1024));
        assert_eq!(parse_size_string("1K"), Some(1024));
        assert_eq!(parse_size_string("1M"), Some(1024 * 1024));
        assert_eq!(parse_size_string("1G"), Some(1024 * 1024 * 1024));
        assert_eq!(parse_size_string("96.0G"), Some(96 * 1024 * 1024 * 1024));
        assert_eq!(parse_size_string("3.0M"), Some(3 * 1024 * 1024));
    }
}
