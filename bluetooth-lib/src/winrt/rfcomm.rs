use crate::{BluetoothAdrr, BthError, RFCOMMClient};
use async_trait::async_trait;
use log::{debug, trace};
use windows::{
    core::{GUID, HSTRING},
    Devices::{
        Bluetooth::{
            BluetoothCacheMode,
            Rfcomm::{RfcommDeviceService, RfcommServiceId},
        },
    },
    Networking::Sockets::StreamSocket,
    Storage::Streams::{DataReader, DataWriter, InputStreamOptions},
};

pub struct RFCOMM {
    sock: StreamSocket,
    connected: bool,
    dr: Option<DataReader>,
    dw: Option<DataWriter>,
    _device_selector: HSTRING,
}

#[async_trait]
impl RFCOMMClient for RFCOMM {
    async fn new() -> Result<RFCOMM, BthError> {
        let sock = StreamSocket::new()?;
        Ok(RFCOMM {
            sock,
            connected: false,
            dr: None,
            dw: None,
            _device_selector: windows::Devices::Bluetooth::BluetoothDevice::GetDeviceSelector()?,
        })
    }

    async fn connect_uuid(&mut self, bt_addr: BluetoothAdrr, uuid: &str) -> Result<(), BthError> {
        trace!("Trying to connect to {} with uuid {}", bt_addr, uuid);
        let svc_id = RfcommServiceId::FromUuid(GUID::from(uuid))?;
        let device = windows::Devices::Bluetooth::BluetoothDevice::FromBluetoothAddressAsync(
            bt_addr.into(),
        )?
        .await?;

        debug!(
            "Found {} rfcomm services",
            device
                .GetRfcommServicesForIdWithCacheModeAsync(&svc_id, BluetoothCacheMode::Uncached)?
                .await?
                .Services()?
                .Size()?,
        );

        let mut service: Option<RfcommDeviceService> = None;
        let mut service_guids: Vec<GUID> = Vec::new();
        for s in device
            .GetRfcommServicesForIdWithCacheModeAsync(&svc_id, BluetoothCacheMode::Uncached)?
            .await?
            .Services()?
            .into_iter()
        {
            let guid = s.ServiceId()?.Uuid()?;
            service_guids.push(guid);
            if guid == GUID::from(uuid) {
                service = Some(s);
            }
        }

        debug!("Found rfcomm services with guids: {:?}", service_guids);

        if service.is_none() {
            return Err(BthError::RfcommServiceNotFound);
        } else {
            let svc = service.unwrap();
            trace!("Attempting to connect to service: {}", svc.ConnectionServiceName()?);
            self.sock
                .ConnectAsync(&svc.ConnectionHostName()?, &svc.ConnectionServiceName()?)?
                .await?;
            trace!("Successfully connected to service. Creating data reader and writer");
            self.connected = true;
            self.dr = Some(DataReader::CreateDataReader(&self.sock.InputStream()?)?);
            self.dw = Some(DataWriter::CreateDataWriter(&self.sock.OutputStream()?)?);
            trace!("Successfully created data reader and writer");
            Ok(())
        }
    }

    async fn connect_port(&mut self, _bt_addr: BluetoothAdrr, _port: u32) -> Result<(), BthError> {
        unimplemented!()
    }

    async fn send(&self, data: &[u8]) -> Result<(), BthError> {
        let dw = self.dw.clone().unwrap();
        dw.WriteBytes(data)?;
        dw.StoreAsync()?.await?;
        Ok(())
    }

    async fn recv(&self, _num_of_bytes: usize) -> Result<Vec<u8>, BthError> {
        let dr = self.dr.clone().unwrap();
        dr.SetInputStreamOptions(InputStreamOptions::Partial)?;
        let mut out_buf: Vec<u8> = Vec::new();
        dr.LoadAsync(1024)?.get()?;
        while dr.UnconsumedBufferLength()? > 0 {
            out_buf.push(dr.ReadByte()?);
        }
        Ok(out_buf)
    }

    async fn close(&self) {
        debug!("Closing winrt socket");
        self.sock.Close().expect("Failed to close socket");
    }
}
