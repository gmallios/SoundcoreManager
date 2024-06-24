use crate::{devices::A3040EqUpdateCommand, models::EQConfiguration, packets::Packet, types::KnownProductCodes};

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
            _ => self.find_builder(),
        }
    }

    fn find_builder(&self) -> Vec<u8> {
        todo!()
    }
}
