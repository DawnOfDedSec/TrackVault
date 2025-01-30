use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// The `Claims` struct represents claims with host ID and expiration time in Rust.
/// 
/// Properties:
/// 
/// * `host_id`: The `host_id` property in the `Claims` struct represents the Subject (host Id) in the
/// claims. It is a String type field.
/// * `exp`: The `exp` property in the `Claims` struct represents the expiration time of the token in
/// UNIX timestamp format. This timestamp indicates the date and time after which the token is no longer
/// considered valid.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    host_id: String, // Subject (host Id)
    exp: usize,      // Expiration time (UNIX timestamp)
}

/// The `encode_token` function generates a token with an expiration time based on the provided host ID,
/// secret key, and expiration hour.
/// 
/// Arguments:
/// 
/// * `host_id`: The `host_id` parameter is a `String` representing the identifier of the host for whom
/// the token is being generated.
/// * `secret_key`: The `secret_key` parameter is a reference to a string slice (`&str`) that represents
/// the secret key used for encoding the token.
/// * `exp_hour`: The `exp_hour` parameter represents the number of hours from the current time that the
/// token should expire.
/// 
/// Returns:
/// 
/// The `encode_token` function returns a `String` which is the encoded token generated based on the
/// input parameters provided.
#[warn(dead_code)]
pub fn encode_token(host_id: String, secret_key: &str, exp_hour: u8) -> String {
    let expiration = (Utc::now() + Duration::hours(exp_hour as i64)).timestamp() as usize;
    let claims = Claims {
        host_id: host_id.to_owned(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .expect("Failed to create token")
}

/// The `decode_token` function decodes a token using a secret key and returns the claims if successful.
/// 
/// Arguments:
/// 
/// * `token`: The `token` parameter is a reference to a `String` containing the token that needs to be
/// decoded.
/// * `secret_key`: The `secret_key` parameter in the `decode_token` function is a string reference
/// (`&str`) that represents the secret key used for decoding the token. This secret key is used along
/// with the token to verify the authenticity of the token and decode its contents.
/// 
/// Returns:
/// 
/// The `decode_token` function returns a `Result` containing either the decoded `Claims` if successful
/// or an error of type `jsonwebtoken::errors::Error` if decoding fails.
#[warn(dead_code)]
pub fn decode_token(
    token: &String,
    secret_key: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    match decode::<Claims>(
        token.as_str(),
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(data) => Ok(data.claims),
        Err(err) => Err(err),
    }
}
