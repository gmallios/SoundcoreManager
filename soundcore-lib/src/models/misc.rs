use serde::{Deserialize, Serialize};
use derive_more::From;

///
/// "Toggle" types, booleans that represent a toggleable feature and are parsed using the bool_parser.
///
 
#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
pub struct GameMode(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
pub struct BassUp(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
pub struct LDAC(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
pub struct InEarBeep(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
pub struct SupportTwoCnn(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
pub struct ThreeDimensionalEffect(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
pub struct SideTone(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
pub struct PowerOnBatteryNotice(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
pub struct TwsStatus(pub bool);

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
pub struct WearDetection(pub bool);


#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
pub struct TouchTone(pub bool);


#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
pub struct AgeRange(pub u8);
