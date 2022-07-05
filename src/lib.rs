#![warn(clippy::pedantic)]

pub type Error = Box<dyn std::error::Error + Sync + Send>;

pub type Result<T> = std::result::Result<T, Error>;
