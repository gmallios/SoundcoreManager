use nom::{combinator::map, error::context, number::complete::le_u8, sequence::tuple};

use crate::models::{DualBattery, SingleBattery};

use super::{base::parse_bool, SoundcoreParseError, SoundcoreParseResult};

pub fn parse_dual_battery<'a, E: SoundcoreParseError<'a>>(
    bytes: &'a [u8],
) -> SoundcoreParseResult<DualBattery, E> {
    context(
        "parse_dual_batt",
        map(
            tuple((le_u8, le_u8, parse_bool, parse_bool)),
            |(left_level, right_level, left_charging, right_charging)| DualBattery {
                left: SingleBattery {
                    charging: left_charging,
                    level: left_level,
                },
                right: {
                    SingleBattery {
                        charging: right_charging,
                        level: right_level,
                    }
                },
            },
        ),
    )(bytes)
}

pub fn parse_single_battery<'a, E: SoundcoreParseError<'a>>(
    bytes: &'a [u8],
) -> SoundcoreParseResult<SingleBattery, E> {
    context(
        "parse_single_batt",
        map(tuple((le_u8, parse_bool)), |(level, charging)| {
            SingleBattery { charging, level }
        }),
    )(bytes)
}

#[cfg(test)]
mod battery_parser {
    use super::*;

    #[test]
    fn parses_dual_battery() {
        let bytes = [0x05, 0x01, 0x00, 0x01];
        let result = parse_dual_battery::<nom::error::VerboseError<&[u8]>>(&bytes);
        assert!(result.is_ok());
        let (remaining, battery) = result.unwrap();
        assert_eq!(remaining, &b""[..]);
        assert_eq!(
            battery,
            DualBattery {
                left: SingleBattery {
                    charging: false,
                    level: 5
                },
                right: SingleBattery {
                    charging: true,
                    level: 1
                }
            }
        );
    }

    #[test]
    fn parses_single_battery() {
        let bytes = [0x05, 0x01];
        let result = parse_single_battery::<nom::error::VerboseError<&[u8]>>(&bytes);
        assert!(result.is_ok());
        let (remaining, battery) = result.unwrap();
        assert_eq!(remaining, &b""[..]);
        assert_eq!(
            battery,
            SingleBattery {
                charging: true,
                level: 5
            }
        );
    }
}
