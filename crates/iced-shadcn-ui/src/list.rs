use crate::config::Config;
use crate::error::CliError;
use crate::registry_load::load_registry;

pub fn run() -> Result<(), CliError> {
    let project_root = std::env::current_dir().map_err(|e| CliError::Io(e.to_string()))?;
    let config = Config::load(&project_root).map_err(|e| CliError::Message(e.to_string()))?;
    let registry = load_registry(&config)?;
    for item in &registry.items {
        if item.item_type.starts_with("registry:component") {
            println!("{}", item.name);
        }
    }
    Ok(())
}
