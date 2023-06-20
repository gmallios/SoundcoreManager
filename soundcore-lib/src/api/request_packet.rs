pub trait RequestPacket {
    fn bytes(&self) -> Vec<u8>;
}
