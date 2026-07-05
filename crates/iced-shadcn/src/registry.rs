use serde::Deserialize;
use std::collections::{HashMap, HashSet, VecDeque};

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
        let mut order = Vec::new();
        let map: HashMap<&str, &RegistryItem> =
            self.items.iter().map(|i| (i.name.as_str(), i)).collect();

        for name in requested {
            Self::visit(name, &map, &mut seen, &mut order)?;
        }
        Ok(order)
    }

    fn visit<'a>(
        name: &str,
        map: &HashMap<&str, &'a RegistryItem>,
        seen: &mut HashSet<String>,
        order: &mut Vec<String>,
    ) -> Result<(), RegistryError> {
        if seen.contains(name) {
            return Ok(());
        }
        let item = map
            .get(name)
            .ok_or_else(|| RegistryError::UnknownComponent(name.into()))?;

        let mut stack = vec![name.to_string()];
        let mut visiting = HashSet::new();
        let mut queue = VecDeque::from([name.to_string()]);

        while let Some(current) = queue.pop_front() {
            if seen.contains(&current) {
                continue;
            }
            if visiting.contains(&current) {
                return Err(RegistryError::CircularDependency(current));
            }
            visiting.insert(current.clone());
            let current_item = map
                .get(current.as_str())
                .ok_or_else(|| RegistryError::UnknownComponent(current.clone()))?;
            for dep in &current_item.registry_dependencies {
                if !seen.contains(dep) {
                    queue.push_back(dep.clone());
                }
            }
            stack.push(current);
        }

        while let Some(current) = stack.pop() {
            if seen.insert(current.clone()) {
                order.push(current);
            }
        }
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
