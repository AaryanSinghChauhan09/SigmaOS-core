//! Service lifecycle management for SigmaOS-core

use crate::boot::BootManager;
use crate::config::Config;
use crate::error::{ComponentError, Result};
use crate::init::InitSupervisor;
use log::info;

pub struct Service {
    config: Config,
    boot: BootManager,
    init: InitSupervisor,
    running: bool,
}

impl Service {
    pub fn new(config: Config) -> Self {
        let init = InitSupervisor::new(&config.init_target);
        Self {
            config,
            boot: BootManager::new(),
            init,
            running: false,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        if self.running {
            return Err(ComponentError::AlreadyRunning);
        }

        info!("Starting SigmaOS-core service...");
        self.config.validate()?;
        self.boot.boot_all()?;
        self.init.start_target()?;
        self.running = true;
        info!("SigmaOS-core service started successfully.");
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        if !self.running {
            return Err(ComponentError::NotRunning);
        }

        info!("Stopping SigmaOS-core service...");
        self.running = false;
        info!("SigmaOS-core service stopped successfully.");
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
