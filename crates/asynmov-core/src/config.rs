use serde::Deserialize;
use thiserror::Error;

use crate::generators::entities::AttrSpec;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("TOML parse error: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("missing [world] section")]
    MissingWorld,
}

/// Top-level config parsed from a `.toml` file.
#[derive(Debug, Deserialize)]
pub struct WorldConfig {
    pub world: WorldMeta,
    #[serde(default)]
    pub attributes: Vec<AttrSpec>,
}

#[derive(Debug, Deserialize)]
pub struct WorldMeta {
    pub name: String,
    pub seed: Option<u64>,
}

impl WorldConfig {
    pub fn from_toml(src: &str) -> Result<Self, ConfigError> {
        let cfg: WorldConfig = toml::from_str(src)?;
        Ok(cfg)
    }
}
