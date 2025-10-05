#![forbid(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
#![warn(missing_docs)]

//! TOTP and HOTP implementation

/// TOTP implementation
pub mod totp;

/// HOTP implementation
pub mod hotp;

pub use hotp::*;
pub use totp::*;
