//! Configuration management for SigmaOS-core

use crate::error::{ComponentError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub enabled: bool,
    pub debug: bool,
    pub log_level: String,
    pub init_target: String,
    pub boot_timeout_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: true,
            debug: false,
            log_level: "info".to_string(),
            init_target: "multi-user.target".to_string(),
            boot_timeout_ms: 5000,
        }
    }
}

impl Config {
    pub fn new(log_level: &str, init_target: &str) -> Self {
        Self {
            enabled: true,
            debug: false,
            log_level: log_level.to_string(),
            init_target: init_target.to_string(),
            boot_timeout_ms: 5000,
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.boot_timeout_ms == 0 {
            return Err(ComponentError::ConfigError(
                "Boot timeout must be greater than 0".to_string(),
            ));
        }
        if self.init_target.is_empty() {
            return Err(ComponentError::ConfigError(
                "Init target cannot be empty".to_string(),
            ));
        }
        Ok(())
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| ComponentError::ConfigError(format!("Failed to read file: {}", e)))?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| ComponentError::ConfigError(format!("Failed to parse config: {}", e)))?;
        config.validate()?;
        Ok(config)
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.validate()?;
        let content = toml::to_string(self)
            .map_err(|e| ComponentError::ConfigError(format!("Failed to serialize config: {}", e)))?;
        fs::write(path, content)
            .map_err(|e| ComponentError::ConfigError(format!("Failed to write file: {}", e)))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default_aaa() {
        // Arrange
        let config = Config::default();

        // Act & Assert
        assert!(config.enabled);
        assert_eq!(config.log_level, "info");
        assert_eq!(config.init_target, "multi-user.target");
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_failure_aaa() {
        // Arrange
        let mut config = Config::default();
        config.init_target = String::new();

        // Act
        let result = config.validate();

        // Assert
        assert!(result.is_err());
    }
}
