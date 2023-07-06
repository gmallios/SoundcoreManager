use crate::api::{BatteryLevel, ChargingStatus, EQValues, SoundMode};
use crate::{
    api::SoundcoreDeviceState,
    bt::ble::BLEConnection,
    devices::{a3951::device::A3951, SupportedModelIDs},
    error::SoundcoreResult,
};
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc::Receiver};

#[enum_dispatch]
#[async_trait]
pub trait SoundcoreDevice<ConnectionType>: Send + Sync
where
    ConnectionType: BLEConnection + Send + Sync,
{
    async fn new(connection: Arc<ConnectionType>) -> SoundcoreResult<Self>
    where
        Self: Sized;
    async fn get_initial_state(
        connection: &Arc<ConnectionType>,
        receiver: &mut Receiver<Vec<u8>>,
    ) -> SoundcoreResult<SoundcoreDeviceState>
    where
        Self: Sized;

    fn subscribe_state(&self) -> broadcast::Receiver<SoundcoreDeviceState>;
    async fn refresh_state(&self) -> SoundcoreResult<()>;
    async fn set_sound_mode(&self, sound_mode: SoundMode) -> SoundcoreResult<()>;
    async fn set_eq(&self, eq: EQValues) -> SoundcoreResult<()>;
    async fn name(&self) -> String;
    fn model_id(&self) -> SupportedModelIDs;
    async fn eq(&self) -> EQValues;
    async fn sound_mode(&self) -> SoundMode;
    async fn battery_level(&self) -> BatteryLevel;
    async fn charging_status(&self) -> ChargingStatus;
}

#[enum_dispatch(SoundcoreDevice)]
pub enum SoundcoreDevices<ConnectionType>
where
    ConnectionType: BLEConnection + Send + Sync,
{
    A3951(A3951<ConnectionType>),
}

impl<ConnectionType> SoundcoreDevices<ConnectionType>
where
    ConnectionType: BLEConnection + Send + Sync,
{
    pub fn to_device(&self) -> Option<&dyn SoundcoreDevice<ConnectionType>> {
        match self {
            SoundcoreDevices::A3951(device) => Some(device),
            _ => None,
        }
    }
}
