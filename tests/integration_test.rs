use anyhow::Result;
use sigmaos_core::{
    AbiRegistry, AbiVersion, BootManager, BootState, Component, ComponentStatus, Config,
    CoreComponent, InitSupervisor, Service,
};

#[test]
fn test_core_component_lifecycle_aaa() -> Result<()> {
    // Arrange
    let config = Config::default();
    let mut component = CoreComponent::new(config);
    assert_eq!(component.status(), ComponentStatus::Uninitialized);

    // Act - Initialize
    component.initialize()?;
    assert_eq!(component.status(), ComponentStatus::Stopped);

    // Act - Start
    component.start()?;
    // Assert
    assert_eq!(component.status(), ComponentStatus::Running);

    // Act - Stop
    component.stop()?;
    // Assert
    assert_eq!(component.status(), ComponentStatus::Stopped);

    Ok(())
}

#[test]
fn test_service_start_stop_aaa() -> Result<()> {
    // Arrange
    let config = Config::default();
    let mut service = Service::new(config);
    assert!(!service.is_running());

    // Act
    service.start()?;
    // Assert
    assert!(service.is_running());
    assert!(service.start().is_err()); // Double start fails fast

    // Act
    service.stop()?;
    // Assert
    assert!(!service.is_running());
    assert!(service.stop().is_err()); // Double stop fails fast

    Ok(())
}

#[test]
fn test_boot_sequence_and_abi_integration_aaa() -> Result<()> {
    // Arrange
    let mut boot = BootManager::new();
    let abi = AbiRegistry::new();

    // Act
    boot.boot_all()?;

    // Assert
    assert_eq!(*boot.state(), BootState::Ready);
    assert!(abi.contract_count() >= 3);
    assert!(abi
        .verify_contract("sigmaos.kernel.syscall.v1", &AbiVersion::CURRENT)
        .is_ok());

    Ok(())
}

#[test]
fn test_init_supervisor_target_execution_aaa() -> Result<()> {
    // Arrange
    let mut supervisor = InitSupervisor::new("multi-user.target");
    assert_eq!(supervisor.target(), "multi-user.target");
    assert!(!supervisor.all_active());

    // Act
    supervisor.start_target()?;

    // Assert
    assert!(supervisor.all_active());

    Ok(())
}
