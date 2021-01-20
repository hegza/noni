use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DisplaySettings {}

impl Default for DisplaySettings {
    fn default() -> Self {
        DisplaySettings {}
    }
}
