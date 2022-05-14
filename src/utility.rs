use std::fs;
use toml_edit::{table, value, Array, Document};

pub fn is_correct_cargo_toml(project_toml: Document) -> bool {
    if project_toml.get("package").is_some() || project_toml.get("lib").is_some() {
        return true;
    }
    false
}

pub fn is_anchor(project_toml: &Document) -> bool {
    if project_toml.get("dependencies").is_some()
        && project_toml["dependencies"].get("anchor-lang").is_some()
    {
        return true;
    }
    false
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
    project_toml["features"]["test-bpf"] = value(empty_arr);
    project_toml
}

pub fn add_framework_as_dev_dependency(
    mut project_toml: Document,
    framework_url: &str,
    framework_branch: &str,
    framework_name: &str,
    is_anchor: bool,
) -> Document {
    if project_toml.get("dev-dependnecies").is_none() {
        project_toml["dev-dependencies"] = table();
    }
    project_toml["dev-dependencies"][framework_name]["git"] = value(framework_url);
    project_toml["dev-dependencies"][framework_name]["branch"] = value(framework_branch);

    if is_anchor {
        project_toml["dev-dependencies"][framework_name]["features"] = table();
        let mut arr = Array::default();
        arr.push("anchor");
        project_toml["dev-dependencies"][framework_name]["features"] = value(arr);
    }

    // Add solana test framework - it's crucial for framework to work
    // @todo when switch between solana version - this should be aligned
    project_toml["dev-dependencies"]["solana-program-test"]["version"] = value("1.9");

    project_toml
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
