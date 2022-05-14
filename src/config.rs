//! SolanaTestSetup Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.

use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

/// SolanaTestSetup Configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SolanaTestSetupConfig {
    pub init: InitSection,
}

impl Default for SolanaTestSetupConfig {
    fn default() -> Self {
        Self {
            init: InitSection::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InitSection {
    // Path to project
    pub path: PathBuf,
    // URL to POC Framework
    pub framework_repo_url: String,
    // Path to save generated tests boilerplate
    pub test_file_path: PathBuf,
    // Framework name
    pub framework_name: String,
    // Framework branch to be used
    pub framework_branch: String,
    // Should initialize with anchor
    pub is_anchor: Option<bool>,
}

impl Default for InitSection {
    fn default() -> Self {
        let current_dir = env::current_dir().expect("Cannot determine current dir");
        Self {
            path: current_dir.clone(),
            test_file_path: current_dir.clone().join("tests/genereted_test.rs"),
            framework_repo_url: String::from(
                "https://github.com/lowprivuser/solana-test-framework",
            ),
            framework_name: String::from("solana-test-framework"),
            framework_branch: String::from("main"),
            is_anchor: None,
        }
    }
}
