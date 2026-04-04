use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowConfig {
    pub name: String,
    #[serde(default)]
    pub directory: Option<String>,
    #[serde(default)]
    pub editor_cmd: Option<String>,
    #[serde(default)]
    pub url_list: Option<Vec<String>>,
    #[serde(default = "default_shell")]
    pub shell: String,
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(default)]
    pub note: String,
}

fn default_shell() -> String {
    "/bin/sh".to_string()
}

impl FlowConfig {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.name.is_empty() {
            return Err(AppError::Config("Flow name cannot be empty".to_string()));
        }

        if self.name.contains('/') || self.name.contains('\\') {
            return Err(AppError::Config(
                "Flow name cannot contain / or \\".to_string(),
            ));
        }

        if let Some(ref dir) = self.directory {
            if dir.is_empty() {
                return Err(AppError::Config("Directory cannot be empty".to_string()));
            }
        }

        if let Some(ref cmd) = self.editor_cmd {
            if cmd.is_empty() {
                return Err(AppError::Config(
                    "Editor command cannot be empty".to_string(),
                ));
            }
        }

        if let Some(ref urls) = self.url_list {
            for url in urls {
                if !url.is_empty() && !is_valid_url(url) {
                    return Err(AppError::Config(format!("Invalid URL: {}", url)));
                }
            }
        }

        Ok(())
    }
}

fn is_valid_url(url: &str) -> bool {
    if url.starts_with("http://")
        || url.starts_with("https://")
        || url.starts_with("file://")
        || url.starts_with("ftp://")
    {
        return true;
    }
    if url.starts_with("localhost") || url.starts_with("127.0.0.1") || url.starts_with("0.0.0.0") {
        return true;
    }
    false
}

pub fn get_config_dir() -> Result<PathBuf, AppError> {
    let dir = dirs::config_dir()
        .ok_or_else(|| AppError::User("Could not find config directory".to_string()))?
        .join("flow");
    Ok(dir)
}

pub fn get_config_path(name: &str) -> Result<PathBuf, AppError> {
    let dir = get_config_dir()?;
    Ok(dir.join(format!("{}.json", name)))
}

pub fn get_lock_path(name: &str) -> Result<PathBuf, AppError> {
    let dir = get_config_dir()?;
    Ok(dir.join(format!("{}.lock", name)))
}

pub fn load_config(name: &str) -> Result<FlowConfig, AppError> {
    let path = get_config_path(name)?;
    let content =
        fs::read_to_string(&path).map_err(|e| AppError::Io(path.display().to_string(), e))?;
    let config: FlowConfig = serde_json::from_str(&content)
        .map_err(|e| AppError::Json(path.display().to_string(), e))?;
    Ok(config)
}

pub fn save_config(config: &FlowConfig) -> Result<(), AppError> {
    let dir = get_config_dir()?;
    fs::create_dir_all(&dir).map_err(|e| AppError::Io(dir.display().to_string(), e))?;
    let path = get_config_path(&config.name)?;
    let content = serde_json::to_string_pretty(config)
        .map_err(|e| AppError::Json(path.display().to_string(), e))?;
    fs::write(&path, content).map_err(|e| AppError::Io(path.display().to_string(), e))?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LockFile {
    pub pids: Vec<u32>,
}

pub fn write_lock_file(name: &str, pids: Vec<u32>) -> Result<(), AppError> {
    let dir = get_config_dir()?;
    fs::create_dir_all(&dir).map_err(|e| AppError::Io(dir.display().to_string(), e))?;
    let path = get_lock_path(name)?;
    let lock = LockFile { pids };
    let content =
        serde_json::to_string(&lock).map_err(|e| AppError::Json(path.display().to_string(), e))?;
    fs::write(&path, content).map_err(|e| AppError::Io(path.display().to_string(), e))?;
    Ok(())
}

pub fn read_lock_file(name: &str) -> Result<LockFile, AppError> {
    let path = get_lock_path(name)?;
    let content =
        fs::read_to_string(&path).map_err(|e| AppError::Io(path.display().to_string(), e))?;
    let lock: LockFile = serde_json::from_str(&content)
        .map_err(|e| AppError::Json(path.display().to_string(), e))?;
    Ok(lock)
}

pub fn delete_lock_file(name: &str) -> Result<(), AppError> {
    let path = get_lock_path(name)?;
    if path.exists() {
        fs::remove_file(&path).map_err(|e| AppError::Io(path.display().to_string(), e))?;
    }
    Ok(())
}

pub fn find_active_flow() -> Result<Option<String>, AppError> {
    let dir = get_config_dir()?;
    if !dir.exists() {
        return Ok(None);
    }

    let mut latest_mtime: Option<(std::time::SystemTime, String)> = None;

    for entry in fs::read_dir(&dir).map_err(|e| AppError::Io(dir.display().to_string(), e))? {
        let entry = entry.map_err(|e| AppError::Io(dir.display().to_string(), e))?;
        let path = entry.path();
        if path.extension().map(|e| e == "lock").unwrap_or(false) {
            if let Ok(metadata) = path.metadata() {
                if let Ok(mtime) = metadata.modified() {
                    let name = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .map(|s| s.to_string());
                    if let Some(name) = name {
                        match latest_mtime {
                            None => latest_mtime = Some((mtime, name)),
                            Some((ref latest_time, _)) if mtime > *latest_time => {
                                latest_mtime = Some((mtime, name));
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    Ok(latest_mtime.map(|(_, name)| name))
}

pub fn list_flows() -> Result<Vec<String>, AppError> {
    let dir = get_config_dir()?;
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut flows = Vec::new();
    for entry in fs::read_dir(&dir).map_err(|e| AppError::Io(dir.display().to_string(), e))? {
        let entry = entry.map_err(|e| AppError::Io(dir.display().to_string(), e))?;
        let path = entry.path();
        if path.extension().map(|e| e == "json").unwrap_or(false) {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                flows.push(name.to_string());
            }
        }
    }

    flows.sort();
    Ok(flows)
}
