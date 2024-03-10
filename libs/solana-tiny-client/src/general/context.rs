use crate::general::slot::Slot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    pub slot: Slot,
}
