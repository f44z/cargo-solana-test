//! Main entry point for SolanaTestInitializer

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use solana_test_initializer::application::APP;

/// Boot SolanaTestInitializer
fn main() {
    abscissa_core::boot(&APP);
}
