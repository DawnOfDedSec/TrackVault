use serde::{Deserialize, Serialize};

use crate::classes::TokenMetadata;

#[derive(Debug, Serialize, Deserialize)]
pub struct EchoResponse {
    pub status: String,
    pub latency: String,
    pub host_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentResponse {
    pub count: u16,
    pub agents: Vec<TokenMetadata>,
}
