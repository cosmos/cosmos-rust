//! Main entry point for Gaiauto
#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use gaiauto::application::APP;

/// Boot Gaiauto
fn main() {
    abscissa_core::boot(&APP);
}
