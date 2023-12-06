use nom::{
    combinator::{map, map_opt},
    error::context,
    number::complete::le_u8,
    sequence::{pair, tuple},
};

use crate::models::{A3909ButtonModel, Action, ButtonSide, NonTwsButtonAction, TwsButtonAction};

use super::{base::parse_bool, ParseError, ParseResult};

pub fn parse_a3909_button_model<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> ParseResult<A3909ButtonModel, E> {
    context(
        "parse_a3909_button_model",
        map(
            tuple((
                parse_tws_button,
                parse_tws_button,
                parse_tws_button,
                parse_tws_button,
                parse_non_tws_button,
                parse_non_tws_button,
            )),
            |(
                l_double_press,
                l_long_press,
                r_double_press,
                r_long_press,
                l_single_press,
                r_single_press,
            )| {
                A3909ButtonModel {
                    left: ButtonSide {
                        double_press: l_double_press,
                        long_press: l_long_press,
                        single_press: l_single_press,
                    },
                    right: ButtonSide {
                        double_press: r_double_press,
                        long_press: r_long_press,
                        single_press: r_single_press,
                    },
                }
            },
        ),
    )(bytes)
}

fn parse_tws_button<'a, E: ParseError<'a>>(bytes: &'a [u8]) -> ParseResult<TwsButtonAction, E> {
    context(
        "parse_a3909_tws_button",
        map_opt(pair(parse_bool, le_u8), |(is_enabled, action)| {
            Some(TwsButtonAction {
                non_tws_action: Action::from_repr(action & 0x0F)?,
                tws_action: Action::from_repr(action >> 4)?,
                enabled: is_enabled,
            })
        }),
    )(bytes)
}

fn parse_non_tws_button<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> ParseResult<NonTwsButtonAction, E> {
    context(
        "parse_a3909_non_tws_button",
        map_opt(pair(parse_bool, le_u8), |(is_enabled, action)| {
            Some(NonTwsButtonAction {
                action: Action::from_repr(action)?,
                enabled: is_enabled,
            })
        }),
    )(bytes)
}

#[cfg(test)]
mod a3909_button_model {
    use super::*;

    const BYTES: [u8; 12] = [1, 99, 1, 84, 1, 102, 1, 84, 0, 1, 0, 0];

    #[test]
    fn parse_valid_bytes() {
        let model = parse_a3909_button_model::<nom::error::VerboseError<&[u8]>>(&BYTES);
        assert!(model.is_ok());

        let (remaining, model) = model.unwrap();
        assert!(remaining.is_empty());

        assert_eq!(model.left.double_press.non_tws_action, Action::NextSong);
        assert_eq!(model.left.double_press.tws_action, Action::PlayPause);
        assert!(model.left.double_press.enabled);

        assert_eq!(model.left.single_press.action, Action::VolumeDown);
        assert!(!model.left.single_press.enabled);

        assert_eq!(model.left.long_press.non_tws_action, Action::Trans);
        assert_eq!(model.left.long_press.tws_action, Action::VoiceAssistant);
        assert!(model.left.long_press.enabled);

        assert_eq!(model.right.double_press.non_tws_action, Action::PlayPause);
        assert_eq!(model.right.double_press.tws_action, Action::PlayPause);
        assert!(model.right.double_press.enabled);

        assert_eq!(model.right.single_press.action, Action::VolumeUp);
        assert!(!model.right.single_press.enabled);

        assert_eq!(model.right.long_press.non_tws_action, Action::Trans);
        assert_eq!(model.right.long_press.tws_action, Action::VoiceAssistant);
        assert!(model.right.long_press.enabled);
    }
}
