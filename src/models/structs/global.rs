use std::sync::{Arc, Mutex};

use crate::classes::TokensDatabase;

#[derive(Clone)]
pub struct AppState {
    pub token_database: Arc<Mutex<TokensDatabase>>,
}
