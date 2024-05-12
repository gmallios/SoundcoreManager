use crate::{models::SoundMode, packets::Packet};

pub struct A3951SoundModeUpdateCommand {
    sound_mode: SoundMode,
}

impl A3951SoundModeUpdateCommand {
    pub fn new(sound_mode: SoundMode) -> Self {
        Self { sound_mode }
    }
}

impl Packet for A3951SoundModeUpdateCommand {
    fn command(&self) -> [u8; 7] {
        [0x08, 0xEE, 0x00, 0x00, 0x00, 0x06, 0x81]
    }

    fn payload(&self) -> Vec<u8> {
        self.sound_mode.to_bytes().to_vec()
    }
}
