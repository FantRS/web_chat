use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Serialize, de::DeserializeOwned};

use crate::app::request_error::RequestResult;

pub fn encode_jwt<T>(claims: T, secret: &str) -> RequestResult<String>
where
    T: Serialize,
{
    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(From::from)
}

pub fn decode_jwt<T>(token: &str, secret: &str) -> RequestResult<TokenData<T>>
where
    T: DeserializeOwned,
{
    jsonwebtoken::decode::<T>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(From::from)
}
