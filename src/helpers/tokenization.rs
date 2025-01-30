use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    host_id: String, // Subject (host Id)
    exp: usize,      // Expiration time (UNIX timestamp)
}

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

pub fn decode_token(
    token: &String,
    secret_key: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token.as_str(),
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(data.claims)
}
