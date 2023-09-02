use jsonwebtoken::{errors::Result, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

const PRI_KEY: &str = include_str!("key/pri.key");
const PUB_KEY: &str = include_str!("key/pub.key");

lazy_static! {
    static ref DECODE_KEY: DecodingKey = DecodingKey::from_secret(PRI_KEY.as_ref());
    static ref ENCODE_KEY: EncodingKey = EncodingKey::from_secret(PUB_KEY.as_ref());
}

const ISSUER: &'static str = "AutoWDS";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub uid: i64,
    iss: String,
}

impl Claims {
    pub fn new(uid: i64) -> Self {
        Self {
            uid,
            iss: String::from(ISSUER),
        }
    }
}

/// JWT编码
pub fn encode(claims: Claims) -> Result<String> {
    jsonwebtoken::encode::<Claims>(&Header::default(), &claims, &ENCODE_KEY)
}

/// JWT解码
pub fn decode(token: &str) -> Result<Claims> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&[ISSUER]);
    jsonwebtoken::decode::<Claims>(&token, &DECODE_KEY, &validation)
        .map(|token_data| token_data.claims)
}
