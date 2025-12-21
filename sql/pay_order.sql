-- 支付订单表
CREATE TABLE IF NOT EXISTS pay_order (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    level VARCHAR(20) NOT NULL CHECK (level IN ('monthly', 'quarterly', 'half_year', 'annual')),
    pay_from VARCHAR(20) NOT NULL CHECK (pay_from IN ('alipay', 'wechat')),
    status VARCHAR(20) NOT NULL DEFAULT 'created' CHECK (status IN ('created', 'paid', 'closed')),
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    confirm TIMESTAMP NULL,
    resp JSONB NULL,
    
    -- 索引
    INDEX idx_pay_order_user_id (user_id),
    INDEX idx_pay_order_status (status),
    INDEX idx_pay_order_created (created),
    INDEX idx_pay_order_confirm (confirm)
);

-- 创建订单级别枚举类型
CREATE TYPE order_level AS ENUM ('monthly', 'quarterly', 'half_year', 'annual');

-- 创建支付来源枚举类型  
CREATE TYPE pay_from AS ENUM ('alipay', 'wechat');

-- 创建订单状态枚举类型
CREATE TYPE order_status AS ENUM ('created', 'paid', 'closed');

-- 如果表已存在，修改表结构使用枚举类型
-- ALTER TABLE pay_order ALTER COLUMN level TYPE order_level USING level::order_level;
-- ALTER TABLE pay_order ALTER COLUMN pay_from TYPE pay_from USING pay_from::pay_from;
-- ALTER TABLE pay_order ALTER COLUMN status TYPE order_status USING status::order_status;