use semver::{Version, VersionReq};
use std::env;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use std::{error::Error, ops::Add};
use toml_edit::{table, value, Array, Document};
use zip;

#[derive(Debug, Clone)]
pub enum PocType {
    Mainnet,
    Devnet,
}

#[derive(Debug)]
pub struct SolanaVersion {
    pub poc_type: PocType,
    pub version: String,
}

impl PocType {
    pub fn get_type_from_version(version: VersionReq) -> Result<Self, String> {
        //@todo fix version comparing
        let mainnet = Version::parse("1.9.1000000").unwrap();
        let devnet = Version::parse("1.10.1000000").unwrap();

        if version.matches(&mainnet) {
            return Ok(Self::Mainnet);
        }

        if version.matches(&devnet) {
            return Ok(Self::Devnet);
        }

        Err("Not supported Solana version".to_string())
    }

    pub fn is_anchor_supported(&self) -> bool {
        match self {
            PocType::Mainnet => true,
            PocType::Devnet => false,
        }
    }

    pub fn get_branch_name(&self) -> String {
        match self {
            PocType::Mainnet => String::from("mainnet"),
            PocType::Devnet => String::from("devnet"),
        }
    }
}

impl SolanaVersion {
    pub fn set(version: String) -> SolanaVersion {
        let version_parsed =
            VersionReq::parse(version.as_str()).expect("Cannot parse Solana version");
        let poc_type =
            PocType::get_type_from_version(version_parsed).expect("Cannot get type from version");
        SolanaVersion { version, poc_type }
    }

    pub fn get_version(&self) -> String {
        self.version.clone()
    }
}

pub async fn download_poc_framework(
    path: &str,
    url: &str,
    framework_name: &str,
) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    if response.status() == 404 {
        eprintln!("Incorect POC Framework URL")
    }
    let bytes = response.bytes().await?;
    let content = Cursor::new(bytes);
    let mut zip = zip::ZipArchive::new(content)?;

    let mut tmp_folder = env::temp_dir();
    tmp_folder = tmp_folder.join("poc_temp");
    zip.extract(tmp_folder.clone())?;

    // by default it extracts to some folder - move this outside folder
    let extracted_dir = get_path_to_framework(tmp_folder.to_str().unwrap(), framework_name)
        .expect("Cannot get downloaded framework");
    fs::remove_dir_all(path)?;
    fs::rename(extracted_dir.clone(), path)?;
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

pub fn get_framework_url(base_url: String, poc_type: PocType) -> String {
    format!(
        "{}/archive/refs/heads/{}{}",
        base_url,
        poc_type.get_branch_name(),
        String::from(".zip")
    )
}

pub fn is_correct_cargo_toml(project_toml: Document) -> bool {
    if project_toml.get("package").is_some() || project_toml.get("lib").is_some() {
        return true;
    }
    false
}

pub fn set_anchor_for_framework(mut poc_toml: Document, anchor_version: String) -> Document {
    poc_toml["dependencies"]["anchor-lang"]["version"] = value(anchor_version);
    poc_toml
}

pub fn get_anchor_version(project_toml: &Document) -> Option<String> {
    if project_toml.get("dependencies").is_some()
        && project_toml["dependencies"].get("anchor-lang").is_some()
    {
        if project_toml["dependencies"]["anchor-lang"]
            .get("version")
            .is_some()
        {
            let anchor_version = &project_toml["dependencies"]["anchor-lang"]["version"]
                .as_str()
                .expect("Cannot parse anchor version. Try to specify anchor version using flag --anchor_version.");
            return Some(String::from(*anchor_version));
        } else {
            let anchor_version = &project_toml["dependencies"]["anchor-lang"]
                .as_str()
                .expect("Cannot parse anchor version. Try to specify anchor version using flag --anchor_version.");
            return Some(String::from(*anchor_version));
        }
    }
    None
}

pub fn get_solana_version(
    project_toml: Document,
    solana_dependencies: Vec<String>,
) -> Option<String> {
    if project_toml.get("dependencies").is_some() {
        for dependency in solana_dependencies.iter() {
            if project_toml["dependencies"].get(dependency).is_some() {
                if project_toml["dependencies"][dependency]
                    .get("version")
                    .is_some()
                {
                    return Some(
                        project_toml["dependencies"][dependency]["version"]
                            .as_str()
                            .expect("Cannot parse dependency version. Try to specify solana version using flag --solana_version")
                            .to_string(),
                    );
                } else {
                    return Some(
                        project_toml["dependencies"][dependency]
                            .as_str()
                            .expect("Cannot parse dependency version. Try to specify solana version using flag --solana_version.")
                            .to_string(),
                    );
                }
            }
        }
    }
    None
}

pub fn set_solana_for_framework(
    mut poc_toml: Document,
    solana_dependencies: Vec<String>,
    solana_version: String,
) -> Document {
    for dependency in solana_dependencies.iter() {
        poc_toml["dependencies"][dependency]["version"] = value(solana_version.clone());
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
    project_toml["features"]["test-bpf"] = value(empty_arr);
    project_toml
}

pub fn add_framework_as_dev_dependency(
    mut project_toml: Document,
    path_to_framework: &str,
    framework_version: &str,
    framework_name: &str,
    anchor_version: Option<String>,
) -> Document {
    if project_toml.get("dev-dependnecies").is_none() {
        project_toml["dev-dependencies"] = table();
    }
    project_toml["dev-dependencies"][framework_name]["version"] = value(framework_version);
    project_toml["dev-dependencies"][framework_name]["path"] = value(path_to_framework);
    if anchor_version.is_some() {
        project_toml["dev-dependencies"][framework_name]["features"] = table();
        let mut arr = Array::default();
        arr.push("anchor");
        project_toml["dev-dependencies"][framework_name]["features"] = value(arr);
    }
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
