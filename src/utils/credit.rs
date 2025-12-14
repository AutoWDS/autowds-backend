use crate::model::{
    account_user, credit_log, prelude::*, sea_orm_active_enums::CreditOperation,
};
use anyhow::{Context, Result};
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, EntityTrait, Set, TransactionTrait,
};

/// 积分服务
pub struct CreditService;

impl CreditService {
    /// 增加积分
    pub async fn add_credits<C>(
        db: &C,
        user_id: i64,
        amount: i32,
        operation: CreditOperation,
        description: Option<String>,
        related_user_id: Option<i64>,
    ) -> Result<i32>
    where
        C: ConnectionTrait + TransactionTrait,
    {
        let txn = db.begin().await?;

        // 获取用户当前积分
        let user = AccountUser::find_by_id(user_id)
            .one(&txn)
            .await
            .with_context(|| format!("find user by id#{}", user_id))?
            .ok_or_else(|| anyhow::anyhow!("用户不存在"))?;

        let new_balance = user.credits + amount;

        // 更新用户积分
        let mut user_active: account_user::ActiveModel = user.into();
        user_active.credits = Set(new_balance);
        user_active.update(&txn).await?;

        // 记录积分日志
        let log = credit_log::ActiveModel {
            user_id: Set(user_id),
            operation: Set(operation),
            amount: Set(amount),
            balance: Set(new_balance),
            description: Set(description),
            related_user_id: Set(related_user_id),
            ..Default::default()
        };
        log.insert(&txn).await?;

        txn.commit().await?;
        Ok(new_balance)
    }

    /// 扣减积分
    pub async fn deduct_credits<C>(
        db: &C,
        user_id: i64,
        amount: i32,
        operation: CreditOperation,
        description: Option<String>,
    ) -> Result<i32>
    where
        C: ConnectionTrait + TransactionTrait,
    {
        let txn = db.begin().await?;

        // 获取用户当前积分
        let user = AccountUser::find_by_id(user_id)
            .one(&txn)
            .await
            .with_context(|| format!("find user by id#{}", user_id))?
            .ok_or_else(|| anyhow::anyhow!("用户不存在"))?;

        if user.credits < amount {
            return Err(anyhow::anyhow!("积分不足"));
        }

        let new_balance = user.credits - amount;

        // 更新用户积分
        let mut user_active: account_user::ActiveModel = user.into();
        user_active.credits = Set(new_balance);
        user_active.update(&txn).await?;

        // 记录积分日志
        let log = credit_log::ActiveModel {
            user_id: Set(user_id),
            operation: Set(operation),
            amount: Set(-(amount as i32)),
            balance: Set(new_balance),
            description: Set(description),
            related_user_id: Set(None),
            ..Default::default()
        };
        log.insert(&txn).await?;

        txn.commit().await?;
        Ok(new_balance)
    }

    /// Base62字符集 (0-9, A-Z, a-z)
    const BASE62_CHARS: &'static [u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    
    /// 将数字转换为Base62编码
    fn encode_base62(mut num: u64) -> String {
        if num == 0 {
            return "0".to_string();
        }
        
        let mut result = Vec::new();
        while num > 0 {
            result.push(Self::BASE62_CHARS[(num % 62) as usize]);
            num /= 62;
        }
        result.reverse();
        String::from_utf8(result).unwrap()
    }
    
    /// 生成邀请码（使用Base62编码，更短更美观）
    pub fn generate_invite_code(user_id: i64) -> String {
        use rand::Rng;
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let mut rng = rand::rng();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // 组合：用户ID + 时间戳 + 随机数
        let user_part = Self::encode_base62(user_id as u64);
        let time_part = Self::encode_base62(timestamp % 238328); // 限制在4位Base62内 (62^4 = 14,776,336)
        let random_part = Self::encode_base62(rng.random_range(0..238328) as u64); // 4位Base62随机数
        
        // 格式: INV + 用户ID(Base62) + 时间戳(4位Base62) + 随机数(4位Base62)
        format!("INV{}{}{}", user_part, time_part, random_part)
    }

    /// 处理邀请注册
    pub async fn handle_invite_register<C>(
        db: &C,
        inviter_id: i64,
        new_user_id: i64,
    ) -> Result<()>
    where
        C: ConnectionTrait + TransactionTrait,
    {
        // 给邀请人增加积分
        Self::add_credits(
            db,
            inviter_id,
            100,
            CreditOperation::Invite,
            Some(format!("邀请用户#{} 注册", new_user_id)),
            Some(new_user_id),
        )
        .await?;

        Ok(())
    }
}