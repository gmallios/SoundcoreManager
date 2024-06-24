use log::warn;

use crate::{
    devices::{A3040EqUpdateCommand, A3951EqUpdateCommand},
    models::EQConfiguration,
    packets::Packet,
    types::KnownProductCodes,
};

pub struct EqCommandBuilder {
    eq: EQConfiguration,
    model: KnownProductCodes,
}

impl EqCommandBuilder {
    pub fn new(eq: EQConfiguration, model: KnownProductCodes) -> Self {
        Self { eq, model }
    }

    pub fn build(self) -> Vec<u8> {
        match self.model {
            KnownProductCodes::A3040 => A3040EqUpdateCommand::new(self.eq).bytes(),
            KnownProductCodes::A3951 => A3951EqUpdateCommand::new(self.eq).bytes(),
            _ => {
                warn!("Unknown product code, using A3951 as default");
                A3951EqUpdateCommand::new(self.eq).bytes()
            }
        }
    }
}
