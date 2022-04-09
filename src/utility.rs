use std::error::Error;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use toml_edit::{table, value, Array, Document};
use zip;

pub async fn download_poc_framework(path: &str, url: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    if response.status() == 404 {
        eprintln!("Incorect POC Framework URL")
    }
    let bytes = response.bytes().await?;
    let content = Cursor::new(bytes);
    let mut zip = zip::ZipArchive::new(content)?;
    zip.extract(path)?;
    Ok(())
}

pub fn get_path_to_framework(
    path_to_framework_dir: &str,
    framework_name: &str,
) -> Result<PathBuf, String> {
    for entry in fs::read_dir(path_to_framework_dir).expect("Cannot get dir") {
        let entry = entry.expect("cannot get entry");
        let path = entry.path();
        let path_as_string = path
            .clone()
            .into_os_string()
            .into_string()
            .expect("Cannot get path");
        if path.is_dir() && path_as_string.contains(framework_name) {
            return Ok(path);
        }
    }
    Err("Cannot find downloaded POC framework".to_string())
}

pub fn is_correct_cargo_toml(project_toml: Document) -> bool {
    if project_toml.get("package").is_some() {
        return true;
    }
    false
}

pub fn set_anchor_for_framework(project_toml: &Document, mut poc_toml: Document) -> Document {
    if project_toml.get("dependencies").is_some()
        && project_toml["dependencies"].get("anchor-lang").is_some()
    {
        if project_toml["dependencies"]["anchor-lang"]
            .get("version")
            .is_some()
        {
            let anchor_version = &project_toml["dependencies"]["anchor-lang"]["version"]
                .as_str()
                .expect("Cannot parse anchor version");
            poc_toml["dependencies"]["anchor-lang"]["version"] = value(*anchor_version)
        } else {
            let anchor_version = &project_toml["dependencies"]["anchor-lang"]
                .as_str()
                .expect("Cannot parse anchor version");
            poc_toml["dependencies"]["anchor-lang"]["version"] = value(*anchor_version)
        }
    }
    poc_toml
}

pub fn add_test_bpf_feature(mut project_toml: Document) -> Document {
    let is_features = project_toml.get("features").is_some();

    if is_features && project_toml["features"].get("test-bpf").is_some() {
        return project_toml;
    }

    if !is_features {
        project_toml["features"] = table();
    }
    let empty_arr = Array::default();
    project_toml["features"]["test_bpf"] = value(empty_arr);
    project_toml
}

pub fn add_framework_as_dev_dependency(
    mut project_toml: Document,
    path_to_framework: &str,
    framework_version: &str,
    framework_name: &str,
) -> Document {
    if project_toml.get("dev-dependnecies").is_none() {
        project_toml["dev-dependencies"] = table();
    }
    project_toml["dev-dependencies"][framework_name]["version"] = value(framework_version);
    project_toml["dev-dependencies"][framework_name]["path"] = value(path_to_framework);
    project_toml
}

pub fn get_framework_version(poc_framework: &Document) -> String {
    String::from(
        poc_framework["package"]["version"]
            .as_str()
            .expect("Cannot unpack framework version"),
    )
}

pub fn save_toml(toml: Document, path: &str) {
    let contents = toml.to_string();
    fs::write(path, contents).expect("Could not write to file!");
}

pub const TEST_TEMPLATE: &str = r#"#![cfg(feature = "test-bpf")]

use solana_poc_async::*;

#[tokio::test]
async fn poc() {
    assert_eq!(1, 1);
}
"#;
