//! Main entry point for SolanaTest

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use solana_test::application::APP;

/// Boot SolanaTest
fn main() {
    abscissa_core::boot(&APP);
}
