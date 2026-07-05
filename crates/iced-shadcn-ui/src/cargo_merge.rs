use std::collections::BTreeSet;
use toml::Value;

pub fn merge_dependencies(
    existing: &str,
    iced_version: &str,
    features: &[String],
    add_lucide_icons: bool,
) -> Result<String, String> {
    let mut root: toml::Table = toml::from_str(existing).map_err(|e| e.to_string())?;

    let deps = root
        .entry("dependencies")
        .or_insert_with(|| Value::Table(toml::Table::new()));

    let deps_table = deps
        .as_table_mut()
        .ok_or_else(|| "dependencies is not a table".to_string())?;

    merge_iced_dep(deps_table, iced_version, features);
    if add_lucide_icons {
        merge_lucide_dep(deps_table);
    }

    toml::to_string_pretty(&root).map_err(|e| e.to_string())
}

fn merge_iced_dep(deps: &mut toml::Table, version: &str, features: &[String]) {
    if let Some(Value::Table(iced)) = deps.get_mut("iced") {
        merge_feature_array(iced, features);
        iced.entry("version".to_string())
            .or_insert(Value::String(version.into()));
        return;
    }

    let mut iced = toml::Table::new();
    iced.insert("version".into(), Value::String(version.into()));
    iced.insert("features".into(), features_to_array(features));
    deps.insert("iced".into(), Value::Table(iced));
}

fn merge_lucide_dep(deps: &mut toml::Table) {
    if deps.contains_key("lucide-icons") {
        return;
    }

    let mut lucide = toml::Table::new();
    lucide.insert("version".into(), Value::String("1".into()));
    lucide.insert(
        "features".into(),
        Value::Array(vec![Value::String("iced".into())]),
    );
    deps.insert("lucide-icons".into(), Value::Table(lucide));
}

fn merge_feature_array(table: &mut toml::Table, new_features: &[String]) {
    let mut set = BTreeSet::new();
    if let Some(Value::Array(arr)) = table.get("features") {
        for value in arr {
            if let Value::String(feature) = value {
                set.insert(feature.clone());
            }
        }
    }
    for feature in new_features {
        set.insert(feature.clone());
    }
    let merged: Vec<String> = set.into_iter().collect();
    table.insert("features".into(), features_to_array(&merged));
}

fn features_to_array(features: &[String]) -> Value {
    Value::Array(
        features
            .iter()
            .map(|feature| Value::String(feature.clone()))
            .collect(),
    )
}

pub fn module_name_from_target(target: &str) -> &str {
    target.strip_suffix(".rs").unwrap_or(target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_lucide_icons_dependency_when_missing() {
        let input = r#"[package]
name = "demo"
version = "0.1.0"
edition = "2024"

[dependencies]
iced = { version = "0.14", features = ["advanced"] }
"#;
        let output = merge_dependencies(input, "0.14", &[], true).unwrap();
        assert!(output.contains("lucide-icons"));
        assert!(output.contains("features = [\"iced\"]"));
    }

    #[test]
    fn adds_iced_dependency_when_missing() {
        let input = r#"[package]
name = "demo"
version = "0.1.0"
edition = "2024"
"#;
        let output =
            merge_dependencies(input, "0.14", &["advanced".into(), "wgpu".into()], false).unwrap();
        assert!(output.contains("iced"));
        assert!(output.contains("\"advanced\""));
        assert!(output.contains("\"wgpu\""));
    }

    #[test]
    fn merges_features_into_existing_iced_dependency() {
        let input = r#"[package]
name = "demo"
version = "0.1.0"
edition = "2024"

[dependencies]
iced = { version = "0.14", features = ["image"] }
"#;
        let output = merge_dependencies(
            input,
            "0.14",
            &["advanced".into(), "wgpu".into(), "tokio".into()],
            false,
        )
        .unwrap();
        assert!(output.contains("\"image\""));
        assert!(output.contains("\"advanced\""));
        assert!(output.contains("\"wgpu\""));
        assert!(output.contains("\"tokio\""));
    }

    #[test]
    fn derives_module_name_from_target() {
        assert_eq!(module_name_from_target("scroll_area.rs"), "scroll_area");
        assert_eq!(module_name_from_target("button.rs"), "button");
    }
}
