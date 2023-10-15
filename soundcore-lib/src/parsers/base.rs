use nom::{
    bytes::complete::take,
    combinator::{map, map_opt},
    error::{ParseError},
    multi::count,
    number::complete::le_u8,
};

use super::{SoundcoreParseError, SoundcoreParseResult};

pub fn parse_bool<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> SoundcoreParseResult<bool, E> {
    map(le_u8, |value| value == 1)(input)
}

pub fn parse_str<'a, E>(len: usize) -> impl Fn(&'a [u8]) -> SoundcoreParseResult<&'a str, E>
where
    E: SoundcoreParseError<'a>,
{
    move |input| map_opt(take(len), |bytes| std::str::from_utf8(bytes).ok())(input)
}

pub fn bool_parser<'a, T, E>(bytes: &'a [u8]) -> SoundcoreParseResult<T, E>
where
    T: From<bool>,
    E: SoundcoreParseError<'a>,
{
    map(parse_bool, T::from)(bytes)
}

pub fn u8_parser<'a, T, E>(bytes: &'a [u8]) -> SoundcoreParseResult<T, E>
where
    T: From<u8>,
    E: SoundcoreParseError<'a>,
{
    map(le_u8, T::from)(bytes)
}

pub fn take_last_byte<'a, E: SoundcoreParseError<'a>>(
    bytes: &'a [u8],
) -> SoundcoreParseResult<u8, E> {
    let (_arr, last_byte) = le_u8(&bytes[bytes.len() - 1..])?;
    Ok((&bytes[..bytes.len() - 1], last_byte))
}

pub fn take_bytes_from_end<'a, E: SoundcoreParseError<'a>>(
    size: usize,
) -> impl Fn(&'a [u8]) -> SoundcoreParseResult<Vec<u8>, E> {
    move |bytes| {
        let (_, mut end_bytes) = count(take_last_byte, size)(bytes)?;
        end_bytes.reverse();
        Ok((&bytes[..bytes.len() - size], end_bytes))
    }
}

#[cfg(test)]
mod base_parsers {
    use super::*;

    #[test]
    fn parses_str() {
        let input = b"ExampleString";
        let expected = Ok((&b""[..], "ExampleString"));
        let actual = parse_str::<nom::error::VerboseError<&[u8]>>(input.len())(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_true_bool() {
        let input = b"\x01";
        let expected = Ok((&b""[..], true));
        let actual = parse_bool::<nom::error::VerboseError<&[u8]>>(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_false_bool() {
        let input = b"\x00";
        let expected = Ok((&b""[..], false));
        let actual = parse_bool::<nom::error::VerboseError<&[u8]>>(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_last_byte() {
        let input = [0x00, 0x00, 0xFF, 0xEE];
        let expected: Result<(&[u8], u8), _> = Ok((&[0x00, 0x00, 0xFF], 0xEE));
        let actual = take_last_byte::<nom::error::VerboseError<&[u8]>>(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn takes_bytes_from_end() {
        let input = [0x00, 0x00, 0x10, 0x20, 0x30, 0xAA, 0xFF];
        let expected: Result<(&[u8], Vec<u8>), _> =
            Ok((&[0x00, 0x00, 0x10, 0x20, 0x30], vec![0xAA, 0xFF]));
        let actual = take_bytes_from_end::<nom::error::VerboseError<&[u8]>>(2)(&input);

        assert_eq!(expected, actual);
    }
}
