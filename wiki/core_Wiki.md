# SigmaOS-core Component Wiki

## Overview
SigmaOS-core is the foundational SigmaOS component providing boot services, kernel boundary definitions, initialization contracts, and base system ABI agreements. It establishes the fundamental infrastructure upon which the entire SigmaOS operating system is built.

## Principles
- Single Responsibility
- Boot Foundation
- Kernel Boundary Abstraction
- Fail-Fast on ABI Violations
- Deterministic System Initialization
- Clear ABI Contracts
- Minimal Surface Area

## Architecture
SigmaOS-core follows a system foundation pattern:
- **Boot Sequence**: Defines the initial system startup sequence and early initialization
- **Kernel ABI**: Establishes the application-kernel interface contract
- **Init Contracts**: Provides the initialization interface for system daemons and services
- **Base System**: Core types and constants used throughout the SigmaOS ecosystem

## API
The component provides the following public APIs:
- `core::entry::bootstrap()` - System bootstrap and early initialization
- `core::abi::version()` - Get kernel ABI version
- `core::abi::call(number, args)` - Make kernel ABI call
- `core::init::register_daemon(handler)` - Register a system daemon
- Error types for ABI violations or initialization failures

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific system configuration
2. **Act**: Execute the code under test (bootstrap, ABI calls)
3. **Assert**: Verify the expected system initialization results

Example:
```rust
#[test]
fn test_core_bootstrap_aaa() {
    // Arrange
    let expected_state = /* initial system state */;
    
    // Act
    let state = core::entry::bootstrap();
    
    // Assert
    assert!(state.is_initialized());
    assert!(state.abi_version() >= 1);
}
```

## Development
- Follows Rust best practices with emphasis on safety and correctness
- System initialization is critical; fail-fast on any ABI contract violations
- Comprehensive error types for all system foundation failures
- Unit tests provided for bootstrap and ABI contract validation
- Integration tests for full system initialization

## TODO
- Document complete boot sequence with all initialization steps
- Expand ABI call catalog with all supported numbers
- Add comprehensive integration tests for system boot
- Establish ABI versioning and compatibility policies
- Create documentation for daemon registration and management