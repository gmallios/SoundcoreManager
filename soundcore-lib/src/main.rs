use env_logger::{Builder, Target};
use log::LevelFilter;

use soundcore_lib::ble::{BLEConnectionFactory, BLEConnectionManager};
use soundcore_lib::device_manager::create_device_manager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::new()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Trace)
        .init();
    let manager = create_device_manager().await;
    let scan_res = manager.ble_scan(None).await?;
    println!("{:?}", scan_res);
    let device = scan_res
        .iter()
        .find(|d| d.descriptor.name.contains("Q45"))
        .unwrap();
    let connection = manager.connect(device.clone()).await?;

    let mut state_channel = connection.state_channel().await;

    while let Ok(()) = state_channel.changed().await {
        println!("{:?}", state_channel.borrow_and_update());
    }
    // let registry = BtlePlugDeviceRegistry::new().await?;
    // let scan_res = registry.scan(None).await?;
    // println!("{:?}", scan_res);
    // let q45 = scan_res.iter().find(|d| d.name.contains("Q45")).unwrap();
    // let _connection = registry.connect(q45.clone(), None).await?;
    Ok(())
}
