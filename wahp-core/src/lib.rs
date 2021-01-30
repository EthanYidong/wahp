use serde::{Serialize, Deserialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WahpRequest {
    pub query: HashMap<String, String>
}

impl WahpRequest {
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn from_bytes(bytes: &[u8]) -> WahpRequest {
        bincode::deserialize(bytes).unwrap()
    }
}
