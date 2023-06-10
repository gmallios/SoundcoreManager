pub enum ResponsePacket {}

pub trait CommandPacket {
    fn bytes(&self) -> Vec<u8>;
}
