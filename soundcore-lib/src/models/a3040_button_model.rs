use crate::models::Action;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
#[typeshare]
pub struct A3040ButtonModel {
    pub single_click: Action,
    pub double_click: Action,
}
