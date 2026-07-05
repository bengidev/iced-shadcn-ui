use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn list_prints_component_names() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname=\"demo\"\nversion=\"0.1.0\"\nedition=\"2024\"\n",
    )
    .unwrap();

    let registry_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../registry");

    Command::cargo_bin("iced-shadcn")
        .unwrap()
        .current_dir(dir.path())
        .arg("list")
        .env("ICED_SHADCN_REGISTRY_DIR", registry_root)
        .assert()
        .success()
        .stdout(predicates::str::contains("button"));
}
