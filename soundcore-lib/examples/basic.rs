use bluetooth_lib::BluetoothAdrr;
use soundcore_lib::{devices::{A3951}, base::SoundcoreDevice};

const BT_ADDR: &str = "AC:12:2F:6A:D2:07";


#[tokio::main]
async fn main() {
    let addr = BluetoothAdrr::from(BT_ADDR);
    let dev =  A3951::default().init(addr).await.unwrap();
    let info = dev.get_status().await.unwrap();
    println!("Device info: {:?}", info);
    // let charging = dev.get_battery_charging().await.unwrap();
    // println!("Charging: {:?}", charging);
}
