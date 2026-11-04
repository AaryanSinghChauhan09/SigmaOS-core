//! Base system init process and supervision contracts for SigmaOS-core

use crate::error::{ComponentError, Result};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnitStatus {
    Loaded,
    Active,
    Failed(String),
    Stopped,
}

#[derive(Debug, Clone)]
pub struct SystemUnit {
    pub name: String,
    pub description: String,
    pub status: UnitStatus,
}

#[derive(Debug, Clone)]
pub struct InitSupervisor {
    target: String,
    units: HashMap<String, SystemUnit>,
}

impl Default for InitSupervisor {
    fn default() -> Self {
        Self::new("multi-user.target")
    }
}

impl InitSupervisor {
    pub fn new(target: &str) -> Self {
        let mut supervisor = Self {
            target: target.to_string(),
            units: HashMap::new(),
        };
        supervisor.register_base_units();
        supervisor
    }

    fn register_base_units(&mut self) {
        self.units.insert(
            "devfs.service".to_string(),
            SystemUnit {
                name: "devfs.service".to_string(),
                description: "Device Filesystem Mounter".to_string(),
                status: UnitStatus::Loaded,
            },
        );
        self.units.insert(
            "syslog.service".to_string(),
            SystemUnit {
                name: "syslog.service".to_string(),
                description: "System Logging Service".to_string(),
                status: UnitStatus::Loaded,
            },
        );
        self.units.insert(
            "network.service".to_string(),
            SystemUnit {
                name: "network.service".to_string(),
                description: "Base Network Stack".to_string(),
                status: UnitStatus::Loaded,
            },
        );
    }

    pub fn start_unit(&mut self, name: &str) -> Result<()> {
        let unit = self
            .units
            .get_mut(name)
            .ok_or_else(|| ComponentError::InitError(format!("Unit '{}' not found", name)))?;
        unit.status = UnitStatus::Active;
        Ok(())
    }

    pub fn stop_unit(&mut self, name: &str) -> Result<()> {
        let unit = self
            .units
            .get_mut(name)
            .ok_or_else(|| ComponentError::InitError(format!("Unit '{}' not found", name)))?;
        unit.status = UnitStatus::Stopped;
        Ok(())
    }

    pub fn start_target(&mut self) -> Result<()> {
        let keys: Vec<String> = self.units.keys().cloned().collect();
        for key in keys {
            self.start_unit(&key)?;
        }
        Ok(())
    }

    pub fn all_active(&self) -> bool {
        self.units.values().all(|u| u.status == UnitStatus::Active)
    }

    pub fn target(&self) -> &str {
        &self.target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_supervisor_lifecycle_aaa() {
        // Arrange
        let mut supervisor = InitSupervisor::new("multi-user.target");
        assert!(!supervisor.all_active());

        // Act
        supervisor.start_target().unwrap();

        // Assert
        assert!(supervisor.all_active());
        supervisor.stop_unit("devfs.service").unwrap();
        assert!(!supervisor.all_active());
    }
}
