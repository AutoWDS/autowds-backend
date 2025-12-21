use serde::{Deserialize, Serialize};
use spring_web::{
    axum::{
        async_trait,
        extract::{FromRequestParts, Request},
        http::request::Parts,
        response::{IntoResponse, Response},
    },
    error::{KnownWebError, Result},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub username: String,
    pub exp: usize,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // TODO: 实现JWT token解析逻辑
        // 这里暂时返回一个模拟的用户信息
        // 实际应该从Authorization header中解析JWT token
        
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok());
            
        if let Some(auth_header) = auth_header {
            if auth_header.starts_with("Bearer ") {
                // TODO: 解析JWT token
                // 暂时返回模拟数据
                return Ok(Claims {
                    user_id: 1,
                    username: "test_user".to_string(),
                    exp: 0,
                });
            }
        }
        
        // 如果没有认证信息，返回未授权错误
        Err(KnownWebError::unauthorized("需要登录").into_response())
    }
}