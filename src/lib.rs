//! SigmaOS-core - Core infrastructure for SigmaOS
//!
//! Provides boot coordination, kernel boundary contracts, ABI verification,
//! and base system init supervision.

pub mod abi;
pub mod boot;
pub mod config;
pub mod error;
pub mod init;
pub mod service;

pub use abi::{AbiRegistry, AbiVersion};
pub use boot::{BootManager, BootStage, BootState};
pub use config::Config;
pub use error::{ComponentError, Result};
pub use init::{InitSupervisor, SystemUnit, UnitStatus};
pub use service::Service;

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Component main trait adhering to SigmaOS architecture guidelines
pub trait Component {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self) -> Result<()>;
    fn start(&mut self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
    fn status(&self) -> ComponentStatus;
}

/// Component status representation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentStatus {
    Uninitialized,
    Initializing,
    Running,
    Stopped,
    Error(String),
}

/// Main Core component instance
pub struct CoreComponent {
    config: Config,
    service: Option<Service>,
    status: ComponentStatus,
}

impl CoreComponent {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            service: None,
            status: ComponentStatus::Uninitialized,
        }
    }

    pub fn from_config_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config = Config::from_file(path)?;
        Ok(Self::new(config))
    }
}

impl Component for CoreComponent {
    fn name(&self) -> &str {
        "SigmaOS-core"
    }

    fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }

    fn initialize(&mut self) -> Result<()> {
        self.status = ComponentStatus::Initializing;
        self.config.validate()?;
        self.service = Some(Service::new(self.config.clone()));
        self.status = ComponentStatus::Stopped;
        Ok(())
    }

    fn start(&mut self) -> Result<()> {
        self.with_service(|svc| {
            svc.start()?;
            self.status = ComponentStatus::Running;
            Ok(())
        })
    }

    fn stop(&mut self) -> Result<()> {
        self.with_service(|svc| {
            svc.stop()?;
            self.status = ComponentStatus::Stopped;
            Ok(())
        })
    }
}

impl CoreComponent {
    /// Helper method to execute operations on the service with error handling
    /// Follows DRY principle by avoiding duplicate error handling code
    fn with_service<F>(&mut self, operation: F) -> Result<()>
    where
        F: FnOnce(&mut Service) -> Result<()>,
    {
        match self.service.as_mut() {
            Some(svc) => operation(svc),
            None => {
                let err = "Service not initialized".to_string();
                self.status = ComponentStatus::Error(err.clone());
                Err(ComponentError::InitError(err))
            }
        }
    }

}

impl Component for CoreComponent {
    fn status(&self) -> ComponentStatus {
        self.status.clone()
    }
}
