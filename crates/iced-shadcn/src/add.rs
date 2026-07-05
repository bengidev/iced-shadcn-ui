use crate::cargo_merge::merge_iced_dependency;
use crate::config::Config;
use crate::error::CliError;
use crate::fetch::{fetch_text, registry_json_url, template_url};
use crate::patch::patch_mod_rs;
use crate::registry::Registry;
use crate::render::render_template;
use std::fs;
use std::path::{Path, PathBuf};

pub fn run(components: Vec<String>) -> Result<(), CliError> {
    let project_root = std::env::current_dir().map_err(|e| CliError::Io(e.to_string()))?;
    let (config, _) = Config::load_or_create(&project_root)
        .map_err(|e| CliError::Message(e.to_string()))?;

    let registry = load_registry(&config)?;
    let order = registry
        .resolve_add_order(&components)
        .map_err(|e| CliError::Message(e.to_string()))?;

    let ui_dir = project_root.join(&config.ui_path);
    fs::create_dir_all(&ui_dir).map_err(|e| CliError::Io(e.to_string()))?;

    let mod_path = ui_dir.join("mod.rs");
    let existing_mod = if mod_path.exists() {
        fs::read_to_string(&mod_path).map_err(|e| CliError::Io(e.to_string()))?
    } else {
        String::new()
    };

    let mut written = Vec::new();
    for name in &order {
        let item = registry
            .item(name)
            .ok_or_else(|| CliError::Message(format!("missing registry item: {name}")))?;
        for file in &item.files {
            let target = ui_dir.join(&file.target);
            if target.exists() {
                eprintln!("skip existing: {}", target.display());
                continue;
            }
            let template = load_template(&config, &file.path)?;
            let rendered = render_template(&template, &config);
            fs::write(&target, rendered).map_err(|e| CliError::Io(e.to_string()))?;
            written.push(file.target.clone());
        }
    }

    let module_names: Vec<&str> = order.iter().map(String::as_str).collect();
    let patched = patch_mod_rs(&existing_mod, &module_names);
    fs::write(&mod_path, patched).map_err(|e| CliError::Io(e.to_string()))?;

    let cargo_path = project_root.join("Cargo.toml");
    let cargo = fs::read_to_string(&cargo_path).map_err(|e| CliError::Io(e.to_string()))?;
    let features = registry.collect_features(&order);
    let merged = merge_iced_dependency(&cargo, &config.iced_version, &features);
    fs::write(&cargo_path, merged).map_err(|e| CliError::Io(e.to_string()))?;

    println!("Added: {}", written.join(", "));
    println!("Import with: use {}::*;", config.ui_module_path());
    Ok(())
}

fn load_registry(config: &Config) -> Result<Registry, CliError> {
    if let Ok(local) = std::env::var("ICED_SHADCN_REGISTRY_DIR") {
        let path = PathBuf::from(local).join("registry.json");
        let text = fs::read_to_string(path).map_err(|e| CliError::Io(e.to_string()))?;
        return serde_json::from_str(&text).map_err(|e| CliError::Message(e.to_string()));
    }
    let url = registry_json_url(&config.registry_url);
    let text = fetch_text(&url)?;
    serde_json::from_str(&text).map_err(|e| CliError::Message(e.to_string()))
}

fn load_template(config: &Config, template_path: &str) -> Result<String, CliError> {
    if let Ok(local) = std::env::var("ICED_SHADCN_REGISTRY_DIR") {
        let path = Path::new(&local).join(template_path);
        return fs::read_to_string(path).map_err(|e| CliError::Io(e.to_string()));
    }
    let url = template_url(&config.registry_url, template_path);
    fetch_text(&url)
}
