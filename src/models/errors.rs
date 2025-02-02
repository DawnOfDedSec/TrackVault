use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Unexpected token: {0}")]
    UnexpectedToken(String),
    #[error("Token \"{0}\" can't be decoded with the Secret Key")]
    TokenDecodingError(String),
    #[error("Token \"{0}\" has been expired")]
    TokenExpired(String),
}

#[derive(Error, Debug)]
pub enum TokenManagerError {
    #[error("SQlite database can't fetch tokens: {0}")]
    CantFetchTokens(rusqlite::Error),
    #[error("SQlite database can't fetch token: {0}")]
    CantFetchToken(rusqlite::Error),
    #[error("SQlite can't prepare Query with Query Statements: {0}")]
    CantPrepareQuery(rusqlite::Error),
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Unauthorized Token has been found: {0}")]
    UnAuthorizedToken(String),
    #[error("Invalid Token has been found: {0}")]
    InvalidToken(String),
    #[error("Bearer Token is not provided")]
    MissingBearerToken,
}
