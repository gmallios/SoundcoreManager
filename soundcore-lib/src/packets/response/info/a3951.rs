use nom::{
    combinator::{all_consuming, map},
    error::context,
    sequence::pair,
};
use serde::{Deserialize, Serialize};

use crate::{
    models::{DeviceFirmware, SerialNumber},
    parsers::{parse_dual_fw, parse_serial_number, ParseError, ParseResult},
};

use super::DeviceInfoResponse;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Hash)]
pub struct A3951DeviceInfoResponse {
    pub sn: SerialNumber,
    pub fw: DeviceFirmware,
}

pub fn parse_a3951_device_info_packet<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> ParseResult<A3951DeviceInfoResponse, E> {
    context(
        "parse_a3951_device_info",
        all_consuming(map(pair(parse_dual_fw, parse_serial_number), |(fw, sn)| {
            A3951DeviceInfoResponse {
                fw: DeviceFirmware::new(fw.0, Some(fw.1)),
                sn,
            }
        })),
    )(bytes)
}

impl From<A3951DeviceInfoResponse> for DeviceInfoResponse {
    fn from(value: A3951DeviceInfoResponse) -> Self {
        DeviceInfoResponse {
            sn: Some(value.sn),
            fw: Some(value.fw),
        }
    }
}
