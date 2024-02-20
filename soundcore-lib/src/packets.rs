pub use request::*;
pub use response::*;

use crate::parsers::generate_checksum;

mod command;
mod request;
mod response;

const PACKET_SIZE_LENGTH: usize = 2;
const CHECKSUM_BIT_LENGTH: usize = 1;

pub trait Packet {
    fn command(&self) -> [u8; 7];
    fn payload(&self) -> Vec<u8>;
    fn bytes(&self) -> Vec<u8> {
        let (command, payload) = (self.command(), self.payload());
        let length_bytes: [u8; PACKET_SIZE_LENGTH] =
            ((command.len() + PACKET_SIZE_LENGTH + payload.len() + CHECKSUM_BIT_LENGTH) as u16)
                .to_le_bytes();
        let mut bytes = vec![command.to_vec(), length_bytes.to_vec(), payload].concat();
        bytes.push(generate_checksum(&bytes));

        bytes
    }
}
