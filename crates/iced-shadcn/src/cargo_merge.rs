pub fn merge_lucide_icons_dependency(existing: &str) -> String {
    if existing.contains("lucide-icons") {
        return existing.to_string();
    }

    let dep_line = "lucide-icons = { version = \"1\", features = [\"iced\"] }\n";
    if let Some(pos) = existing.find("[dependencies]") {
        let insert_at = existing[pos..]
            .find('\n')
            .map(|n| pos + n + 1)
            .unwrap_or(existing.len());
        let mut out = String::with_capacity(existing.len() + dep_line.len());
        out.push_str(&existing[..insert_at]);
        out.push_str(dep_line);
        out.push_str(&existing[insert_at..]);
        return out;
    }

    format!("{existing}\n[dependencies]\n{dep_line}")
}

pub fn merge_iced_dependency(existing: &str, version: &str, features: &[String]) -> String {
    if existing.contains("iced =") || existing.contains("iced=") {
        return existing.to_string();
    }
    let feature_list = features
        .iter()
        .map(|f| format!("\"{f}\""))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "{existing}\n[dependencies]\niced = {{ version = \"{version}\", features = [{feature_list}] }}\n"
    )
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
        let output = merge_lucide_icons_dependency(input);
        assert!(output.contains("lucide-icons = { version = \"1\""));
        assert!(output.contains("features = [\"iced\"]"));
    }

    #[test]
    fn adds_iced_dependency_when_missing() {
        let input = r#"[package]
name = "demo"
version = "0.1.0"
edition = "2024"
"#;
        let output = merge_iced_dependency(input, "0.14", &["advanced".into(), "wgpu".into()]);
        assert!(output.contains("iced = { version = \"0.14\""));
        assert!(output.contains("\"advanced\""));
        assert!(output.contains("\"wgpu\""));
    }
}
