use bluetooth_lib::{platform::BthScanner, BluetoothDevice, Scanner};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use soundcore_lib::BluetoothAdrr;

#[tokio::main]
async fn main() {
    let scan_res = BthScanner::new().scan().await;
    let res: Vec<BtDevice> = scan_res
        .into_iter()
        .filter(|device| device.connected)
        .map(BtDevice::from)
        .collect();

    let bt_device_selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select your device")
        .items(&res)
        .default(0)
        .interact()
        .unwrap();

    println!("{:?}", bt_device_selection);
}

struct BtDevice {
    name: String,
    address: BluetoothAdrr,
}

impl From<BluetoothDevice> for BtDevice {
    fn from(device: BluetoothDevice) -> Self {
        Self {
            name: device.name,
            address: device.address,
        }
    }
}

impl std::fmt::Display for BtDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.address)
    }
}
