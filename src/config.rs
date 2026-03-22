use anyhow::{Context, Result, anyhow};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const CONFIG_FILE_NAME: &str = "base_dir";

pub fn save_base_dir(dir: &Path) -> Result<PathBuf> {
    let config_file = config_file_path()?;
    let config_dir = config_file
        .parent()
        .ok_or_else(|| anyhow!("Failed to resolve config directory"))?;

    fs::create_dir_all(config_dir).with_context(|| {
        format!(
            "Failed to create config directory: {}",
            config_dir.display()
        )
    })?;

    fs::write(&config_file, dir.display().to_string())
        .with_context(|| format!("Failed to write config file: {}", config_file.display()))?;

    Ok(config_file)
}

pub fn load_base_dir() -> Result<Option<PathBuf>> {
    let config_file = config_file_path()?;
    if !config_file.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&config_file)
        .with_context(|| format!("Failed to read config file: {}", config_file.display()))?;
    let trimmed = content.trim();

    if trimmed.is_empty() {
        return Ok(None);
    }

    Ok(Some(PathBuf::from(trimmed)))
}

fn config_file_path() -> Result<PathBuf> {
    Ok(config_dir()?.join(CONFIG_FILE_NAME))
}

fn config_dir() -> Result<PathBuf> {
    if let Ok(xdg_config_home) = env::var("XDG_CONFIG_HOME")
        && !xdg_config_home.trim().is_empty()
    {
        return Ok(PathBuf::from(xdg_config_home).join("til"));
    }

    let home = env::var("HOME").context("Failed to resolve HOME for til config")?;
    Ok(PathBuf::from(home).join(".config").join("til"))
}
