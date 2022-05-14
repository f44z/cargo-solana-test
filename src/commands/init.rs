//! `start` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::{prelude::*, utility};

use crate::config::SolanaTestSetupConfig;
use abscissa_core::{config, Command, FrameworkError, Runnable};
use clap::Parser;
use std::fs;
use std::{path::PathBuf, process::exit};
use toml_edit::Document;

/// `start` subcommand
///
/// The `Parser` proc macro generates an option parser based on the struct
/// definition, and is defined in the `clap` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/clap/>
#[derive(Command, Debug, Parser)]
pub struct InitCmd {
    /// Path to tested project
    #[clap(long = "path", help = "Path to tested tested project.")]
    path: Option<PathBuf>,

    /// Framework version
    #[clap(
        long = "framework_url",
        help = "URL to poc-framework to download (must be in zip format)."
    )]
    framework_repo_url: Option<String>,

    /// Path to test file
    #[clap(long = "test_file_path", help = "Path to create test file.")]
    test_file_path: Option<PathBuf>,

    #[clap(long = "is_anchor", help = "Is anchor project.")]
    is_anchor: Option<bool>,
}

impl Runnable for InitCmd {
    /// Start the application.
    fn run(&self) {
        let config = APP.config();

        let path_to_project_toml = PathBuf::new()
            .join(config.init.path.clone())
            .join("Cargo.toml");

        if !path_to_project_toml.exists() {
            status_err!("Could not find project Cargo.toml");
            exit(1);
        }

        let project_toml = fs::read_to_string(
            path_to_project_toml
                .to_str()
                .expect("Cannot convert path to str"),
        )
        .expect("Something went wrong reading the file");

        let mut project_toml_parsed = project_toml.parse::<Document>().unwrap();

        if !utility::is_correct_cargo_toml(project_toml_parsed.clone()) {
            status_err!("Incorrect project Cargo.toml - make sure to select package Cargo.toml. Workspace toml is not allowed ");
            exit(1);
        }

        let is_anchor: bool = if config.init.is_anchor.is_some() {
            config.init.is_anchor.unwrap()
        } else {
            utility::is_anchor(&project_toml_parsed)
        };

        project_toml_parsed = utility::add_test_bpf_feature(project_toml_parsed.clone());
        project_toml_parsed = utility::add_framework_as_dev_dependency(
            project_toml_parsed.clone(),
            &config.init.framework_repo_url,
            &config.init.framework_branch,
            &config.init.framework_name,
            is_anchor.clone(),
        );

        utility::save_toml(
            project_toml_parsed,
            path_to_project_toml
                .to_str()
                .expect("Cannot convert path to str"),
        );

        // Create tests boilerplate
        fs::write(
            config.init.test_file_path.to_str().unwrap(),
            utility::TEST_TEMPLATE,
        )
        .expect("Could not write to file!");
        status_ok!(
            "Completed",
            "Setup completed! You can run your tests using cargo test-bpf"
        );
    }
}

impl config::Override<SolanaTestSetupConfig> for InitCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(
        &self,
        mut config: SolanaTestSetupConfig,
    ) -> Result<SolanaTestSetupConfig, FrameworkError> {
        if self.path.is_some() {
            if self.path.clone().unwrap().exists() {
                config.init.path = self.path.clone().unwrap();
            } else {
                status_err!(
                    "Incorrect path to tested project: {}",
                    self.path.clone().unwrap().to_str().unwrap()
                );
                exit(1);
            }
        }

        if self.framework_repo_url.is_some() {
            config.init.framework_repo_url = self.framework_repo_url.clone().unwrap();
        }

        if self.is_anchor.is_some() {
            config.init.is_anchor = self.is_anchor;
        }

        if self.test_file_path.is_some() {
            assert!(
                "rs" == self
                    .test_file_path
                    .clone()
                    .unwrap()
                    .extension()
                    .expect("Missing test file extension"),
                "Incorrect test file extension"
            );
            config.init.test_file_path = self.test_file_path.clone().unwrap();

            config.init.test_file_path = self.test_file_path.clone().unwrap();
        }

        if config.init.test_file_path.clone().parent().is_some() {
            if !config
                .init
                .test_file_path
                .clone()
                .parent()
                .clone()
                .unwrap()
                .exists()
            {
                fs::create_dir_all(config.init.test_file_path.clone().parent().unwrap()).unwrap();
            }
        }

        Ok(config)
    }
}
