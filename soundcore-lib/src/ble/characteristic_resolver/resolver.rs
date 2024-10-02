#![allow(unused)]
use std::sync::Arc;

use uuid::{uuid, Uuid};

use crate::error::SoundcoreLibResult;

static EXCLUDED_SERVICE_UUIDS: [Uuid; 4] = [
    uuid!("00001800-0000-1000-8000-00805f9b34fb"),
    uuid!("00001801-0000-1000-8000-00805f9b34fb"),
    uuid!("86868686-8686-8686-8686-868686868686"),
    uuid!("66666666-6666-6666-6666-666666666666"),
];

pub struct Service {
    pub uuid: Uuid,
    pub characteristics: Arc<[Characteristic]>,
    pub should_exclude: bool,
}

pub struct Characteristic {
    pub uuid: Uuid,
    pub properties: Arc<[CharacteristicProperty]>,
}

pub enum CharacteristicProperty {
    READ,
    WRITE,
    #[allow(non_camel_case_types)]
    WRITE_WITHOUT_RESPONSE,
}

impl Service {
    #[cfg(feature = "btleplug-backend")]
    async fn from_btleplug(_service: btleplug::api::Service) -> SoundcoreLibResult<Self> {
        todo!()
    }
}
