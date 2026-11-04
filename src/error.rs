//! Error handling for SigmaOS-core

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ComponentError>;

#[derive(Error, Debug)]
pub enum ComponentError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Boot error: {0}")]
    BootError(String),

    #[error("ABI contract error: {0}")]
    AbiError(String),

    #[error("Initialization error: {0}")]
    InitError(String),

    #[error("Runtime error: {0}")]
    RuntimeError(String),

    #[error("Service already running")]
    AlreadyRunning,

    #[error("Service not running")]
    NotRunning,
}

impl From<anyhow::Error> for ComponentError {
    fn from(err: anyhow::Error) -> Self {
        ComponentError::RuntimeError(err.to_string())
    }
}
