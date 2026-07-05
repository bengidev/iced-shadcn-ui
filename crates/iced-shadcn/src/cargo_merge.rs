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
