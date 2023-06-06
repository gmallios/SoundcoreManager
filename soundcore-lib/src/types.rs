pub(crate) trait ResponsePacket<T> { 
    
}

pub(crate) trait CommandPacket { 
    fn bytes(&self) -> Vec<u8>;
}