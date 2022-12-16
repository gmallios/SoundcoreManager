use bluetooth_lib::{platform::RFCOMM, RFCOMMClient};

#[tokio::main]
async fn main() {
    let mut rfcomm = RFCOMM::new().unwrap();
    rfcomm
        .connect_uuid(
            "AC:12:2F:6A:D2:07".into(),
            "00001101-0000-1000-8000-00805F9B34FB",
        )
        .await
        .unwrap();
}
