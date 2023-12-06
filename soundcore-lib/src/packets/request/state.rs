use serde::{Deserialize, Serialize};

use crate::{packets::COMMAND_BYTE_SIZE, types::SupportedModels};

use super::RequestPacket;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
struct RequestStatePacket {
    variant: Option<SupportedModels>,
}

impl RequestStatePacket {
    const DEFAULT: [u8; COMMAND_BYTE_SIZE] = [0x01, 0x01];

    fn new() -> Self {
        Self { variant: None }
    }

    fn with_variant(variant: SupportedModels) -> Self {
        Self {
            variant: Some(variant),
        }
    }
}

impl RequestPacket for RequestStatePacket {
    fn default_bytes(&self) -> [u8; COMMAND_BYTE_SIZE] {
        match self.variant {
            Some(variant) => self.variant_bytes(variant),
            None => Self::DEFAULT,
        }
    }

    fn variant_bytes(&self, variant: SupportedModels) -> [u8; COMMAND_BYTE_SIZE] {
        match variant {
            SupportedModels::A3951 => Self::DEFAULT,
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod request_state_packet {
    use super::*;

    #[test]
    fn test_default() {
        let packet = RequestStatePacket::new();
        assert_eq!(packet.default_bytes(), RequestStatePacket::DEFAULT);
    }
}
