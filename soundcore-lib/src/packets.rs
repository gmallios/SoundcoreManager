mod command;
mod request;
mod response;

pub use command::*;
pub use request::*;
pub use response::*;

const COMMAND_BYTE_SIZE: usize = 2;
const PACKET_PREFIX: [u8; 5] = [0x08, 0xEE, 0x00, 0x00, 0x00];
pub trait SoundcorePacket {

    type ByteArr;
    /// Returns the packet's bytes + checksum
    fn bytes(&self) -> Self::ByteArr;
}

impl<T> SoundcorePacket for T
where
    T: RequestPacket,
{
    type ByteArr = [u8; COMMAND_BYTE_SIZE + PACKET_PREFIX.len() + 3];

    fn bytes(&self) -> Self::ByteArr {
        let mut bytes = [0; COMMAND_BYTE_SIZE + PACKET_PREFIX.len() + 3];
        // Add the prefix
        bytes[..PACKET_PREFIX.len()].copy_from_slice(&PACKET_PREFIX);
        // Add the command bytes
        bytes[PACKET_PREFIX.len()..PACKET_PREFIX.len() + COMMAND_BYTE_SIZE]
            .copy_from_slice(&self.default_bytes());

        bytes
    }
}
