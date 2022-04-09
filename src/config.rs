//! SolanaTestInitializer Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.

use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

/// SolanaTestInitializer Configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SolanaTestInitializerConfig {
    /// An example configuration section
    pub init: InitSection,
}

/// Default configuration settings.
///
/// Note: if your needs are as simple as below, you can
/// use `#[derive(Default)]` on SolanaTestInitializerConfig instead.
impl Default for SolanaTestInitializerConfig {
    fn default() -> Self {
        Self {
            init: InitSection::default(),
        }
    }
}

/// Example configuration section.
///
/// Delete this and replace it with your actual configuration structs.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InitSection {
    /// Example configuration value
    pub path: PathBuf,
    pub poc_framework_repo_url: String,
    pub poc_framework_output_path: PathBuf,
    pub test_file_path: PathBuf,
    pub framework_name: String,
}

impl Default for InitSection {
    fn default() -> Self {
        let current_dir = env::current_dir().expect("Cannot determine current dir");
        Self {
            path: current_dir.clone(),
            poc_framework_output_path: current_dir.clone(),
            test_file_path: current_dir.clone().join("tests"),
            poc_framework_repo_url: String::from(
                "https://github.com/lowprivuser/solana-poc-async/archive/refs/tags/v0.1.0.zip",
            ),
            framework_name: String::from("solana-poc-async"),
        }
    }
}
