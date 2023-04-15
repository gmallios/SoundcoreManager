use crate::{BluetoothAdrr, BthError, RFCOMMClient};
use async_trait::async_trait;
use windows::{
    core::{GUID, HSTRING},
    Devices::{Bluetooth::Rfcomm::RfcommServiceId, Enumeration::DeviceInformation},
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
        let svc_id = RfcommServiceId::FromUuid(GUID::from(uuid))?;
        let connected_devices = self.get_connected_devices().await?;
        let device = connected_devices
            .into_iter()
            .find(|device| BluetoothAdrr::from(device.BluetoothAddress().unwrap()) == bt_addr)
            .ok_or(BthError::DeviceNotFound)?;
        let svc = device
            .GetRfcommServicesForIdAsync(&svc_id)?
            .await?
            .Services()?
            .First()?
            .Current()?;
        self.sock
            .ConnectAsync(&svc.ConnectionHostName()?, &svc.ConnectionServiceName()?)?
            .await?;
        self.connected = true;
        self.dr = Some(DataReader::CreateDataReader(&self.sock.InputStream()?)?);
        self.dw = Some(DataWriter::CreateDataWriter(&self.sock.OutputStream()?)?);
        Ok(())
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
        self.sock.Close().expect("Failed to close socket");
    }
}

impl RFCOMM {
    async fn get_connected_devices(
        &self,
    ) -> Result<Vec<windows::Devices::Bluetooth::BluetoothDevice>, BthError> {
        let devices_inf = DeviceInformation::FindAllAsyncAqsFilter(&self._device_selector)?.await?;
        let connected_devices = devices_inf
            .into_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|d| {
                windows::Devices::Bluetooth::BluetoothDevice::FromIdAsync(&d.Id().unwrap())
                    .unwrap()
                    .get()
                    .unwrap()
            })
            .collect::<Vec<windows::Devices::Bluetooth::BluetoothDevice>>()
            .into_iter()
            .filter(|device| {
                device.ConnectionStatus().unwrap()
                    == windows::Devices::Bluetooth::BluetoothConnectionStatus::Connected
            })
            .collect::<Vec<windows::Devices::Bluetooth::BluetoothDevice>>();
        Ok(connected_devices)
    }
}