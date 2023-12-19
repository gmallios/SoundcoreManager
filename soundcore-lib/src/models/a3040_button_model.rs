use crate::models::Action;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
pub struct A3040ButtonModel {
    pub single_click: Action,
    pub double_click: Action,
}
