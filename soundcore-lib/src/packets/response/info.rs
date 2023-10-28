mod a3951;

pub use a3951::*;
use nom::{combinator::map, error::context};
use serde::{Deserialize, Serialize};

use crate::{
    models::{DeviceFirmware, SerialNumber},
    parsers::{SoundcoreParseError, SoundcoreParseResult},
};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Hash)]
pub struct DeviceInfoResponse {
    pub sn: Option<SerialNumber>,
    pub fw: Option<DeviceFirmware>,
}

// TODO: Add more parsers
pub fn parse_device_info_packet<'a, E: SoundcoreParseError<'a>>(
    bytes: &'a [u8],
) -> SoundcoreParseResult<DeviceInfoResponse, E> {
    context("parse_device_info", |bytes| {
        map(
            parse_a3951_device_info_packet::<'a, E>,
            DeviceInfoResponse::from,
        )(bytes)
    })(bytes)
}
