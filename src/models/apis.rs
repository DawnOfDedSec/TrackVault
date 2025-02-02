use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::helpers::tokenization::TokenManager;

#[derive(Debug, Serialize, Deserialize)]
pub struct EchoResponse {
    pub status: String,
    pub latency: String,
    pub host_id: String,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Token {
    pub host_id: String,
    pub value: String,
}

#[derive(Clone)]
pub struct AppState {
    pub token_manager: Arc<Mutex<TokenManager>>,
}
