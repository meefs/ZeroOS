use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

pub fn find_workspace_root() -> Result<PathBuf> {
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;

    let mut dir = current_dir.as_path();

    loop {
        let cargo_toml = dir.join("Cargo.toml");
        if cargo_toml.exists() {
            let content =
                std::fs::read_to_string(&cargo_toml).context("Failed to read Cargo.toml")?;
            if content.contains("[workspace]") {
                return Ok(dir.to_path_buf());
            }
        }

        dir = dir
            .parent()
            .context("Could not find workspace root (no Cargo.toml with [workspace])")?;
    }
}

pub fn get_target_directory(workspace_root: &PathBuf) -> Result<PathBuf> {
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version=1")
        .arg("--no-deps")
        .current_dir(workspace_root)
        .output()
        .context("Failed to run cargo metadata")?;

    if !output.status.success() {
        return Ok(workspace_root.join("target"));
    }

    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).context("Failed to parse cargo metadata")?;

    let target_dir = metadata["target_directory"]
        .as_str()
        .map(PathBuf::from)
        .unwrap_or_else(|| workspace_root.join("target"));

    Ok(target_dir)
}

pub fn detect_profile(args: &[String]) -> String {
    if args.contains(&"--release".to_string()) {
        "release".to_string()
    } else if let Some(i) = args.iter().position(|a| a == "--profile") {
        args.get(i + 1).cloned().unwrap_or("debug".to_string())
    } else {
        "debug".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_profile_default() {
        let args = vec![];
        assert_eq!(detect_profile(&args), "debug");
    }

    #[test]
    fn test_detect_profile_release() {
        let args = vec!["--release".to_string()];
        assert_eq!(detect_profile(&args), "release");
    }

    #[test]
    fn test_detect_profile_custom() {
        let args = vec!["--profile".to_string(), "custom".to_string()];
        assert_eq!(detect_profile(&args), "custom");
    }

    #[test]
    fn test_detect_profile_with_other_args() {
        let args = vec![
            "--features".to_string(),
            "std".to_string(),
            "--release".to_string(),
        ];
        assert_eq!(detect_profile(&args), "release");
    }
}
