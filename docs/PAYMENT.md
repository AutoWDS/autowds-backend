# 支付系统集成文档

本文档描述了从 dtiku-pay 项目复制到 autowds-backend 的支付功能。

## 功能特性

- ✅ 支付宝支付（扫码支付）
- ✅ 微信支付（Native支付）
- ✅ 支付回调处理
- ✅ 订单状态查询
- ✅ 支付统计
- ✅ 定时任务检查订单状态

## 项目结构

```
autowds-backend/
├── src/
│   ├── config/
│   │   └── pay.rs              # 支付配置
│   ├── model/
│   │   └── pay_order.rs        # 支付订单模型
│   ├── router/
│   │   └── pay.rs              # 支付路由
│   ├── task/
│   │   └── pay_check.rs        # 支付检查定时任务
│   └── utils/
│       ├── pay_plugin.rs       # 支付插件
│       └── pay_service.rs      # 支付服务
├── config/
│   └── pay.toml.example        # 支付配置示例
└── sql/
    └── pay_order.sql           # 数据库表结构
```

## 配置说明

### 1. 支付宝配置

在配置文件中添加支付宝相关配置：

```toml
[pay]
alipay_enable = true
alipay_api_url = "https://openapi.alipay.com/gateway.do"
alipay_app_id = "你的应用ID"
alipay_root_cert_sn = "支付宝根证书序列号"
alipay_public_key = "支付宝公钥"
alipay_app_cert_sn = "应用证书序列号"
alipay_app_private_key = "应用私钥"
alipay_app_public_key = "应用公钥"
alipay_callback_url = "https://yourdomain.com/api/pay/notify/alipay"
```

### 2. 微信支付配置

微信支付通过环境变量配置：

```bash
export WECHAT_PAY_MCH_ID="商户号"
export WECHAT_PAY_APPID="应用ID"
export WECHAT_PAY_API_V3_KEY="APIv3密钥"
export WECHAT_PAY_SERIAL_NO="证书序列号"
export WECHAT_PAY_PRIVATE_KEY="/path/to/private_key.pem"
export WECHAT_PAY_PUB_KEY_DIR="/data/wechat-cert/pubkey"
```

## API 接口

### 1. 创建订单

```http
POST /api/pay/create
Content-Type: application/json

{
  "level": "monthly",
  "pay_from": "alipay"
}
```

响应：
```json
{
  "order_id": 123456,
  "qrcode_url": "https://qr.alipay.com/bax08431..."
}
```

### 2. 查询订单状态

```http
GET /api/pay/query/{order_id}
```

### 3. 支付回调

- 支付宝回调：`POST /api/pay/notify/alipay`
- 微信回调：`POST /api/pay/notify/wechat`

### 4. 支付统计

```http
GET /api/pay/stats?start_date=2024-01-01&end_date=2024-01-31
```

## 数据库表结构

支付订单表 `pay_order`：

| 字段 | 类型 | 说明 |
|------|------|------|
| id | SERIAL | 订单ID（主键）|
| user_id | INTEGER | 用户ID |
| level | order_level | 订单级别（monthly/quarterly/half_year/annual）|
| pay_from | pay_from | 支付方式（alipay/wechat）|
| status | order_status | 订单状态（created/paid/closed）|
| created | TIMESTAMP | 创建时间 |
| modified | TIMESTAMP | 修改时间 |
| confirm | TIMESTAMP | 确认时间 |
| resp | JSONB | 支付平台响应数据 |

## 订单级别和价格

| 级别 | 价格（分） | 说明 |
|------|-----------|------|
| monthly | 2900 | 月度会员 29元 |
| quarterly | 7900 | 季度会员 79元 |
| half_year | 14900 | 半年会员 149元 |
| annual | 26900 | 年度会员 269元 |

## 定时任务

系统会每5分钟自动检查30分钟前创建但未确认的订单状态，确保订单状态的及时更新。

## 安全注意事项

1. **验签机制**：所有支付回调都会进行签名验证
2. **HTTPS**：生产环境必须使用HTTPS
3. **回调URL**：确保回调URL可以被支付平台访问
4. **证书管理**：定期更新支付平台证书

## 测试模式

开启测试模式后，所有订单金额将变为1分钱：

```toml
[pay]
test_pay_amount = true
```

## 部署注意事项

1. 确保数据库表已创建
2. 配置正确的支付参数
3. 设置正确的回调URL
4. 确保证书文件路径正确
5. 配置环境变量（微信支付）