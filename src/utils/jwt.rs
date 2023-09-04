use chrono::Utc;
use jsonwebtoken::{errors::Result, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref DECODE_KEY: DecodingKey =
        DecodingKey::from_rsa_pem(include_bytes!("key/pub.key")).expect("public key parse failed");
    static ref ENCODE_KEY: EncodingKey =
        EncodingKey::from_rsa_pem(include_bytes!("key/pri.key")).expect("private key parse failed");
}

const ISSUER: &'static str = "AutoWDS";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub uid: i64,
    iss: String,
    exp: usize,
}

impl Claims {
    pub fn new(uid: i64) -> Self {
        Self {
            uid,
            iss: String::from(ISSUER),
            exp: (Utc::now().timestamp() + 360 * 24 * 60 * 60) as usize,
        }
    }
}

/// JWT编码
pub fn encode(claims: Claims) -> Result<String> {
    jsonwebtoken::encode::<Claims>(&Header::new(Algorithm::RS256), &claims, &ENCODE_KEY)
}

/// JWT解码
pub fn decode(token: &str) -> Result<Claims> {
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[ISSUER]);
    jsonwebtoken::decode::<Claims>(&token, &DECODE_KEY, &validation)
        .map(|token_data| token_data.claims)
}
