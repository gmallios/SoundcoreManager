use log::warn;

use crate::devices::A3040BassUpCommand;
use crate::packets::Packet;
use crate::types::KnownProductCodes;

pub struct BassUpCommandBuilder {
    product_code: KnownProductCodes,
    enable: bool,
}

impl BassUpCommandBuilder {
    pub fn new(product_code: KnownProductCodes, enable: bool) -> Self {
        Self {
            product_code,
            enable,
        }
    }

    pub fn build(&self) -> Vec<u8> {
        match self.product_code {
            KnownProductCodes::A3040 => A3040BassUpCommand::new(self.enable).bytes(),
            _ => {
                warn!("Unknown or unhandled product code, using A3951 as default");
                A3040BassUpCommand::new(self.enable).bytes()
            }
        }
    }
}
