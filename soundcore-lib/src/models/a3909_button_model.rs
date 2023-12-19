use crate::models::Action;
use serde::{Deserialize, Serialize};


/// This is the A3909 variant of the CustomBtnModel
/// TODO: Check if there are common models to other button models, if so move them to a common file
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct A3909ButtonModel {
    pub left: ButtonSide,
    pub right: ButtonSide,
}
impl A3909ButtonModel {
    fn bytes(&self) -> [u8; 12] {
        let mut bytes = [0u8; 12];
        bytes[0..2].copy_from_slice(&self.left.double_press.bytes());
        bytes[2..4].copy_from_slice(&self.left.long_press.bytes());
        bytes[4..6].copy_from_slice(&self.right.double_press.bytes());
        bytes[6..8].copy_from_slice(&self.right.long_press.bytes());
        bytes[8..10].copy_from_slice(&self.left.single_press.bytes());
        bytes[10..12].copy_from_slice(&self.right.single_press.bytes());
        bytes
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct ButtonSide {
    pub double_press: TwsButtonAction,
    pub single_press: NonTwsButtonAction,
    pub long_press: TwsButtonAction,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct TwsButtonAction {
    pub non_tws_action: Action, /* Disconnected Action */
    pub tws_action: Action,     /* Connected Action */
    pub enabled: bool,
}

impl TwsButtonAction {
    pub fn bytes(&self) -> [u8; 2] {
        [
            self.enabled as u8,
            (self.non_tws_action.as_u8() << 4) | (self.tws_action.as_u8() & 0x0F),
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct NonTwsButtonAction {
    pub action: Action,
    pub enabled: bool,
}

impl NonTwsButtonAction {
    pub fn bytes(&self) -> [u8; 2] {
        [self.enabled as u8, self.action.as_u8() & 0x0F]
    }
}

#[cfg(test)]
mod a3909_button_model {
    use super::*;

    #[test]
    fn to_bytes() {
        let model = A3909ButtonModel {
            left: ButtonSide {
                double_press: TwsButtonAction {
                    non_tws_action: Action::NextSong,
                    tws_action: Action::PreviousSong,
                    enabled: true,
                },
                single_press: NonTwsButtonAction {
                    action: Action::VoiceAssistant,
                    enabled: false,
                },
                long_press: TwsButtonAction {
                    non_tws_action: Action::NextSong,
                    tws_action: Action::PreviousSong,
                    enabled: true,
                },
            },
            right: ButtonSide {
                double_press: TwsButtonAction {
                    non_tws_action: Action::NextSong,
                    tws_action: Action::PreviousSong,
                    enabled: true,
                },
                single_press: NonTwsButtonAction {
                    action: Action::VoiceAssistant,
                    enabled: false,
                },
                long_press: TwsButtonAction {
                    non_tws_action: Action::NextSong,
                    tws_action: Action::PreviousSong,
                    enabled: true,
                },
            },
        };

        let expected = [
            0x01, 0x32, 0x01, 0x32, 0x01, 0x32, 0x01, 0x32, 0x00, 0x05, 0x00, 0x05,
        ];

        assert_eq!(expected, model.bytes());
    }
}
