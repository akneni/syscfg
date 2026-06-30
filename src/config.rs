use std::{fs, path::Path};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "AppConfig")]
    pub app_config: Vec<String>,
    #[serde(rename = "Font")]
    pub font: String,
    #[serde(rename = "Package")]
    pub package: Vec<String>,
}

impl Config {
    pub fn read(ss_path: &Path) -> Result<Self> {
        let path = ss_path.join("SnapshotConfig.json");
        let contents = fs::read_to_string(&path)
            .with_context(|| format!("failed to read config file {}", path.display()))?;

        serde_json::from_str(&contents)
            .with_context(|| format!("failed to parse config file {}", path.display()))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_config: Vec::new(),
            font: "*".to_string(),
            package: Vec::new(),
        }
    }
}
