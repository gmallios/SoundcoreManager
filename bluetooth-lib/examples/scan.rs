use bluetooth_lib::{platform::BthScanner, Scanner};

#[tokio::main]
async fn main() {
    let mut scanner = BthScanner::new();
    let devices = scanner.scan().await;
    println!("{:?}", devices);
}
