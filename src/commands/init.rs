//! `start` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::{prelude::*, project_toml, utility};

use crate::config::SolanaTestConfig;
use abscissa_core::{config, Command, FrameworkError, Runnable};
use clap::Parser;
use std::fs;
use std::{path::PathBuf, process::exit};

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
    #[clap(long = "path", help = "Path to tested project.")]
    path: Option<PathBuf>,

    /// Framework version
    #[clap(long = "framework_url", help = "Url to framework repository.")]
    framework_repo_url: Option<String>,

    /// Path to test file
    #[clap(long = "test_file_path", help = "Path where to create test file.")]
    test_file_path: Option<PathBuf>,

    #[clap(long = "is_anchor", help = "Is anchor project.")]
    is_anchor: Option<bool>,
}

impl Runnable for InitCmd {
    /// Start the application.
    fn run(&self) {
        let config = APP.config();
        let project_toml =
            project_toml::ProjectToml::new(config.init.path.clone(), &config.init.is_anchor)
                .expect("Cannot parse project toml");

        let modify_project_toml = &project_toml.clone().modify_project_toml(
            &config.init.framework_repo_url,
            &config.init.framework_branch,
            &config.init.framework_name,
        );

        match modify_project_toml {
            Ok(_) => {}
            Err(e) => {
                status_err!("{}", e);
                exit(2);
            }
        };

        // Create tests boilerplate
        if project_toml.get_is_anchor() {
            fs::write(
                config.init.test_file_path.to_str().unwrap(),
                utility::ANCHOR_TEMPLATE,
            )
            .expect("Could not write to file!");
        } else {
            fs::write(
                config.init.test_file_path.to_str().unwrap(),
                utility::SOLANA_TEMPLATE,
            )
            .expect("Could not write to file!");
        }
        status_ok!(
            "Completed",
            "Setup completed! You can run your tests using cargo test-bpf"
        );
    }
}

impl config::Override<SolanaTestConfig> for InitCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(
        &self,
        mut config: SolanaTestConfig,
    ) -> Result<SolanaTestConfig, FrameworkError> {
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
