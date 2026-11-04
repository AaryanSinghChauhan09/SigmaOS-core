//! Boot sequence and bootloader integration for SigmaOS-core

use crate::error::{ComponentError, Result};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootState {
    Uninitialized,
    FirmwareHandoff,
    EarlyInit,
    KernelBoundaryReached,
    Ready,
}

#[derive(Debug, Clone)]
pub struct BootStage {
    pub name: String,
    pub priority: u32,
    pub completed: bool,
}

#[derive(Debug, Clone)]
pub struct BootManager {
    state: BootState,
    stages: VecDeque<BootStage>,
}

impl Default for BootManager {
    fn default() -> Self {
        Self::new()
    }
}

impl BootManager {
    pub fn new() -> Self {
        let mut stages = VecDeque::new();
        stages.push_back(BootStage {
            name: "early-firmware-handoff".to_string(),
            priority: 10,
            completed: false,
        });
        stages.push_back(BootStage {
            name: "memory-identity-mapping".to_string(),
            priority: 20,
            completed: false,
        });
        stages.push_back(BootStage {
            name: "kernel-boundary-contract".to_string(),
            priority: 30,
            completed: false,
        });

        Self {
            state: BootState::Uninitialized,
            stages,
        }
    }

    pub fn state(&self) -> &BootState {
        &self.state
    }

    pub fn execute_next_stage(&mut self) -> Result<Option<String>> {
        for stage in self.stages.iter_mut() {
            if !stage.completed {
                stage.completed = true;
                match stage.priority {
                    10 => self.state = BootState::FirmwareHandoff,
                    20 => self.state = BootState::EarlyInit,
                    30 => self.state = BootState::KernelBoundaryReached,
                    _ => self.state = BootState::Ready,
                }
                return Ok(Some(stage.name.clone()));
            }
        }
        self.state = BootState::Ready;
        Ok(None)
    }

    pub fn boot_all(&mut self) -> Result<()> {
        while let Some(_) = self.execute_next_stage()? {}
        if self.state != BootState::Ready {
            return Err(ComponentError::BootError(
                "Boot sequence did not complete successfully".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boot_sequence_lifecycle_aaa() {
        // Arrange
        let mut manager = BootManager::new();
        assert_eq!(*manager.state(), BootState::Uninitialized);

        // Act
        let stage1 = manager.execute_next_stage().unwrap();
        // Assert
        assert_eq!(stage1, Some("early-firmware-handoff".to_string()));
        assert_eq!(*manager.state(), BootState::FirmwareHandoff);

        // Complete remaining stages
        manager.boot_all().unwrap();
        assert_eq!(*manager.state(), BootState::Ready);
    }
}
