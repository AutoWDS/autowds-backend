use crate::model::account_user;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use derive_more::derive::Deref;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use schemars::json_schema;
use serde::{Deserialize, Serialize};
use spring_web::aide::generate::GenContext;
use spring_web::aide::openapi::{
    Operation, Parameter, ParameterData, ParameterSchemaOrContent, ReferenceOr, SchemaObject,
};
use spring_web::aide::OperationInput;
use spring_web::axum::http::header;
use spring_web::axum::http::request::Parts;
use spring_web::axum::RequestPartsExt;
use spring_web::error::{KnownWebError, Result, WebError};
use spring_web::extractor::FromRequestParts;

lazy_static! {
    static ref DECODE_KEY: DecodingKey =
        DecodingKey::from_rsa_pem(include_bytes!("./keys/public.key"))
            .expect("public key parse failed");
    static ref ENCODE_KEY: EncodingKey =
        EncodingKey::from_rsa_pem(include_bytes!("./keys/private.key"))
            .expect("private key parse failed");
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub uid: i64,
    pub email: String,
    pub is_admin: bool,
    exp: u64,
}

impl Claims {
    pub fn new(user: account_user::Model) -> Self {
        Self {
            uid: user.id,
            email: user.email.clone(),
            is_admin: user.id <= 1,
            exp: jsonwebtoken::get_current_timestamp() + 360 * 24 * 60 * 60 * 1000,
        }
    }
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = WebError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| KnownWebError::unauthorized("invalid token"))?;
        // Decode the user data
        let claims = decode(bearer.token())?;

        Ok(claims)
    }
}

/// # define the OpenAPI doc for Claims
impl OperationInput for Claims {
    fn operation_input(_ctx: &mut GenContext, operation: &mut Operation) {
        let aide_schema = SchemaObject {
            json_schema: json_schema!({
                "description": "JWT Claims",
                "type": ["string"]
            }),
            external_docs: None,
            example: None,
        };
        operation
            .parameters
            .push(ReferenceOr::Item(Parameter::Header {
                parameter_data: ParameterData {
                    name: "Authorization".into(),
                    description: Some("Bearer token for authentication".into()),
                    required: true,
                    format: ParameterSchemaOrContent::Schema(aide_schema),
                    example: Some("Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...".into()),
                    examples: Default::default(),
                    explode: Default::default(),
                    extensions: Default::default(),
                    deprecated: Default::default(),
                },
                style: Default::default(),
            }));
    }
}

pub struct OptionalClaims(Option<Claims>);

impl OptionalClaims {
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }
    pub fn get(self) -> Result<Claims> {
        Ok(self
            .0
            .ok_or_else(|| KnownWebError::unauthorized("token不存在"))?)
    }
}

impl<S> FromRequestParts<S> for OptionalClaims
where
    S: Send + Sync,
{
    type Rejection = WebError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        if !parts.headers.contains_key(header::AUTHORIZATION) {
            return Ok(Self(None));
        }
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| KnownWebError::unauthorized("invalid token"))?;
        // Decode the user data
        let claims = decode(bearer.token())?;

        Ok(Self(Some(claims)))
    }
}

/// # define the OpenAPI doc for Claims
impl OperationInput for OptionalClaims {
    fn operation_input(_ctx: &mut GenContext, operation: &mut Operation) {
        let aide_schema = SchemaObject {
            json_schema: json_schema!({
                "description": "JWT Claims",
                "type": ["string"]
            }),
            external_docs: None,
            example: None,
        };
        operation
            .parameters
            .push(ReferenceOr::Item(Parameter::Header {
                parameter_data: ParameterData {
                    name: "Authorization".into(),
                    description: Some("Bearer token for authentication".into()),
                    required: false,
                    format: ParameterSchemaOrContent::Schema(aide_schema),
                    example: Some("Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...".into()),
                    examples: Default::default(),
                    explode: Default::default(),
                    extensions: Default::default(),
                    deprecated: Default::default(),
                },
                style: Default::default(),
            }));
    }
}

/// # Admin Claims - 管理员权限验证
#[derive(Debug, Deref)]
pub struct AdminClaims(Claims);

impl<S> FromRequestParts<S> for AdminClaims
where
    S: Send + Sync,
{
    type Rejection = WebError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let claims = Claims::from_request_parts(parts, state).await?;
        
        if !claims.is_admin {
            return Err(KnownWebError::forbidden("需要管理员权限").into());
        }
        
        Ok(Self(claims))
    }
}

/// # define the OpenAPI doc for AdminClaims
/// ### 管理员接口不暴露openapi文档

/// # JWT encode
pub fn encode(claims: Claims) -> Result<String> {
    let header = Header::new(Algorithm::RS256);

    let token = jsonwebtoken::encode::<Claims>(&header, &claims, &ENCODE_KEY)
        .map_err(|_| KnownWebError::internal_server_error("Token created error"))?;

    Ok(token)
}

/// # JWT decode
pub fn decode(token: &str) -> Result<Claims> {
    let validation = Validation::new(Algorithm::RS256);
    let token_data =
        jsonwebtoken::decode::<Claims>(&token, &DECODE_KEY, &validation).map_err(|e| {
            tracing::error!("{:?}", e);
            KnownWebError::unauthorized("invalid token")
        })?;
    Ok(token_data.claims)
}
