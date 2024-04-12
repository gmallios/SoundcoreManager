use nom::{bytes::complete::take, combinator::map, error::context, sequence::tuple};

use crate::models::{MonoEQ, StereoEQ};

use super::{ParseError, ParseResult};

pub fn parse_mono_eq<'a, E: ParseError<'a>>(
    bands: usize,
) -> impl Fn(&'a [u8]) -> ParseResult<MonoEQ, E> {
    move |bytes| {
        context(
            "parse_mono_eq",
            map(take(bands), |bytes: &[u8]| MonoEQ::from_vec(bytes.to_vec())),
        )(bytes)
    }
}

pub fn parse_stereo_eq<'a, E: ParseError<'a>>(
    bands: usize,
) -> impl Fn(&'a [u8]) -> ParseResult<StereoEQ, E> {
    move |bytes| {
        context(
            "parse_stereo_eq",
            map(
                tuple((parse_mono_eq(bands), parse_mono_eq(bands))),
                |(left, right)| StereoEQ { left, right },
            ),
        )(bytes)
    }
}

#[cfg(test)]
mod eq_parser {
    use super::*;

    #[test]
    fn parses_valid_mono_bytes() {
        let bytes = [0x00, 0x10, 0xF0, 0x10, 0x00, 0x10, 0xF0, 0x10];
        let expected = Ok((
            &b""[..],
            MonoEQ::from_signed_bytes(vec![-120, -104, 120, -104, -120, -104, 120, -104]),
        ));
        let output = parse_mono_eq::<nom::error::VerboseError<&[u8]>>(8)(&bytes);

        assert_eq!(expected, output);
    }

    #[test]
    fn parses_valid_stereo_bytes() {
        let bytes = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0,
            0xF0, 0xF0,
        ];
        let eq = MonoEQ::from_signed_bytes(vec![120, 120, 120, 120, 120, 120, 120, 120]);
        let expected = Ok((
            &b""[..],
            StereoEQ {
                left: MonoEQ::from_signed_bytes(vec![
                    -120, -120, -120, -120, -120, -120, -120, -120,
                ]),
                right: eq,
            },
        ));
        let output = parse_stereo_eq::<nom::error::VerboseError<&[u8]>>(8)(&bytes);

        assert_eq!(expected, output);
    }
}
