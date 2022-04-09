use std::error::Error;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use toml_edit::{value, Document};
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
    //Adding test-bpf feature
    let is_features = project_toml.get("features").is_some();

    if is_features && project_toml["features"].get("test-bpf").is_none() {
        let features = project_toml["features"].as_str().expect("ToDO");
        let bpf = r#"
    test-bpf = []
    "#;
        let concat = format!("{}{}", features, bpf);
        project_toml["features"] = value(concat);
    };

    if !is_features {
        let mut project_toml_string = project_toml.to_string();
        project_toml_string.push_str(FEATURES_TEMPLATE);
        project_toml_string.parse::<Document>().unwrap()
    } else {
        project_toml
    }
}

pub fn add_framework_as_dev_dependency(
    mut project_toml: Document,
    path_to_framework: &str,
    framework_version: &str,
    framework_name: &str,
) -> Document {
    // Add framework as dependency
    if project_toml.get("dev-dependencies").is_some()
        && project_toml["dev-dependencies"]
            .get(framework_name)
            .is_some()
    {
        project_toml["dev-dependencies"][framework_name]["version"] = value(framework_version);
        project_toml["dev-dependencies"][framework_name]["path"] = value(path_to_framework);
        project_toml
    } else {
        let mut project_toml_string = project_toml.to_string();
        let finished = POC_DEPENDENCY_TEMPLATE
            .replace("VERSION", framework_version)
            .replace("PATH", path_to_framework)
            .replace("FRAMEWORK_NAME", framework_name);
        println!("FINISHED {}", finished);
        project_toml_string.push_str(finished.as_str());
        project_toml_string.parse::<Document>().unwrap()
    }
}

pub fn get_framework_version(poc_framework: &Document) -> String {
    poc_framework["package"]["version"]
        .to_string()
        .replace("\"", "")
        .replace(" ", "")
}

pub fn save_toml(toml: Document, path: &str) {
    let contents = toml.to_string();
    fs::write(path, contents).expect("Could not write to file!");
}

pub const POC_DEPENDENCY_TEMPLATE: &str = r#"
[dev-dependencies.FRAMEWORK_NAME]
version = "VERSION"
path = "PATH""#;

const FEATURES_TEMPLATE: &str = r#"
[features]
test-bpf = []
"#;
