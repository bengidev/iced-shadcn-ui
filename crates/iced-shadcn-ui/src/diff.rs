use crate::config::Config;
use crate::error::CliError;
use crate::registry_load::{load_registry, load_template};
use crate::render::render_template;
use similar::TextDiff;
use std::fs;

pub fn run(component: String) -> Result<(), CliError> {
    let project_root = std::env::current_dir().map_err(|e| CliError::Io(e.to_string()))?;
    let config = Config::load(&project_root).map_err(|e| CliError::Message(e.to_string()))?;
    let registry = load_registry(&config)?;
    let item = registry
        .item(&component)
        .ok_or_else(|| CliError::Message(format!("unknown component: {component}")))?;

    let ui_dir = project_root.join(&config.ui_path);
    for file in &item.files {
        let target = ui_dir.join(&file.target);
        let local = if target.exists() {
            fs::read_to_string(&target).map_err(|e| CliError::Io(e.to_string()))?
        } else {
            String::new()
        };
        let template = load_template(&config, &file.path)?;
        let remote = render_template(&template, &config);

        let diff = TextDiff::from_lines(&local, &remote);
        let header_local = target.display().to_string();
        let header_remote = format!("{component} (template)");
        print!(
            "{}",
            diff.unified_diff()
                .context_radius(3)
                .header(&header_local, &header_remote)
        );
    }
    Ok(())
}
