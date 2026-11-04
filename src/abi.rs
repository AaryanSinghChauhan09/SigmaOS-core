//! ABI contract specification and kernel boundary verification for SigmaOS-core

use crate::error::{ComponentError, Result};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbiVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl AbiVersion {
    pub const CURRENT: Self = Self {
        major: 1,
        minor: 0,
        patch: 0,
    };

    pub fn is_compatible_with(&self, target: &AbiVersion) -> bool {
        self.major == target.major && self.minor >= target.minor
    }
}

#[derive(Debug, Clone)]
pub struct AbiRegistry {
    contracts: HashMap<String, AbiVersion>,
}

impl Default for AbiRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl AbiRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            contracts: HashMap::new(),
        };
        registry.register_core_contracts();
        registry
    }

    fn register_core_contracts(&mut self) {
        self.contracts.insert(
            "sigmaos.kernel.syscall.v1".to_string(),
            AbiVersion::CURRENT,
        );
        self.contracts.insert(
            "sigmaos.kernel.ipc.v1".to_string(),
            AbiVersion::CURRENT,
        );
        self.contracts.insert(
            "sigmaos.base.init.v1".to_string(),
            AbiVersion::CURRENT,
        );
    }

    pub fn verify_contract(&self, name: &str, required: &AbiVersion) -> Result<()> {
        match self.contracts.get(name) {
            Some(v) if v.is_compatible_with(required) => Ok(()),
            Some(v) => Err(ComponentError::AbiError(format!(
                "Incompatible ABI contract '{}': available {}.{}.{}, required {}.{}.{}",
                name, v.major, v.minor, v.patch, required.major, required.minor, required.patch
            ))),
            None => Err(ComponentError::AbiError(format!(
                "ABI contract '{}' not registered",
                name
            ))),
        }
    }

    pub fn contract_count(&self) -> usize {
        self.contracts.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abi_compatibility_aaa() {
        // Arrange
        let registry = AbiRegistry::new();
        let target = AbiVersion {
            major: 1,
            minor: 0,
            patch: 0,
        };

        // Act & Assert
        assert!(registry.verify_contract("sigmaos.kernel.syscall.v1", &target).is_ok());
        assert!(registry.verify_contract("sigmaos.unknown.contract", &target).is_err());
    }
}
