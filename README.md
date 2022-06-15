# SolanaTest
Cargo plugin to setup solana test environment.

By default it's using [solana-test-framework](https://github.com/lowprivuser/solana-test-framework)

## Installation

To install run: `cargo install --git https://github.com/f44z/cargo-solana-test`

## Usage
```
USAGE:
    cargo solana_test init [OPTIONS]

OPTIONS:
        --framework_url <FRAMEWORK_REPO_URL>    Url to framework repository.
    -h, --help                                  Print help information
        --is_anchor <IS_ANCHOR>                 Is anchor project.
        --path <PATH>                           Path to tested project.
        --test_file_path <TEST_FILE_PATH>       Path where to create test file.
```
