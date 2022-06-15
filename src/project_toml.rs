use std::fs;
use std::path::PathBuf;
use toml_edit::{table, value, Array, Document};

use crate::error::{Error, ErrorKind};

#[derive(Debug)]
pub struct ProjectToml {
    pub document: Document,
    pub path: PathBuf,
    pub is_anchor: bool,
}
impl ProjectToml {
    pub fn new(path: PathBuf, is_anchor: &Option<bool>) -> Result<ProjectToml, ErrorKind> {
        let path_to_project_toml = PathBuf::new().join(path.clone()).join("Cargo.toml");

        if !path_to_project_toml.exists() {
            return Err(ErrorKind::MissingCargoFile);
        }

        let project_toml = fs::read_to_string(
            path_to_project_toml
                .to_str()
                .expect("Cannot convert path to str"),
        )
        .expect("Something went wrong reading the file");

        let project_toml_parsed = project_toml.parse::<Document>().unwrap();

        if !is_correct_cargo_toml(project_toml_parsed.clone()) {
            return Err(ErrorKind::IncorrectCargoFile);
        }

        let is_anchor: bool = if is_anchor.is_some() {
            is_anchor.unwrap()
        } else {
            check_if_is_anchor(&project_toml_parsed)
        };

        Ok(ProjectToml {
            document: project_toml_parsed,
            path: path_to_project_toml,
            is_anchor,
        })
    }
    pub fn add_test_bpf_feature(&mut self) -> Result<(), ErrorKind> {
        let is_features = self.document.get("features").is_some();

        if is_features && self.document["features"].get("test-bpf").is_some() {
            return Ok(());
        }

        if !is_features {
            self.document["features"] = table();
        }
        let empty_arr = Array::default();
        self.document["features"]["test-bpf"] = value(empty_arr);
        Ok(())
    }
    pub fn add_framework_as_dev_dependency(
        &mut self,
        framework_url: &str,
        framework_branch: &str,
        framework_name: &str,
    ) -> Result<(), ErrorKind> {
        if self.document.get("dev-dependnecies").is_none() {
            self.document["dev-dependencies"] = table();
        }
        self.document["dev-dependencies"][framework_name]["git"] = value(framework_url);
        self.document["dev-dependencies"][framework_name]["branch"] = value(framework_branch);

        if self.is_anchor {
            self.document["dev-dependencies"][framework_name]["features"] = table();
            let mut arr = Array::default();
            arr.push("anchor");
            self.document["dev-dependencies"][framework_name]["features"] = value(arr);
        }

        // Add solana test framework - it's crucial for framework to work
        // @todo when switch between solana version - this should be aligned
        self.document["dev-dependencies"]["solana-program-test"]["version"] = value("1.9");

        Ok(())
    }
    pub fn save_toml(self) {
        let contents = self.document.to_string();
        fs::write(self.path, contents).expect("Could not write to file!");
    }
}

pub fn is_correct_cargo_toml(project_toml: Document) -> bool {
    if project_toml.get("package").is_some() || project_toml.get("lib").is_some() {
        return true;
    }
    false
}

pub fn check_if_is_anchor(project_toml: &Document) -> bool {
    if project_toml.get("dependencies").is_some()
        && project_toml["dependencies"].get("anchor-lang").is_some()
    {
        return true;
    }
    false
}

pub fn modify_project_toml(
    path: PathBuf,
    is_anchor: &Option<bool>,
    framework_repo_url: &String,
    framework_branch: &String,
    framework_name: &String,
) -> Result<(), Error> {
    let mut project_toml = ProjectToml::new(path, is_anchor)?;
    project_toml.add_test_bpf_feature()?;

    project_toml.add_framework_as_dev_dependency(
        framework_repo_url,
        framework_branch,
        framework_name,
    )?;

    project_toml.save_toml();

    Ok(())
}
