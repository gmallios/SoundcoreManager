

use nom::{
    combinator::{map_opt},
    error::context,
};

use super::{take_bytes_from_end, SoundcoreParseError, SoundcoreParseResult};

pub fn parse_and_check_checksum<'a, E: SoundcoreParseError<'a>>(
    bytes: &'a [u8],
) -> SoundcoreParseResult<(), E> {
    context(
        "parse_and_check_checksum",
        map_opt(take_bytes_from_end(2usize), |last_bytes| {
            let checksum = generate_checksum(&bytes[..bytes.len() - 1]);
            match last_bytes == [0x00, checksum] {
                // TODO: Padding bit??
                true => Some(()),
                false => None,
            }
        }),
    )(bytes)
}

fn generate_checksum<'a, I>(bytes: I) -> u8
where
    I: IntoIterator<Item = &'a u8>,
{
    bytes.into_iter().fold(0, |acc, x| acc.wrapping_add(*x))
}

#[cfg(test)]
mod checksum_parser {
    use super::*;

    #[test]
    fn parses_and_checks_checksum() {
        let input = [0x01, 0x02, 0x00, 0x00, 0x00, 0x03];
        let expected: (&[u8], ()) = (&[0x01, 0x02, 0x00, 0x00], ());
        let actual = parse_and_check_checksum::<nom::error::VerboseError<&[u8]>>(&input);
        assert!(actual.is_ok());
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn fails_to_parse_and_check_checksum() {
        let input = [0x01, 0x02, 0x00, 0x00, 0x00, 0x05];
        let res = parse_and_check_checksum::<nom::error::VerboseError<&[u8]>>(&input);
        assert!(res.is_err());
    }
}
