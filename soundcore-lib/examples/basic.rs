use std::error::Error;

use soundcore_lib::{
    api::{DeviceDescriptor, DeviceRegistry},
    device_registry::{create_soundcore_device_registry, SoundcoreDeviceRegistry},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let reg = create_soundcore_device_registry().await;
    let registry = SoundcoreDeviceRegistry::new(reg);
    let desc = registry.descriptors().await?;
    let desc = desc.get(0).unwrap();
    // registry.device(desc.model_id(), desc.mac_address()).await?;
    println!("{:?}", desc.name());
    let _dev = registry.device(desc.mac_address()).await?;
    Ok(())
}
