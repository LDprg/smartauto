use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use lazy_static::lazy_static;
use ring::{self, signature::KeyPair};
use serde::{Deserialize, Serialize};
use tonic::{Request, Response, Status};
use tracing::*;

use crate::constants::*;
use crate::smartauto::*;
use crate::*;

pub use crate::smartauto::auth_service_server::{AuthService, AuthServiceServer};

lazy_static! {
    static ref auth_keys: AuthKeys = AuthKeys::default();
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
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

struct AuthKeys {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl Default for AuthKeys {
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

#[derive(Debug, Default)]
pub struct AuthImpl {}

#[tonic::async_trait]
impl AuthService for AuthImpl {
    #[tracing::instrument(level = "trace", skip(self, request))]
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        info!("Got a request from {:?}", request.remote_addr());

        let now = Utc::now().timestamp();

        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::hours(12))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            exp: expiration as usize,
            iat: now as usize,
            nbf: now as usize,
            sub: "admin".to_string(),
        };

        let token = resolve_error!(encode(
            &Header::new(Algorithm::EdDSA),
            &claims,
            &auth_keys.encoding_key,
        ))?;

        let response = LoginResponse {
            access_token: token,
        };
        Ok(Response::new(response))
    }
}
