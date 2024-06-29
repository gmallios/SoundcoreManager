use crate::packets::Packet;

pub struct A3040BassUpCommand {
    pub enable: bool,
}

impl A3040BassUpCommand {
    pub fn new(enable: bool) -> Self {
        Self { enable }
    }
}

impl Packet for A3040BassUpCommand {
    fn command(&self) -> [u8; 7] {
        [0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x84]
    }

    fn payload(&self) -> Vec<u8> {
        [if self.enable { 0x01 } else { 0x00 }].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn enable_bass_up() {
        let command = super::A3040BassUpCommand::new(true);
        assert_eq!(command.bytes(), test_data::a3040::BASS_UP_UPDATE_ENABLE.to_vec());
    }

    #[test]
    pub fn disable_bass_up() {
        let command = super::A3040BassUpCommand::new(false);
        assert_eq!(command.bytes(), test_data::a3040::BASS_UP_UPDATE_DISABLE.to_vec());
    }
}
