use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatHistory {
    pub id: String,
    pub messages: String,
}
