use std::{env, path::Path};

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rusqlite::{Connection, Error, Result, Row};
use serde::{Deserialize, Serialize};

use crate::{
    models::{TokenDatabaseError, TokenMetadataError},
    utils::to_base64,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TokenClaim {
    id: String,
    exp: usize,
}

impl TokenClaim {
    fn new(id: String, exp: usize) -> Self {
        TokenClaim { id, exp }
    }
}

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

        let sql = "CREATE TABLE IF NOT EXISTS Tokens ( id TEXT PRIMARY KEY, exp INTEGER NOT NULL, exp TEXT NOT NULL )";
        match conn.execute(sql, []) {
            Ok(_) => Ok(()),
            Err(e) => Err(TokenDatabaseError::CantExecuteQuery(String::from(sql), e)),
        }
    }

    pub fn get_all(&self) -> Result<Vec<TokenMetadata>, TokenDatabaseError> {
        let conn = match self.connect() {
            Ok(stmt) => stmt,
            Err(err) => {
                return Err(TokenDatabaseError::SQLiteConnectionError(
                    self.location.clone(),
                    err,
                ))
            }
        };

        let mut stmt = match conn.prepare("SELECT id, exp, token FROM Tokens") {
            Ok(stmt) => stmt,
            Err(err) => return Err(TokenDatabaseError::CantPrepareQuery(err)),
        };

        let tokens: Result<Vec<TokenMetadata>, _> = stmt
            .query_map([], TokensDatabase::from_db_row)
            .and_then(|rows_tokens| rows_tokens.collect());

        tokens.map_err(TokenDatabaseError::CantFetchTokens)
    }

    pub fn get(
        &self,
        token: Option<&str>,
        id: Option<&str>,
    ) -> Result<TokenMetadata, TokenDatabaseError> {
        let conn = match self.connect() {
            Ok(stmt) => stmt,
            Err(e) => return Err(TokenDatabaseError::CantPrepareQuery(e)),
        };

        let mut stmt =
            match conn.prepare("SELECT id, exp, token FROM Tokens WHERE :column = :value") {
                Ok(stmt) => stmt,
                Err(e) => return Err(TokenDatabaseError::CantPrepareQuery(e)),
            };

        if id.is_none() {
            match stmt.query_row(
                &[(":column", "token"), (":value", token.unwrap())],
                TokensDatabase::from_db_row,
            ) {
                Ok(token) => Ok(token),
                Err(e) => Err(TokenDatabaseError::CantFetchToken(e)),
            }
        } else {
            match stmt.query_row(
                &[(":column", "id"), (":value", id.unwrap())],
                TokensDatabase::from_db_row,
            ) {
                Ok(token) => Ok(token),
                Err(e) => Err(TokenDatabaseError::CantFetchToken(e)),
            }
        }
    }

    pub fn get_from_id(&self, id: &str) -> Result<TokenMetadata, TokenDatabaseError> {
        let conn = match self.connect() {
            Ok(stmt) => stmt,
            Err(e) => return Err(TokenDatabaseError::CantPrepareQuery(e)),
        };

        let mut stmt = match conn.prepare("SELECT id, exp, token FROM Tokens WHERE id = :id") {
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
    pub id: String,            // Subject (host Id)
    pub exp: usize,            //
    pub token: Option<String>, // JWT token
}

impl TokenMetadata {
    pub fn new(id: String, exp_in_hour: u64) -> Self {
        TokenMetadata {
            id,
            exp: (Utc::now() + Duration::hours(exp_in_hour as i64)).timestamp() as usize,
            token: None,
        }
    }

    pub fn update(&mut self, exp_in_hour: u64) -> () {
        self.exp = (Utc::now() + Duration::hours(exp_in_hour as i64)).timestamp() as usize;
        match self.generate(String::from(env::var("JWT_SECRET").unwrap())) {
            Ok(token) => self.token = Some(token),
            Err(_) => (),
        }
    }

    pub fn parse(token: &str, secret_key: &str) -> Result<TokenMetadata, TokenMetadataError> {
        match decode::<TokenMetadata>(
            &token,
            &DecodingKey::from_base64_secret(&to_base64(&secret_key)).unwrap(),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(data) => Ok(data.claims),
            Err(_) => Err(TokenMetadataError::TokenDecodingError(String::from(token))),
        }
    }

    pub fn generate(&self, secret_key: String) -> Result<String, jsonwebtoken::errors::Error> {
        encode(
            &Header::default(),
            &TokenClaim::new(self.id.clone(), self.exp.clone()),
            &EncodingKey::from_base64_secret(&to_base64(&secret_key)).unwrap(),
        )
    }
}
