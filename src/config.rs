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
    pub poc_framework_repo_url: String,
    // Path to which POC framework should be saved
    pub poc_framework_output_path: Option<PathBuf>,
    // Path to save generated tests boilerplate
    pub test_file_path: PathBuf,
    // POC framework name
    pub framework_name: String,
    // Solana associated depencencies
    pub solana_dependencies: Vec<String>,
    // Anchor version
    pub anchor_version: Option<String>,
    // Anchor version
    pub solana_version: Option<String>,
}

impl Default for InitSection {
    fn default() -> Self {
        let current_dir = env::current_dir().expect("Cannot determine current dir");
        Self {
            path: current_dir.clone(),
            poc_framework_output_path: None,
            test_file_path: current_dir.clone().join("tests/genereted_test.rs"),
            poc_framework_repo_url: String::from("https://github.com/lowprivuser/solana-poc-async"),
            framework_name: String::from("solana-poc-async"),
            solana_dependencies: vec![
                "solana-banks-client".to_string(),
                "solana-banks-server".to_string(),
                "solana-bpf-loader-program".to_string(),
                "solana-logger".to_string(),
                "solana-program-runtime".to_string(),
                "solana-runtime".to_string(),
                "solana-sdk".to_string(),
                "solana-vote-program".to_string(),
                "solana-program".to_string(),
            ],
            anchor_version: None,
            solana_version: None,
        }
    }
}
