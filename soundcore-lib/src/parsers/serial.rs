use nom::{combinator::map, error::context};

use crate::models::SerialNumber;

use super::{base::parse_str, ParseError, ParseResult};

pub fn parse_serial_number<'a, E: ParseError<'a>>(bytes: &'a [u8]) -> ParseResult<SerialNumber, E> {
    context("parse_serial", map(parse_str(16usize), SerialNumber::from))(bytes)
}

#[cfg(test)]
mod serial_parser {

    #[test]
    fn parses_valid_serial() {
        let bytes = b"AC12F3B4D5E6A7B8";
        let expected = Ok((&b""[..], "AC12F3B4D5E6A7B8".into()));
        let actual = super::parse_serial_number::<nom::error::VerboseError<&[u8]>>(bytes);
        assert_eq!(expected, actual);
    }
}
