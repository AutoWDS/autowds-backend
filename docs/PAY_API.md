# 支付 API 接口文档

## 接口概览

基于 dtiku-pay 项目的支付接口，支持支付宝和微信支付。

### 基础路径
```
/api/pay
```

## 1. 渲染支付页面

### GET /api/pay/render

渲染支付创建页面（HTML页面）

**请求头:**
```
Authorization: Bearer <jwt_token>
```

**响应:**
- HTML 页面，包含支付表单

---

## 2. 创建支付订单（表单提交）

### POST /api/pay/create

通过表单提交创建支付订单

**请求头:**
```
Authorization: Bearer <jwt_token>
Content-Type: application/x-www-form-urlencoded
```

**请求参数:**
```
level=monthly&pay_from=alipay
```

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| level | string | 是 | 会员级别：monthly/quarterly/half_year/annual |
| pay_from | string | 是 | 支付方式：alipay/wechat |

**响应:**
- HTML 页面，包含支付二维码和订单信息

---

## 3. 查询订单状态

### POST /api/pay/{order_id}/status

查询指定订单的支付状态

**请求头:**
```
Authorization: Bearer <jwt_token>
```

**路径参数:**
| 参数 | 类型 | 说明 |
|------|------|------|
| order_id | integer | 订单ID |

**响应:**
```json
"paid"
```

可能的状态值：
- `"created"` - 已创建，等待支付
- `"paid"` - 已支付
- `"closed"` - 已关闭

---

## 4. 支付回调接口

### POST /api/pay/notify/wechat

微信支付回调接口（由微信支付平台调用）

**请求头:**
```
Wechatpay-Serial: <证书序列号>
Wechatpay-Signature: <签名>
Wechatpay-Timestamp: <时间戳>
Wechatpay-Nonce: <随机字符串>
Content-Type: application/json
```

**请求体:**
微信支付回调数据（JSON格式）

**响应:**
```json
{"code": "SUCCESS"}
```

### POST /api/pay/notify/alipay

支付宝支付回调接口（由支付宝平台调用）

**请求头:**
```
Content-Type: application/x-www-form-urlencoded
```

**请求体:**
支付宝回调数据（表单格式）

**响应:**
```
success
```

---

## 5. 支付统计

### GET /api/pay/stats

获取支付统计数据

**查询参数:**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| start_date | string | 否 | 开始日期，格式：YYYY-MM-DD |
| end_date | string | 否 | 结束日期，格式：YYYY-MM-DD |

**响应:**
```json
[
  {
    "day": "2024-01-01T00:00:00",
    "paid_count": 10,
    "paid_amount": 29000,
    "pending_count": 5,
    "pending_amount": 14500
  }
]
```

---

## 会员级别和价格

| 级别 | 代码 | 价格 | 说明 |
|------|------|------|------|
| 月度会员 | monthly | ¥29 | 1个月有效期 |
| 季度会员 | quarterly | ¥79 | 3个月有效期 |
| 半年会员 | half_year | ¥149 | 6个月有效期 |
| 年度会员 | annual | ¥269 | 12个月有效期 |

## 支付流程

1. **用户访问支付页面**: `GET /api/pay/render`
2. **选择会员类型和支付方式**: 在页面表单中选择
3. **提交订单**: `POST /api/pay/create`
4. **扫码支付**: 用户使用支付宝或微信扫描二维码
5. **支付确认**: 支付平台调用回调接口确认支付
6. **状态查询**: 前端定时查询订单状态 `POST /api/pay/{order_id}/status`
7. **支付完成**: 用户会员状态更新

## 错误处理

所有接口都会返回适当的HTTP状态码：

- `200` - 成功
- `400` - 请求参数错误
- `401` - 未授权（需要登录）
- `404` - 资源不存在
- `500` - 服务器内部错误

错误响应格式：
```json
{
  "type": "https://httpstatuses.com/500",
  "title": "Internal Server Error",
  "status": 500,
  "detail": "具体错误信息",
  "instance": "/api/pay/create"
}
```

## 安全说明

1. **认证**: 所有用户相关接口都需要JWT token认证
2. **签名验证**: 支付回调接口会验证支付平台的签名
3. **HTTPS**: 生产环境必须使用HTTPS
4. **回调URL**: 确保回调URL可以被支付平台访问

## 测试

在测试环境中，可以开启测试模式：

```toml
[pay]
test_pay_amount = true
```

开启后，所有订单金额将变为1分钱，便于测试。