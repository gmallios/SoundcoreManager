use crate::models::{BassUp, MonoEQConfiguration};
use crate::packets::Packet;

pub struct A3040EqUpdateCommand {
    eq: MonoEQConfiguration,
    bass_up: BassUp,
}

impl Packet for A3040EqUpdateCommand {
    fn command(&self) -> [u8; 7] {
        match self.bass_up.0 {
            true => [0x08, 0xEE, 0x00, 0x00, 0x00, 0x02, 0x84],
            false => [0x08, 0xEE, 0x00, 0x00, 0x00, 0x03, 0x87],
        }
    }

    fn payload(&self) -> Vec<u8> {
        // 2 bytes profile - FEFE - Custom
        todo!()
    }
}
