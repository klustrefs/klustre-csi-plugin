use anyhow::{Context, Result};
use std::process::Command;
use tracing::{debug, info, warn};

/// Manages Lustre filesystem mount operations
#[derive(Debug, Clone)]
pub struct MountManager;

impl MountManager {
    pub fn new() -> Self {
        Self
    }

    /// Mount a Lustre filesystem using nsenter to execute on host
    pub async fn mount(
        &self,
        source: &str,       // e.g., "192.168.1.10@tcp0:/lustre"
        target: &str,       // e.g., "/var/lib/kubelet/pods/.../volumes/..."
        options: &[String], // e.g., ["flock", "user_xattr"]
    ) -> Result<()> {
        info!("Mounting Lustre: {} -> {}", source, target);

        // Create target directory on host using nsenter
        self.ensure_mount_point(target).await?;

        // Check if already mounted
        if self.is_mounted(target).await? {
            info!("Target {} is already mounted", target);
            return Ok(());
        }

        // Build mount options
        let mount_opts = options.to_vec();
        let opts_str = mount_opts.join(",");

        // Use nsenter to execute mount.lustre on the host
        // -t 1: target PID 1 (init/systemd on host)
        // -m: enter mount namespace
        let mut cmd = Command::new("nsenter");
        cmd.arg("-t")
            .arg("1")
            .arg("-m")
            .arg("/usr/sbin/mount.lustre");

        if !opts_str.is_empty() {
            cmd.arg("-o").arg(&opts_str);
        }

        cmd.arg(source).arg(target);

        debug!("Executing mount command: {:?}", cmd);

        // Execute mount
        let output = cmd.output().context("Failed to execute mount command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            anyhow::bail!(
                "Mount failed (exit code: {})\nStderr: {}\nStdout: {}",
                output.status.code().unwrap_or(-1),
                stderr,
                stdout
            );
        }

        info!("Successfully mounted {} at {}", source, target);
        Ok(())
    }

    /// Unmount a Lustre filesystem using nsenter
    pub async fn unmount(&self, target: &str) -> Result<()> {
        info!("Unmounting: {}", target);

        // Check if mounted
        if !self.is_mounted(target).await? {
            warn!("Target {} is not mounted, nothing to do", target);
            return Ok(());
        }

        // Execute unmount using nsenter
        let output = Command::new("nsenter")
            .arg("-t")
            .arg("1")
            .arg("-m")
            .arg("umount")
            .arg(target)
            .output()
            .context("Failed to execute umount command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Unmount failed: {}", stderr);
        }

        info!("Successfully unmounted {}", target);
        Ok(())
    }

    /// Check if a path is already mounted on the host
    async fn is_mounted(&self, target: &str) -> Result<bool> {
        let output = Command::new("nsenter")
            .arg("-t")
            .arg("1")
            .arg("-m")
            .arg("findmnt")
            .arg("-o")
            .arg("TARGET")
            .arg("-n")
            .arg(target)
            .output()
            .context("Failed to check mount status")?;

        Ok(output.status.success() && !output.stdout.is_empty())
    }

    /// Ensure mount point directory exists on the host
    async fn ensure_mount_point(&self, target: &str) -> Result<()> {
        // Check if directory exists on host using nsenter
        let check_output = Command::new("nsenter")
            .arg("-t")
            .arg("1")
            .arg("-m")
            .arg("test")
            .arg("-d")
            .arg(target)
            .output()
            .context("Failed to check if mount point exists")?;

        if !check_output.status.success() {
            debug!("Creating mount point on host: {}", target);

            // Create directory on host using nsenter
            let mkdir_output = Command::new("nsenter")
                .arg("-t")
                .arg("1")
                .arg("-m")
                .arg("mkdir")
                .arg("-p")
                .arg(target)
                .output()
                .context("Failed to create mount point on host")?;

            if !mkdir_output.status.success() {
                let stderr = String::from_utf8_lossy(&mkdir_output.stderr);
                anyhow::bail!("Failed to create mount point: {}", stderr);
            }
        }

        Ok(())
    }
}
