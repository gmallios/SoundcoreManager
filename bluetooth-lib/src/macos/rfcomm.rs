use async_trait::async_trait;
use crate::{BthError, RFCOMMClient};

use iobluetooth::{
    rfcomm::{self, rfcomm_client::RfcommClient},
    Request, TonicTransportChannel,
};

pub struct RFCOMM {
    client: RfcommClient<TonicTransportChannel>,
}

#[async_trait]
impl RFCOMMClient for RFCOMM {
    async fn new() -> Result<Self, BthError> {
        let client = RfcommClient::connect("http://[::1]:55777").await;
        match client {
            Ok(c) => Ok(RFCOMM { client: c }),
            Err(_e) => Err(BthError::FdInitError),
        }
    }

    async fn connect_uuid(
        &mut self,
        bt_addr: crate::BluetoothAdrr,
        uuid: &str,
    ) -> Result<(), crate::BthError> {
        let req = Request::new(rfcomm::OpenRfcommChannelRequest {
            addr: bt_addr.to_string(),
            uuid: uuid.to_string(),
        });
        match self.client.open_rfcomm_channel(req).await {
            Ok(_r) => Ok(()),
            Err(_e) => Err(BthError::TryConnectError),
        }
    }

    async fn connect_port(
        &mut self,
        address: crate::BluetoothAdrr,
        port: u32,
    ) -> Result<(), crate::BthError> {
        unimplemented!()
    }

    async fn send(&self, data: &[u8]) -> Result<(), crate::BthError> {
        let mut client = self.client.clone();
        let req = Request::new(rfcomm::SendRfcommDataRequest {
            data: data.to_vec(),
        });
        match client.send_rfcomm_data(req).await {
            Ok(_r) => Ok(()),
            Err(_e) => Err(BthError::SendError),
        }
    }

    async fn recv(&self, num_of_bytes: usize) -> Result<Vec<u8>, crate::BthError> {
        let mut res: Vec<u8> = Vec::with_capacity(num_of_bytes);
        let mut client = self.client.clone();
        let req = Request::new(rfcomm::RecvRfcommDataRequest {});
        match client.recv_rfcomm_data(req).await {
            Ok(r) => {
                let mut resp_data = r.into_inner().data;
                resp_data.resize(num_of_bytes, 0);
                Ok(resp_data)
            }
            Err(_e) => Err(BthError::RecvError),
        }
    }

    async fn close(&self) {
        let mut client = self.client.clone();
        let req = Request::new(rfcomm::CloseRfcommChannelRequest {});
        let _ = client.close_rfcomm_channel(req).await;
    }
}
