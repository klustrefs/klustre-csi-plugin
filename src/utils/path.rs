/// Validate a volume path
#[allow(dead_code)]
pub fn validate_volume_path(path: &str) -> Result<(), String> {
    if path.is_empty() {
        return Err("Path cannot be empty".to_string());
    }
    Ok(())
}
