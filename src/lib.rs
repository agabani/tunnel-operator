#![warn(clippy::pedantic)]

pub mod configuration;

pub mod server;

mod route;

pub mod shutdown;

/// Error returned by most functions.
///
/// For performance reasons, boxing is avoided in any hot path.
pub type Error = Box<dyn std::error::Error + Sync + Send>;

/// A specialized `Result` type for this crate's operations.
///
/// This is defined as a convenience to make error handling easier.
pub type Result<T> = std::result::Result<T, Error>;
