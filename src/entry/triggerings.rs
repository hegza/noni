use crate::Time;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Triggerings {
    Single(Time, bool),
    Multi(Vec<(Time, bool)>),
}
