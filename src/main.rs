//! SigmaOS-core executable entry point

use anyhow::Result;
use log::info;
use sigmaos_core::{Component, Config, CoreComponent};

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting SigmaOS-core daemon...");

    let config = Config::default();
    let mut component = CoreComponent::new(config);

    info!("Initializing component: {}", component.name());
    component.initialize().map_err(|e| anyhow::anyhow!(e))?;

    info!("Starting component: {} (v{})", component.name(), component.version());
    component.start().map_err(|e| anyhow::anyhow!(e))?;

    info!("Component running with status: {:?}", component.status());

    component.stop().map_err(|e| anyhow::anyhow!(e))?;
    info!("Component stopped cleanly.");

    Ok(())
}
