use rusqlite;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenMetadataError {
    #[error("Unexpected token: \'{0}\'")]
    UnexpectedToken(String),
    #[error("Token \'{0}\' can't be decoded with the Secret Key")]
    TokenDecodingError(String),
    #[error("Token \'{0}\' has been expired")]
    TokenExpired(String),
}

#[derive(Error, Debug)]
pub enum TokenDatabaseError {
    #[error("SQlite database can't fetch tokens -> {0}")]
    CantFetchTokens(rusqlite::Error),
    #[error("SQlite database can't fetch token -> {0}")]
    CantFetchToken(rusqlite::Error),
    #[error("SQlite can't prepare Query with Query Statements -> {0}")]
    CantPrepareQuery(rusqlite::Error),
    #[error("SQlite database can't connect to the database: \'{0}\' -> {1}")]
    SQLiteConnectionError(String, rusqlite::Error),
    #[error("SQlite database can't execute to the database: \'{0}\' -> {1}")]
    CantExecuteQuery(String, rusqlite::Error),
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Unauthorized Token has been found: \'{0}\'")]
    UnAuthorizedToken(String),
    #[error("Invalid or Empty Token has been found: \'{0}\'")]
    InvalidToken(String),
    #[error("Bearer Token is not provided")]
    MissingBearerToken,
    #[error("Api Server faced an internal issue: \'{0}\'")]
    InternalServerError(String),
    #[error("Api Server faced an runtime issue: \'{0}\'")]
    RuntimeServerError(io::Error),
}
