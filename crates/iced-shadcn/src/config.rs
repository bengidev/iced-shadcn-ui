use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub const CONFIG_FILE: &str = "iced-shadcn.toml";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub style: String,
    pub base_color: String,
    pub ui_path: PathBuf,
    pub iced_version: String,
    pub registry_url: String,
    #[serde(default = "default_registry_branch")]
    pub registry_branch: String,
}

fn default_registry_branch() -> String {
    "main".into()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            style: "new-york".into(),
            base_color: "neutral".into(),
            ui_path: PathBuf::from("src/ui"),
            iced_version: "0.14".into(),
            registry_url: "https://github.com/bengidev/iced-shadcn-ui".into(),
            registry_branch: default_registry_branch(),
        }
    }
}

impl Config {
    pub fn ui_module_path(&self) -> String {
        self.ui_path
            .to_string_lossy()
            .trim_start_matches("src/")
            .replace('/', "::")
    }

    pub fn load(project_root: &Path) -> Result<Self, ConfigError> {
        let path = project_root.join(CONFIG_FILE);
        let contents = std::fs::read_to_string(&path)?;
        Ok(toml::from_str(&contents)?)
    }

    pub fn load_or_create(project_root: &Path) -> Result<(Self, PathBuf), ConfigError> {
        let path = project_root.join(CONFIG_FILE);
        if path.exists() {
            let contents = std::fs::read_to_string(&path)?;
            return Ok((toml::from_str(&contents)?, path));
        }
        let config = Config::default();
        std::fs::write(&path, toml::to_string_pretty(&config)?)?;
        Ok((config, path))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("toml error: {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("toml serialize error: {0}")]
    TomlSer(#[from] toml::ser::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_roundtrips_through_toml() {
        let config = Config::default();
        let serialized = toml::to_string(&config).unwrap();
        let parsed: Config = toml::from_str(&serialized).unwrap();
        assert_eq!(parsed.style, "new-york");
        assert_eq!(parsed.ui_path, PathBuf::from("src/ui"));
        assert_eq!(parsed.iced_version, "0.14");
        assert_eq!(parsed.registry_branch, "main");
    }
}
