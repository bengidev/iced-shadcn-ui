use crate::config::Config;
use crate::error::CliError;
use crate::fetch::{fetch_text, registry_json_url, template_url};
use crate::registry::Registry;
use std::fs;
use std::path::{Path, PathBuf};

pub fn load_registry(config: &Config) -> Result<Registry, CliError> {
    if let Ok(local) = std::env::var("ICED_SHADCN_REGISTRY_DIR") {
        let path = PathBuf::from(local).join("registry.json");
        let text = fs::read_to_string(path).map_err(|e| CliError::Io(e.to_string()))?;
        return serde_json::from_str(&text).map_err(|e| CliError::Message(e.to_string()));
    }
    let url = registry_json_url(&config.registry_url, &config.registry_branch);
    let text = fetch_text(&url)?;
    serde_json::from_str(&text).map_err(|e| CliError::Message(e.to_string()))
}

pub fn load_template(config: &Config, template_path: &str) -> Result<String, CliError> {
    if let Ok(local) = std::env::var("ICED_SHADCN_REGISTRY_DIR") {
        let path = Path::new(&local).join(template_path);
        return fs::read_to_string(path).map_err(|e| CliError::Io(e.to_string()));
    }
    let url = template_url(
        &config.registry_url,
        &config.registry_branch,
        template_path,
    );
    fetch_text(&url)
}
