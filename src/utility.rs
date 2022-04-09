use std::error::Error;
use std::fs;
use std::fs::File;
use std::future::Future;
use std::include_str;
use std::io::prelude::*;
use std::io::Cursor;
use std::path::Path;
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

pub fn modify_toml() {
    const PROJECT_TOML: &'static str =
        include_str!("/Users/przem/code/hal_tools/solana_poc_async_init/testing/Cargo.toml");
    let POC_TOML: &'static str =
        include_str!("/Users/przem/code/hal_tools/solana_poc_async_init/poc/Cargo.toml");

    let mut project_toml_parsed = PROJECT_TOML.parse::<Document>().unwrap();
    let mut poc_toml_parsed = POC_TOML.parse::<Document>().unwrap();

    //Check if it's Anchor project
    if project_toml_parsed.get("dependencies").is_some()
        && project_toml_parsed["dependencies"]
            .get("anchor-lang")
            .is_some()
        && project_toml_parsed["dependencies"]["anchor-lang"]
            .get("version")
            .is_some()
    {
        let anchor_version = &project_toml_parsed["dependencies"]["anchor-lang"]["version"]
            .as_str()
            .expect("Cannot parse version");
        poc_toml_parsed["dependencies"]["anchor-lang"]["version"] = value(*anchor_version);
    }

    //Adding test-bpf feature
    let is_features = project_toml_parsed.get("features").is_some();

    if is_features && project_toml_parsed["features"].get("test-bpf").is_none() {
        let features = project_toml_parsed["features"].as_str().expect("ToDO");
        let bpf = r#"
test-bpf = []
"#;
        let concat = format!("{}{}", features, bpf);
        project_toml_parsed["features"] = value(concat);
    };

    let poc_toml_string = poc_toml_parsed.to_string();
    let mut project_toml_string = project_toml_parsed.to_string();

    if !is_features {
        let features_string = r#"
[features]
test-bpf = []
"#;
        project_toml_string.push_str(features_string);
    }

    // Add framework as dependency
    let dev_dependency = r#"
[dev-dependencies.solana-poc-async]
solana-poc-async = { version = "0.1.1", path = "../../../solana-poc-async" }"#;

    project_toml_string.push_str(dev_dependency);
    fs::write("servers.toml", poc_toml_string).expect("Could not write to file!");
    fs::write("servers2.toml", project_toml_string).expect("Could not write to file!");
}
