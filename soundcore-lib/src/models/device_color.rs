use derive_more::From;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// This u8 is a char which based on it fetches the apropriate image for the device (Reference: OtaBaseActivity/renderProductIcon).
/// Perhaps this can be internally mapped to our product images.
/// Further investigation is needed.
#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
#[typeshare]
pub struct DeviceColor(pub u8);
