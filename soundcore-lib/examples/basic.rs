use bluetooth_lib::BluetoothAdrr;
use soundcore_lib::{base::SoundcoreDevice, devices::A3951};

const BT_ADDR: &str = "E8:EE:CC:56:C3:EA";

#[tokio::main]
async fn main() {
    let addr = BluetoothAdrr::from(BT_ADDR);
    let dev = A3951::default().init(addr).await.unwrap();
    let info = dev.get_info().await.unwrap();
    println!("Device info: {:?}", info);
    // println!("Device status: {:?}", info);
    // let charging = dev.get_battery_charging().await.unwrap();
    // println!("Charging: {:?}", charging);
}
