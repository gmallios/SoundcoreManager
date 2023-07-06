/// Structs that implement this trait are used to Set/Request data
pub trait RequestPacket {
    fn bytes(&self) -> Vec<u8>;
}
