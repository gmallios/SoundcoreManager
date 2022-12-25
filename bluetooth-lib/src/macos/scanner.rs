use crate::types::{Scanner, BluetoothDevice, BluetoothAdrr};
use async_trait::async_trait;
use iobluetooth::{searcher::{SearchRequest, bt_searcher_client::BtSearcherClient}, scan};

pub struct BthScanner {}

#[async_trait]
impl Scanner for BthScanner {
    fn new() -> BthScanner {
        BthScanner {}
    }
    
    async fn scan(&mut self) -> Vec<BluetoothDevice> {
        let request = iobluetooth::Request::new(SearchRequest{
            time_to_scan: Some(5)
        });
        let mut client = BtSearcherClient::connect("http://[::1]:8080").await.unwrap();
        let resp = client.scan(request).await.unwrap();
        drop(client);
        let devices = resp.into_inner().result;
        let mut res = Vec::new();
        for dev in devices {
            res.push(BluetoothDevice {
                name: dev.name,
                address: BluetoothAdrr::from(dev.addr),
                connected: dev.is_connected,
                remembered: false,
            });
        }
        res
    }
}