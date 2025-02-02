use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::classes::TokensDatabase;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiEchoResponse {
    pub status: String,
    pub latency: String,
    pub host_id: String,
}

#[derive(Clone)]
pub struct AppState {
    pub token_database: Arc<Mutex<TokensDatabase>>,
}
