use crate::types::SupportedModels;

use super::COMMAND_BYTE_SIZE;

/// Request Packets are used to request data from the device.
pub trait RequestPacket {
    /// Returns the default command bytes without checksum
    fn default_bytes(&self) -> [u8; COMMAND_BYTE_SIZE];
    /// Returns the command bytes for the specified variant without checksum
    fn variant_bytes(&self, variant: SupportedModels) -> [u8; COMMAND_BYTE_SIZE];
}

mod state;


