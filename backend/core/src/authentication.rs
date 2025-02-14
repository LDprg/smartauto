use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use lazy_static::lazy_static;
use ring::signature::KeyPair;
use serde::{Deserialize, Serialize};
use tonic::{Request, Status};

use crate::constants::*;
use crate::*;

lazy_static! {
    pub static ref auth_keys: AuthKeys = AuthKeys::default();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

#[tracing::instrument(level = "trace", skip(req))]
pub fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let token = resolve_error!(
        resolve_error!(try_required!(
            req.metadata().get(AUTH),
            "authorization header"
        ))?
        .to_str()
    )?;

    if !token.starts_with(BEARER) {
        return Err(Status::unauthenticated("Auth token malformed"));
    }
    let token = token.trim_start_matches(BEARER);

    let token = decode::<Claims>(
        token,
        &auth_keys.decoding_key,
        &Validation::new(Algorithm::EdDSA),
    );

    match token {
        Ok(_) => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}

pub fn generate_pwd_hash(password: &str) -> Result<String, Status> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = resolve_error!(argon2.hash_password(password.as_bytes(), &salt))?.to_string();

    Ok(hash)
}

pub fn validate_pwd_hash(password: &str, hash: &str) -> Result<(), Status> {
    let parsed_hash = resolve_error!(PasswordHash::new(hash))?;
    let err = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);

    if let Err(err) = err {
        error!(%err);
        return Err(Status::unauthenticated("Invalid Password"));
    }
    Ok(())
}

pub struct AuthKeys {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
}

impl Default for AuthKeys {
    #[tracing::instrument(level = "trace", skip())]
    fn default() -> Self {
        let rng = ring::rand::SystemRandom::new();
        let pkcs8_bytes =
            ring::signature::Ed25519KeyPair::generate_pkcs8(&rng).expect("generate pkcs8");
        let key_pair = ring::signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
            .expect("conversion to key pair");

        let enc_key = EncodingKey::from_ed_der(pkcs8_bytes.as_ref());
        let dec_key = DecodingKey::from_ed_der(key_pair.public_key().as_ref());

        Self {
            encoding_key: enc_key,
            decoding_key: dec_key,
        }
    }
}

impl AuthKeys {
    #[tracing::instrument(level = "trace", skip(self, sub))]
    pub fn generate_token(&self, sub: &str) -> Result<String, Status> {
        let now = Utc::now().timestamp();

        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::hours(12))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            exp: expiration as usize,
            iat: now as usize,
            nbf: now as usize,
            sub: sub.to_string(),
        };

        resolve_error!(encode(
            &Header::new(Algorithm::EdDSA),
            &claims,
            &auth_keys.encoding_key,
        ))
    }
}
