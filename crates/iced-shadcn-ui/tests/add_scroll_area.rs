use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn add_scroll_area_uses_snake_case_module_name() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname=\"demo\"\nversion=\"0.1.0\"\nedition=\"2024\"\n",
    )
    .unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::write(
        dir.path().join("src/main.rs"),
        "mod ui;\nfn main() {}\n",
    )
    .unwrap();

    let registry_root =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../registry");

    Command::cargo_bin("iced-shadcn")
        .unwrap()
        .current_dir(dir.path())
        .arg("add")
        .arg("scroll-area")
        .env("ICED_SHADCN_REGISTRY_DIR", registry_root)
        .assert()
        .success();

    let mod_rs = fs::read_to_string(dir.path().join("src/ui/mod.rs")).unwrap();
    assert!(mod_rs.contains("pub mod scroll_area;"));
    assert!(!mod_rs.contains("pub mod scroll-area;"));
}
