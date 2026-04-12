use crate::utils::rand;
use anyhow::Context;
use summer_redis::{redis::AsyncCommands, Redis};
use summer_web::error::Result;

/// 验证码用途：不同用途使用独立 Redis 键，避免注册码与重置码混用。
#[derive(Clone, Copy, Debug)]
pub enum ValidateCodePurpose {
    Register,
    ResetPassword,
}

pub async fn get_validate_code(
    redis: &mut Redis,
    email: &str,
    purpose: ValidateCodePurpose,
) -> Result<Option<String>> {
    let key = validate_redis_key(email, purpose);
    Ok(redis
        .get(&key)
        .await
        .with_context(|| format!("get {} from redis failed", key))?)
}

pub async fn gen_validate_code(
    redis: &mut Redis,
    email: &str,
    purpose: ValidateCodePurpose,
) -> Result<String> {
    gen_validate_code_with_duration(redis, email, 5 * 60, purpose).await
}

pub async fn gen_validate_code_with_duration(
    redis: &mut Redis,
    email: &str,
    seconds: u64,
    purpose: ValidateCodePurpose,
) -> Result<String> {
    let key = validate_redis_key(email, purpose);
    let rand_code = rand::rand_alphanumeric(6);
    redis
        .set_ex::<_, _, ()>(&key, &rand_code, seconds)
        .await
        .with_context(|| format!("set {} to redis failed", key))?;
    Ok(rand_code)
}

fn validate_redis_key(email: &str, purpose: ValidateCodePurpose) -> String {
    let scope = match purpose {
        ValidateCodePurpose::Register => "register",
        ValidateCodePurpose::ResetPassword => "reset",
    };
    format!("email-validate:{scope}:{email}")
}
