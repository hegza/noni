use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tag {
    name: String,
    aliases: Vec<String>,
}
