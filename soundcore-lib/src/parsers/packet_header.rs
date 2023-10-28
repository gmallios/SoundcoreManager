use nom::{
    bytes::complete::take,
    combinator::{map, map_opt},
    error::context,
    number::complete::le_u16,
    sequence::tuple,
};

use crate::models::{ResponsePacketHeader, ResponsePacketKind, PACKET_KIND_MAP};

use super::{SoundcoreParseError, SoundcoreParseResult};

pub fn parse_packet_header<'a, E: SoundcoreParseError<'a>>(
    bytes: &'a [u8],
) -> SoundcoreParseResult<ResponsePacketHeader, E> {
    context(
        "parse_packet_header",
        map(
            tuple((
                parse_packet_prefix,
                parse_packet_kind,
                context("header_length", le_u16), // TODO: extract this to PacketHeaderSuffix?
            )),
            |(_, packet_kind, size)| ResponsePacketHeader {
                kind: packet_kind,
                length: size,
            },
        ),
    )(bytes)
}

fn parse_packet_kind<'a, E: SoundcoreParseError<'a>>(
    bytes: &'a [u8],
) -> SoundcoreParseResult<ResponsePacketKind, E> {
    context(
        "parse_packet_header",
        map_opt(take(2usize), |bytes: &[u8]| {
            PACKET_KIND_MAP
                .iter()
                .find(|(map_bytes, _)| map_bytes == &bytes)
                .map(|(_, packet_kind)| *packet_kind)
        }),
    )(bytes)
}

fn parse_packet_prefix<'a, E: SoundcoreParseError<'a>>(
    bytes: &'a [u8],
) -> SoundcoreParseResult<Result<(), ()>, E> {
    /* If any other prefixes are found, they can be added here */
    context(
        "parse_packet_header",
        map(take(5usize), |prefix: &[u8]| match prefix {
            [0x09, 0xFF, 0x00, 0x00, 0x01] => Ok(()),
            _ => Err(()),
        }),
    )(bytes)
}

#[cfg(test)]
mod packet_header_parser {
    use crate::models::{ResponsePacketHeader, ResponsePacketKind};

    #[test]
    fn test_parse_complete_packet_header() {
        let bytes = [0x09, 0xFF, 0x00, 0x00, 0x01, 0xFF, 0xFF, 0x05, 0x00];
        let result = super::parse_packet_header::<nom::error::VerboseError<&[u8]>>(&bytes);
        assert!(result.is_ok());
        let (remaining, packet_kind) = result.unwrap();
        assert_eq!(remaining, &[]);
        assert_eq!(
            packet_kind,
            ResponsePacketHeader {
                kind: ResponsePacketKind::Unknown,
                length: 0x05
            }
        );
    }

    #[test]
    fn test_parse_packet_kind() {
        let bytes = [0xFF, 0xFF];
        let result = super::parse_packet_kind::<nom::error::VerboseError<&[u8]>>(&bytes);
        assert!(result.is_ok());
        let (remaining, packet_kind) = result.unwrap();
        assert_eq!(remaining, &[]);
        assert_eq!(packet_kind, ResponsePacketKind::Unknown);
    }

    #[test]
    fn test_parse_packet_prefix() {
        let bytes = [0x09, 0xFF, 0x00, 0x00, 0x01];
        let result = super::parse_packet_prefix::<nom::error::VerboseError<&[u8]>>(&bytes);
        assert!(result.is_ok());
        let (remaining, _) = result.unwrap();
        assert_eq!(remaining, &[]);
    }
}
