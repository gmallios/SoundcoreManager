use nom::{combinator::map, error::context, sequence::pair};

use crate::models::FirmwareVer;

use super::{parse_str, SoundcoreParseError, SoundcoreParseResult};

pub fn parse_fw<'a, E: SoundcoreParseError<'a>>(
    bytes: &'a [u8],
) -> SoundcoreParseResult<FirmwareVer, E> {
    context(
        "parse_fw",
        map(parse_str(5usize), |ver| {
            let (major, minor) = match ver.split_once('.') {
                Some((major_str, minor_str)) => (
                    major_str.parse().unwrap_or_default(),
                    minor_str.parse().unwrap_or_default(),
                ),
                None => (0, 0),
            };
            FirmwareVer::new(major, minor)
        }),
    )(bytes)
}

pub fn parse_dual_fw<'a, E: SoundcoreParseError<'a>>(
    bytes: &'a [u8],
) -> SoundcoreParseResult<(FirmwareVer, FirmwareVer), E> {
    context("map_dual_fw", pair(parse_fw, parse_fw))(bytes)
}

#[cfg(test)]
mod fw_parser {
    // TODO: Add tests
}
