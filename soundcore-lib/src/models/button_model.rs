use serde::{Deserialize, Serialize};

use super::A3909ButtonModel;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ButtonModel {
    A3909(A3909ButtonModel),
}
