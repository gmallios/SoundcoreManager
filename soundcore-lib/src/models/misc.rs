use derive_more::From;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

///
/// "Toggle" types, booleans that represent a toggleable feature and are parsed using the bool_parser.
///

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
#[typeshare]
pub struct GameMode(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
#[typeshare]
pub struct BassUp(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
#[typeshare]
pub struct LDAC(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
#[typeshare]
pub struct InEarBeep(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
#[typeshare]
pub struct SupportTwoCnn(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
#[typeshare]
pub struct ThreeDimensionalEffect(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
#[typeshare]
pub struct SideTone(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
#[typeshare]
pub struct PowerOnBatteryNotice(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
#[typeshare]
pub struct TwsStatus(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
#[typeshare]
pub struct WearDetection(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
#[typeshare]
pub struct TouchTone(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
#[typeshare]
pub struct AgeRange(pub u8);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
#[typeshare]
pub struct AmbientSoundNotice(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
#[typeshare]
pub struct LeakyCompensation(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
#[typeshare]
pub struct MediaTone(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
#[typeshare]
pub struct CustomButtonWearEnable(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
#[typeshare]
pub struct ChargingCaseBattery(pub u8);
