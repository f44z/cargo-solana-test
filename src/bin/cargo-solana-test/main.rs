//! Main entry point for CargoSolanaTest

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use cargo_solana_test::application::APP;

/// Boot CargoSolanaTest
fn main() {
    abscissa_core::boot(&APP);
}
