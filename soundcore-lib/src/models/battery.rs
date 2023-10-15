use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Battery {
    Single(SingleBattery),
    Dual(DualBattery),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
#[serde(rename_all = "camelCase", tag = "type")]
pub struct DualBattery {
    pub left: SingleBattery,
    pub right: SingleBattery,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
#[serde(rename_all = "camelCase", tag = "type")]
pub struct SingleBattery {
    pub charging: bool,
    pub level: u8,
}

impl From<SingleBattery> for Battery {
    fn from(b: SingleBattery) -> Self {
        Battery::Single(b)
    }
}

impl From<DualBattery> for Battery {
    fn from(b: DualBattery) -> Self {
        Battery::Dual(b)
    }
}

impl Default for SingleBattery {
    fn default() -> Self {
        SingleBattery {
            charging: false,
            level: u8::MAX,
        }
    }
}

impl Default for Battery {
    fn default() -> Self {
        Battery::Single(SingleBattery::default())
    }
}
