use crate::packets::Packet;
use crate::types::KnownProductCodes;

pub enum RequestPacketKind {
    State,
    Info,
    BatteryLevel,
    BatteryStatus,
}

pub struct RequestPacketBuilder {
    kind: RequestPacketKind,
    model: Option<KnownProductCodes>,
}

// TODO: Add different packets for different models (if required)
impl RequestPacketBuilder {
    pub fn new(kind: RequestPacketKind) -> Self {
        Self { kind, model: None }
    }

    pub fn model(mut self, model: KnownProductCodes) -> Self {
        self.model = Some(model);
        self
    }

    pub fn build(self) -> Vec<u8> {
        self.bytes()
    }

    fn state_request(&self) -> [u8; 7] {
        [0x08, 0xEE, 0x00, 0x00, 0x00, 0x01, 0x01]
    }

    fn info_request(&self) -> [u8; 7] {
        [0x08, 0xEE, 0x00, 0x00, 0x00, 0x01, 0x05]
    }

    fn battery_level(&self) -> [u8; 7] {
        [0x08, 0xEE, 0x00, 0x00, 0x00, 0x01, 0x03]
    }

    fn battery_status(&self) -> [u8; 7] {
        [0x08, 0xEE, 0x00, 0x00, 0x00, 0x01, 0x04]
    }
}

impl Packet for RequestPacketBuilder {
    fn command(&self) -> [u8; 7] {
        match self.kind {
            RequestPacketKind::State => self.state_request(),
            RequestPacketKind::Info => self.info_request(),
            RequestPacketKind::BatteryLevel => self.battery_level(),
            RequestPacketKind::BatteryStatus => self.battery_status(),
        }
    }

    fn payload(&self) -> Vec<u8> {
        // No payload for request packets
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn state_request_default() {
        let packet = RequestPacketBuilder::new(RequestPacketKind::State).build();
        assert_eq!(
            packet,
            [0x08, 0xEE, 0x00, 0x00, 0x00, 0x01, 0x01, 0x0A, 0x00, 0x02]
        );
    }

    #[test]
    fn info_request_default() {
        let packet = RequestPacketBuilder::new(RequestPacketKind::Info).build();
        assert_eq!(
            packet,
            [0x08, 0xEE, 0x00, 0x00, 0x00, 0x01, 0x05, 0x0A, 0x00, 0x06]
        );
    }

    #[test]
    fn battery_level_default() {
        let packet = RequestPacketBuilder::new(RequestPacketKind::BatteryLevel).build();
        assert_eq!(
            packet,
            vec![0x08, 0xEE, 0x00, 0x00, 0x00, 0x01, 0x03, 0x0A, 0x00, 0x04]
        );
    }

    #[test]
    fn battery_status_default() {
        let packet = RequestPacketBuilder::new(RequestPacketKind::BatteryStatus).build();
        assert_eq!(
            packet,
            vec![0x08, 0xEE, 0x00, 0x00, 0x00, 0x01, 0x04, 0x0A, 0x00, 0x05]
        );
    }
}
