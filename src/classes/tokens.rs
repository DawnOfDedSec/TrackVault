use std::path::Path;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rusqlite::{Connection, Error, Result, Row};
use serde::{Deserialize, Serialize};

use crate::models::{TokenDatabaseError, TokenMetadataError};

#[derive(Debug, Clone)]
pub struct TokensDatabase {
    location: String, // Subject (host Id)
}

impl TokensDatabase {
    pub fn new(location: &str) -> Self {
        TokensDatabase {
            location: String::from(location),
        }
    }

    fn from_db_row(row: &Row) -> Result<TokenMetadata> {
        Ok(TokenMetadata::new(
            row.get::<&str, String>("id").unwrap(),
            row.get::<&str, u64>("exp").unwrap(),
        ))
    }

    pub fn connect(&self) -> Result<Connection, Error> {
        let path = Path::new(&self.location);
        Connection::open(path)
    }

    pub fn init(&self) -> Result<(), TokenDatabaseError> {
        let conn = match self.connect() {
            Ok(stmt) => stmt,
            Err(e) => return Err(TokenDatabaseError::CantPrepareQuery(e)),
        };

        let sql = "CREATE TABLE IF NOT EXISTS Tokens ( id TEXT PRIMARY KEY, exp INTEGER NOT NULL )";
        match conn.execute(sql, []) {
            Ok(_) => Ok(()),
            Err(e) => Err(TokenDatabaseError::CantExecuteQuery(String::from(sql), e)),
        }
    }

    pub fn get_tokens(&self) -> Result<Vec<TokenMetadata>, TokenDatabaseError> {
        let conn = match self.connect() {
            Ok(stmt) => stmt,
            Err(err) => {
                return Err(TokenDatabaseError::SQLiteConnectionError(
                    self.location.clone(),
                    err,
                ))
            }
        };

        let mut stmt = match conn.prepare("SELECT id, exp FROM Tokens") {
            Ok(stmt) => stmt,
            Err(err) => return Err(TokenDatabaseError::CantPrepareQuery(err)),
        };

        let tokens: Result<Vec<TokenMetadata>, _> = stmt
            .query_map([], TokensDatabase::from_db_row)
            .and_then(|rows_tokens| rows_tokens.collect());

        tokens.map_err(TokenDatabaseError::CantFetchTokens)
    }

    pub fn get_token(&self, id: &str) -> Result<TokenMetadata, TokenDatabaseError> {
        let conn = match self.connect() {
            Ok(stmt) => stmt,
            Err(e) => return Err(TokenDatabaseError::CantPrepareQuery(e)),
        };

        let mut stmt = match conn.prepare("SELECT id, exp FROM Tokens WHERE id = :id") {
            Ok(stmt) => stmt,
            Err(e) => return Err(TokenDatabaseError::CantPrepareQuery(e)),
        };

        match stmt.query_row(&[(":id", &id)], TokensDatabase::from_db_row) {
            Ok(token) => Ok(token),
            Err(e) => Err(TokenDatabaseError::CantFetchToken(e)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct TokenMetadata {
    pub id: String, // Subject (host Id)
    pub exp: usize, //
}

impl TokenMetadata {
    pub fn new(id: String, exp_in_hour: u64) -> Self {
        TokenMetadata {
            id,
            exp: (Utc::now() + Duration::hours(exp_in_hour as i64)).timestamp() as usize,
        }
    }

    pub fn update_exp(&mut self, exp_in_hour: u64) -> () {
        self.exp = (Utc::now() + Duration::hours(exp_in_hour as i64)).timestamp() as usize
    }

    pub fn parse(token: &str, base64_key: &str) -> Result<TokenMetadata, TokenMetadataError> {
        match decode::<TokenMetadata>(
            &token,
            &DecodingKey::from_base64_secret(&base64_key).unwrap(),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(data) => Ok(data.claims),
            Err(_) => Err(TokenMetadataError::TokenDecodingError(String::from(token))),
        }
    }

    pub fn generate(&self, base64_key: String) -> Result<String, jsonwebtoken::errors::Error> {
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_base64_secret(&base64_key).unwrap(),
        )
    }
}
