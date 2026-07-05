use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn add_button_from_local_registry() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname=\"demo\"\nversion=\"0.1.0\"\nedition=\"2024\"\n",
    )
    .unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::write(dir.path().join("src/main.rs"), "fn main() {}\n").unwrap();

    let registry_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../registry");

    Command::cargo_bin("iced-shadcn")
        .unwrap()
        .current_dir(dir.path())
        .arg("add")
        .arg("button")
        .env("ICED_SHADCN_REGISTRY_DIR", registry_root)
        .assert()
        .success();

    assert!(dir.path().join("src/ui/button.rs").exists());
    assert!(dir.path().join("src/ui/theme.rs").exists());
    assert!(dir.path().join("iced-shadcn.toml").exists());
}
