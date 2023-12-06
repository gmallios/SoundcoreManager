use nom::{bytes::complete::take, combinator::map, error::context, sequence::tuple};

use crate::models::{MonoEQ, StereoEQ};

use super::{ParseError, ParseResult};

// The official app supports up to 10-Band EQs,
// since we currently support up to 8,
// we should use all consuming
pub fn parse_mono_eq<'a, E: ParseError<'a>>(bytes: &'a [u8]) -> ParseResult<MonoEQ, E> {
    context(
        "parse_mono_eq",
        map(take(8usize), |bytes: &[u8]| {
            MonoEQ::from_bytes(bytes.try_into().unwrap())
        }),
    )(bytes)
}

pub fn parse_stereo_eq<'a, E: ParseError<'a>>(bytes: &'a [u8]) -> ParseResult<StereoEQ, E> {
    context(
        "parse_stereo_eq",
        map(tuple((parse_mono_eq, parse_mono_eq)), |(left, right)| {
            StereoEQ { left, right }
        }),
    )(bytes)
}

#[cfg(test)]
mod eq_parser {
    use super::*;

    #[test]
    fn parses_valid_mono_bytes() {
        let bytes = [0x00, 0x10, 0xF0, 0x10, 0x00, 0x10, 0xF0, 0x10];
        let expected = Ok((
            &b""[..],
            MonoEQ {
                values: [-120, -104, 120, -104, -120, -104, 120, -104],
            },
        ));
        let output = parse_mono_eq::<nom::error::VerboseError<&[u8]>>(&bytes);

        assert_eq!(expected, output);
    }

    #[test]
    fn parses_valid_stereo_bytes() {
        let bytes = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0,
            0xF0, 0xF0,
        ];
        let eq = MonoEQ::new([120, 120, 120, 120, 120, 120, 120, 120]);
        let expected = Ok((
            &b""[..],
            StereoEQ {
                left: MonoEQ {
                    values: [-120, -120, -120, -120, -120, -120, -120, -120],
                },
                right: eq,
            },
        ));
        let output = parse_stereo_eq::<nom::error::VerboseError<&[u8]>>(&bytes);

        assert_eq!(expected, output);
    }
}
