use crate::models::EQProfile;
use crate::parsers::{ParseError, ParseResult};
use log::warn;
use nom::combinator::map;
use nom::error::context;
use nom::number::complete::le_u8;
use nom::sequence::tuple;
use strum::EnumCount;

pub fn parse_a3040_eq_info_update<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> ParseResult<EQProfile, E> {
    context(
        "parse_a3040_eq_info_update",
        map(tuple((le_u8, le_u8)), |(b1, b2)| {
            let eq_idx = (b1 as u16) | ((b2 as u16) << 8).clamp(0, EQProfile::COUNT as u16);
            match EQProfile::from_id_le(eq_idx) {
                Some(eq) => eq,
                None => {
                    warn!("Unknown EQ profile index from eq info update: {}", eq_idx);
                    EQProfile::SoundcoreSignature
                }
            }
        }),
    )(bytes)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn should_parse_eq_info_update() {
        let bytes = &[0x01, 0x00];
        let result = parse_a3040_eq_info_update::<nom::error::VerboseError<&[u8]>>(bytes);
        assert_eq!(result, Ok((&[][..], EQProfile::Acoustic)));
    }

    #[test]
    pub fn should_default_if_index_is_out_of_bounds() {
        let bytes = &[0xFF, 0x00];
        let result = parse_a3040_eq_info_update::<nom::error::VerboseError<&[u8]>>(bytes);
        assert_eq!(result, Ok((&[][..], EQProfile::SoundcoreSignature)));
    }
}
