use std::path::Path;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rusqlite::{Connection, Error, Result, Row};
use serde::{Deserialize, Serialize};

use crate::models::errors::{TokenError, TokenManagerError};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct TokenClaim {
    pub id: String, // Subject (host Id)
    pub exp: usize, // Expiration time (UNIX timestamp)
}

#[derive(Debug, Clone)]
pub struct TokenManager {
    location: String, // Subject (host Id)
}

impl TokenManager {
    pub fn new(location: String) -> Self {
        TokenManager { location }
    }

    fn from_row(row: &Row) -> Result<Token> {
        Ok(Token::new(
            row.get::<&str, String>("id").unwrap(),
            row.get::<&str, i64>("exp").unwrap(),
        ))
    }

    pub fn connect(&self) -> Result<Connection, Error> {
        let path = Path::new(&self.location);
        Connection::open(path)
    }

    pub fn get_tokens(&self) -> Result<Vec<Token>, TokenManagerError> {
        let conn = match self.connect() {
            Ok(stmt) => stmt,
            Err(e) => return Err(TokenManagerError::CantPrepareQuery(e)),
        };

        let mut stmt = match conn.prepare("SELECT id, exp FROM Tokens") {
            Ok(stmt) => stmt,
            Err(e) => return Err(TokenManagerError::CantPrepareQuery(e)),
        };

        let tokens: Result<Vec<Token>, _> = stmt
            .query_map([], TokenManager::from_row)
            .and_then(|rows_tokens| rows_tokens.collect());

        tokens.map_err(TokenManagerError::CantFetchTokens)
    }

    pub fn get_token(&self, id: &str) -> Result<Token, TokenManagerError> {
        let conn = match self.connect() {
            Ok(stmt) => stmt,
            Err(e) => return Err(TokenManagerError::CantPrepareQuery(e)),
        };

        let mut stmt = match conn.prepare("SELECT id, exp FROM Tokens WHERE id = :id") {
            Ok(stmt) => stmt,
            Err(e) => return Err(TokenManagerError::CantPrepareQuery(e)),
        };

        match stmt.query_row(&[(":id", &id)], TokenManager::from_row) {
            Ok(token) => Ok(token),
            Err(e) => Err(TokenManagerError::CantFetchToken(e)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Token {
    pub id: String, // Subject (host Id)
    pub exp: i64,   // Expiration time (Hour based)
}

impl Token {
    pub fn new(id: String, exp: i64) -> Self {
        Token { id, exp }
    }

    pub fn create(&self, base64_key: String) -> Result<String, jsonwebtoken::errors::Error> {
        let expiration = (Utc::now() + Duration::hours(self.exp)).timestamp() as usize;
        let claims = TokenClaim {
            id: self.id.clone(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_base64_secret(&base64_key).unwrap(),
        )
    }

    pub fn parse(token: &str, base64_key: &str) -> Result<TokenClaim, TokenError> {
        match decode::<TokenClaim>(
            &token,
            &DecodingKey::from_base64_secret(&base64_key).unwrap(),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(data) => Ok(data.claims),
            Err(_) => Err(TokenError::TokenDecodingError(String::from(token))),
        }
    }
}
