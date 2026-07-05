use crate::cargo_merge::{merge_iced_dependency, merge_lucide_icons_dependency};
use crate::config::Config;
use crate::error::CliError;
use crate::patch::patch_mod_rs;
use crate::registry_load::{load_registry, load_template};
use crate::render::render_template;
use std::fs;

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
    let mut merged = merge_iced_dependency(&cargo, &config.iced_version, &features);
    if order.iter().any(|name| name == "icons") {
        merged = merge_lucide_icons_dependency(&merged);
    }
    fs::write(&cargo_path, merged).map_err(|e| CliError::Io(e.to_string()))?;

    println!("Added: {}", written.join(", "));
    println!("Import with: use {}::*;", config.ui_module_path());
    Ok(())
}
