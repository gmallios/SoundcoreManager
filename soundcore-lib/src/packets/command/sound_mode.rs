use crate::{
    devices::A3040SoundModeUpdateCommand, models::SoundMode, packets::Packet,
    types::SupportedModels,
};

pub struct SoundModeCommandBuilder {
    sound_mode: SoundMode,
    model: SupportedModels,
}

impl SoundModeCommandBuilder {
    pub fn new(sound_mode: SoundMode, model: SupportedModels) -> Self {
        Self { sound_mode, model }
    }

    pub fn build(self) -> Vec<u8> {
        match self.model {
            SupportedModels::A3040 => A3040SoundModeUpdateCommand::new(self.sound_mode).bytes(),
            // TODO: use a default comamnd A3951?
            _ => panic!("Unsupported model"),
        }
    }
}
