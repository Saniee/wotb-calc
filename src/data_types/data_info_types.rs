use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct DataInfo {
    pub status: String,
    pub data: Data,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Data {
    pub tanks_updated_at: i64,
    pub game_version: String,
}
