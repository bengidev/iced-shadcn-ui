use serde::Deserialize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Deserialize)]
pub struct Registry {
    pub name: String,
    pub items: Vec<RegistryItem>,
}

#[derive(Debug, Deserialize)]
pub struct RegistryItem {
    pub name: String,
    #[serde(rename = "type")]
    pub item_type: String,
    #[serde(default)]
    pub files: Vec<RegistryFile>,
    #[serde(default, rename = "registryDependencies")]
    pub registry_dependencies: Vec<String>,
    #[serde(default)]
    pub features: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct RegistryFile {
    pub path: String,
    pub target: String,
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("unknown component: {0}")]
    UnknownComponent(String),
    #[error("circular dependency involving: {0}")]
    CircularDependency(String),
}

impl Registry {
    pub fn item(&self, name: &str) -> Option<&RegistryItem> {
        self.items.iter().find(|i| i.name == name)
    }

    pub fn resolve_add_order(&self, requested: &[String]) -> Result<Vec<String>, RegistryError> {
        let mut seen = HashSet::new();
        let mut visiting = HashSet::new();
        let mut order = Vec::new();
        let map: HashMap<&str, &RegistryItem> =
            self.items.iter().map(|i| (i.name.as_str(), i)).collect();

        for name in requested {
            Self::visit(name, &map, &mut seen, &mut visiting, &mut order)?;
        }
        Ok(order)
    }

    fn visit<'a>(
        name: &str,
        map: &HashMap<&str, &'a RegistryItem>,
        seen: &mut HashSet<String>,
        visiting: &mut HashSet<String>,
        order: &mut Vec<String>,
    ) -> Result<(), RegistryError> {
        if seen.contains(name) {
            return Ok(());
        }
        if !visiting.insert(name.to_string()) {
            return Err(RegistryError::CircularDependency(name.into()));
        }

        let item = map
            .get(name)
            .ok_or_else(|| RegistryError::UnknownComponent(name.into()))?;
        for dep in &item.registry_dependencies {
            Self::visit(dep, map, seen, visiting, order)?;
        }

        visiting.remove(name);
        seen.insert(name.to_string());
        order.push(name.to_string());
        Ok(())
    }

    pub fn collect_features(&self, names: &[String]) -> Vec<String> {
        let mut features = HashSet::new();
        for name in names {
            if let Some(item) = self.item(name) {
                features.extend(item.features.iter().cloned());
            }
        }
        features.insert("advanced".into());
        features.insert("tokio".into());
        features.insert("wgpu".into());
        let mut out: Vec<_> = features.into_iter().collect();
        out.sort();
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{
        "name": "iced-shadcn",
        "items": [
            {"name": "theme", "type": "registry:theme", "files": [], "registryDependencies": []},
            {"name": "button", "type": "registry:component", "files": [], "registryDependencies": ["theme"]}
        ]
    }"#;

    #[test]
    fn resolves_transitive_dependencies() {
        let registry: Registry = serde_json::from_str(SAMPLE).unwrap();
        let ordered = registry.resolve_add_order(&["button".to_string()]).unwrap();
        assert_eq!(ordered, vec!["theme", "button"]);
    }
}
