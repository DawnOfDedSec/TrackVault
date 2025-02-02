use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogManager {
    location: String,
}

impl LogManager {
    pub fn new(location: &str) -> Self {
        LogManager {
            location: location.to_string(),
        }
    }

    pub fn print(category: Option<&str>, msg: &str) {
        println!("[{}] {}", category.unwrap_or("Others"), msg);
    }

    pub fn eprint<E: fmt::Debug + fmt::Display>(category: Option<&str>, err: E) {
        println!("[{}] {:?}", category.unwrap_or("Others"), err);
    }
}
