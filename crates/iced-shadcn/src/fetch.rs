use crate::error::CliError;
use std::path::Path;

pub fn registry_json_url(registry_url: &str) -> String {
    format!(
        "{}/main/registry/registry.json",
        registry_url.replace("https://github.com/", "https://raw.githubusercontent.com/")
    )
}

pub fn template_url(registry_url: &str, template_path: &str) -> String {
    format!(
        "{}/main/registry/{}",
        registry_url.replace("https://github.com/", "https://raw.githubusercontent.com/"),
        template_path
    )
}

pub fn fetch_text(url: &str) -> Result<String, CliError> {
    reqwest::blocking::get(url)
        .map_err(|e| CliError::Network(e.to_string()))?
        .error_for_status()
        .map_err(|e| CliError::Network(e.to_string()))?
        .text()
        .map_err(|e| CliError::Network(e.to_string()))
}

pub fn load_registry_from_disk(path: &Path) -> Result<String, CliError> {
    std::fs::read_to_string(path).map_err(|e| CliError::Io(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_raw_github_urls() {
        let url = registry_json_url("https://github.com/bengidev/iced-shadcn-ui");
        assert_eq!(
            url,
            "https://raw.githubusercontent.com/bengidev/iced-shadcn-ui/main/registry/registry.json"
        );
        let tpl = template_url(
            "https://github.com/bengidev/iced-shadcn-ui",
            "styles/new-york/button.rs.template",
        );
        assert!(tpl.ends_with("/registry/styles/new-york/button.rs.template"));
    }
}
