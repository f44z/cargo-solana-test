//! Main entry point for SolanaTestSetup

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use solana_test_initializer::application::APP;

/// Boot SolanaTestSetup
fn main() {
    abscissa_core::boot(&APP);
}
