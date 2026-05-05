# 数据导出

数据导出是将采集到的数据保存为不同格式文件或同步到外部服务的功能。八爪鱼爬虫支持多种导出格式和目标，满足不同的数据使用需求。

## 支持的导出格式

### Excel (.xlsx)

最常用的数据导出格式，适合数据分析和报表制作。

**特点**:
- 支持多个工作表
- 保留数据格式和样式
- 支持公式和图表
- 兼容性好

**适用场景**:
- 数据分析和统计
- 制作报表
- 数据分享和展示
- 导入到其他系统

**导出示例**:
```
文件名: products_2024-01-15.xlsx
工作表: Sheet1
数据行数: 1000
包含字段: 标题, 价格, 图片, 链接, 评分
```

### CSV (.csv)

通用的文本格式，适合大数据量和程序处理。

**特点**:
- 文件体积小
- 加载速度快
- 兼容性极好
- 易于程序处理

**适用场景**:
- 大数据量导出
- 导入数据库
- 程序批处理
- 跨平台使用

**编码选项**:
- UTF-8 (推荐)
- GBK (兼容老版本Excel)
- UTF-8 with BOM

**导出示例**:
```csv
标题,价格,图片,链接,评分
"iPhone 15 Pro",9999,"https://...","/product/123",4.9
"Samsung S24",8999,"https://...","/product/456",4.8
```

### JSON (.json)

结构化数据格式，适合API对接和程序处理。

**特点**:
- 结构清晰
- 支持嵌套数据
- 易于解析
- 适合API传输

**适用场景**:
- API数据对接
- 前端应用使用
- 数据备份
- 程序间数据交换

**格式选项**:
- 标准JSON
- JSON Lines (每行一个JSON对象)
- 美化格式 (带缩进)
- 压缩格式 (无空格)

**导出示例**:
```json
[
  {
    "title": "iPhone 15 Pro",
    "price": 9999,
    "image": "https://...",
    "link": "/product/123",
    "rating": 4.9,
    "specs": {
      "brand": "Apple",
      "model": "A2848"
    }
  }
]
```

### Google Sheets

直接导出到Google表格，支持云端协作。

**特点**:
- 实时同步
- 多人协作
- 自动备份
- 在线访问

**适用场景**:
- 团队协作
- 实时数据监控
- 远程访问
- 数据分享

## 导出操作

### 方法1: 任务完成后导出

1. **采集完成**
   - 等待任务执行完成
   - 查看采集数据预览

2. **选择导出**
   - 点击"导出"按钮
   - 选择导出格式

3. **配置选项**
   - 选择要导出的字段
   - 设置文件名
   - 选择保存位置

4. **开始导出**
   - 点击"确认"按钮
   - 等待导出完成
   - 打开导出的文件

### 方法2: 从数据管理导出

1. **打开数据管理**
   - 点击插件图标
   - 进入"数据管理"模块

2. **选择数据**
   - 浏览采集的数据
   - 选择要导出的数据集
   - 可以筛选和排序

3. **导出数据**
   - 点击"导出"按钮
   - 选择格式和选项
   - 完成导出

### 方法3: 自动导出

配置任务自动导出采集结果。

1. **编辑任务设置**
   - 打开任务配置
   - 进入"导出设置"

2. **配置自动导出**
   ```json
   {
     "autoExport": true,
     "format": "xlsx",
     "filename": "products_{{date}}",
     "location": "~/Downloads/crawler-data/"
   }
   ```

3. **保存设置**
   - 任务完成后自动导出
   - 无需手动操作

## 导出配置

### 字段选择

选择要导出的字段和顺序。

**全部字段**:
```
✓ 标题
✓ 价格
✓ 图片
✓ 链接
✓ 评分
✓ 销量
✓ 店铺
```

**自定义字段**:
```
✓ 标题
✓ 价格
✓ 链接
✗ 图片 (不导出)
✗ 评分 (不导出)
```

**字段重命名**:
```
原字段名 → 导出字段名
title → 商品标题
price → 价格(元)
rating → 用户评分
```

### 数据过滤

导出前过滤数据。

**按条件过滤**:
```json
{
  "filters": [
    {
      "field": "price",
      "operator": ">",
      "value": 1000
    },
    {
      "field": "rating",
      "operator": ">=",
      "value": 4.5
    }
  ]
}
```

**按时间过滤**:
```json
{
  "dateRange": {
    "field": "collectTime",
    "start": "2024-01-01",
    "end": "2024-01-31"
  }
}
```

**去重**:
```json
{
  "deduplication": {
    "enabled": true,
    "fields": ["title", "link"],
    "keepFirst": true
  }
}
```

### 数据排序

设置导出数据的排序方式。

**单字段排序**:
```json
{
  "sort": {
    "field": "price",
    "order": "desc"
  }
}
```

**多字段排序**:
```json
{
  "sort": [
    {
      "field": "rating",
      "order": "desc"
    },
    {
      "field": "price",
      "order": "asc"
    }
  ]
}
```

### 文件命名

自定义导出文件名。

**使用变量**:
```
模板: products_{{date}}_{{time}}
结果: products_2024-01-15_143000.xlsx

模板: {{taskName}}_{{count}}条数据
结果: 商品采集_1000条数据.xlsx
```

**可用变量**:
- `{{date}}` - 日期 (YYYY-MM-DD)
- `{{time}}` - 时间 (HHMMSS)
- `{{datetime}}` - 日期时间
- `{{taskName}}` - 任务名称
- `{{count}}` - 数据条数
- `{{timestamp}}` - 时间戳

## 导出到Google Sheets

### 首次配置

1. **授权Google账户**
   - 点击"连接Google账户"
   - 登录Google账户
   - 授权访问权限

2. **选择工作表**
   - 选择现有工作表
   - 或创建新工作表

3. **配置映射**
   - 映射字段到表格列
   - 设置表头名称

### 导出模式

**追加模式**:
```
现有数据: 100行
新数据: 50行
结果: 150行 (新数据追加到末尾)
```

**覆盖模式**:
```
现有数据: 100行
新数据: 50行
结果: 50行 (清空后写入新数据)
```

**更新模式**:
```
根据关键字段更新现有数据
新数据更新匹配的行
不匹配的行追加到末尾
```

### 实时同步

配置实时同步到Google Sheets。

```json
{
  "googleSheets": {
    "enabled": true,
    "spreadsheetId": "1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgvE2upms",
    "sheetName": "产品数据",
    "mode": "append",
    "syncInterval": 60
  }
}
```

**同步选项**:
- 实时同步 (采集一条同步一条)
- 批量同步 (每N条同步一次)
- 定时同步 (每N分钟同步一次)

## 高级导出功能

### 分批导出

大数据量分批导出。

**按数量分批**:
```json
{
  "batchExport": {
    "enabled": true,
    "batchSize": 1000,
    "fileNamePattern": "products_batch_{{batchNumber}}.xlsx"
  }
}
```

**结果**:
```
products_batch_1.xlsx (1-1000)
products_batch_2.xlsx (1001-2000)
products_batch_3.xlsx (2001-3000)
```

**按日期分批**:
```json
{
  "batchExport": {
    "enabled": true,
    "groupBy": "date",
    "dateField": "collectTime"
  }
}
```

**结果**:
```
products_2024-01-15.xlsx
products_2024-01-16.xlsx
products_2024-01-17.xlsx
```

### 多格式同时导出

一次导出多种格式。

```json
{
  "multiFormat": {
    "enabled": true,
    "formats": ["xlsx", "csv", "json"]
  }
}
```

**结果**:
```
products_2024-01-15.xlsx
products_2024-01-15.csv
products_2024-01-15.json
```

### 数据转换

导出前转换数据格式。

**价格格式化**:
```javascript
// 输入: 9999
// 输出: ¥9,999.00
return '¥' + value.toLocaleString('zh-CN', {
  minimumFractionDigits: 2,
  maximumFractionDigits: 2
})
```

**日期格式化**:
```javascript
// 输入: 2024-01-15T10:30:00Z
// 输出: 2024年1月15日 10:30
const date = new Date(value)
return date.toLocaleString('zh-CN')
```

**URL转换**:
```javascript
// 相对URL转绝对URL
if (value.startsWith('/')) {
  return 'https://example.com' + value
}
return value
```

### 模板导出

使用自定义模板导出。

**Excel模板**:
1. 创建Excel模板文件
2. 定义数据区域和格式
3. 上传模板到插件
4. 导出时应用模板

**HTML模板**:
```html
<!DOCTYPE html>
<html>
<head>
  <title>采集数据报告</title>
  <style>
    table { border-collapse: collapse; }
    th, td { border: 1px solid #ddd; padding: 8px; }
  </style>
</head>
<body>
  <h1>{{taskName}} - 采集报告</h1>
  <p>采集时间: {{datetime}}</p>
  <p>数据条数: {{count}}</p>
  <table>
    <thead>
      <tr>
        {{#fields}}
        <th>{{name}}</th>
        {{/fields}}
      </tr>
    </thead>
    <tbody>
      {{#data}}
      <tr>
        {{#fields}}
        <td>{{value}}</td>
        {{/fields}}
      </tr>
      {{/data}}
    </tbody>
  </table>
</body>
</html>
```

## 导出到数据库

### 支持的数据库

- MySQL
- PostgreSQL
- MongoDB
- SQLite

### 配置数据库连接

**MySQL示例**:
```json
{
  "database": {
    "type": "mysql",
    "host": "localhost",
    "port": 3306,
    "database": "crawler_data",
    "username": "root",
    "password": "password",
    "table": "products"
  }
}
```

### 字段映射

```json
{
  "fieldMapping": {
    "title": "product_name",
    "price": "price",
    "image": "image_url",
    "link": "product_url",
    "rating": "rating",
    "collectTime": "created_at"
  }
}
```

### 插入模式

**插入新数据**:
```sql
INSERT INTO products (product_name, price, image_url)
VALUES ('iPhone 15', 9999, 'https://...')
```

**更新已有数据**:
```sql
INSERT INTO products (id, product_name, price)
VALUES (123, 'iPhone 15', 9999)
ON DUPLICATE KEY UPDATE
  product_name = VALUES(product_name),
  price = VALUES(price)
```

**忽略重复数据**:
```sql
INSERT IGNORE INTO products (product_name, price)
VALUES ('iPhone 15', 9999)
```

## 导出到云存储

### 支持的云存储服务

- 阿里云OSS
- 腾讯云COS
- AWS S3
- Google Cloud Storage

### 配置示例

**阿里云OSS**:
```json
{
  "cloudStorage": {
    "type": "aliyun-oss",
    "accessKeyId": "your-access-key-id",
    "accessKeySecret": "your-access-key-secret",
    "bucket": "crawler-data",
    "region": "oss-cn-hangzhou",
    "path": "exports/{{date}}/"
  }
}
```

**上传选项**:
- 自动上传
- 压缩后上传
- 加密上传
- 设置访问权限

## 数据安全

### 敏感数据处理

**脱敏处理**:
```javascript
// 手机号脱敏
// 13812345678 → 138****5678
return value.replace(/(\d{3})\d{4}(\d{4})/, '$1****$2')

// 邮箱脱敏
// user@example.com → u***@example.com
return value.replace(/(.{1}).*(@.*)/, '$1***$2')

// 身份证脱敏
// 110101199001011234 → 110101********1234
return value.replace(/(\d{6})\d{8}(\d{4})/, '$1********$2')
```

### 加密导出

**文件加密**:
```json
{
  "encryption": {
    "enabled": true,
    "algorithm": "AES-256",
    "password": "your-password"
  }
}
```

**压缩加密**:
```json
{
  "compression": {
    "enabled": true,
    "format": "zip",
    "password": "your-password"
  }
}
```

## 导出日志

### 查看导出历史

1. 进入"数据管理"
2. 点击"导出历史"
3. 查看所有导出记录

**记录信息**:
- 导出时间
- 文件名称
- 数据条数
- 文件大小
- 导出格式
- 导出状态

### 重新导出

对于历史数据，可以重新导出：

1. 在导出历史中找到记录
2. 点击"重新导出"
3. 选择新的格式或配置
4. 完成导出

## 故障排除

### 问题1: 导出文件打开乱码

**原因**: 编码问题

**解决方法**:
1. CSV文件使用UTF-8 with BOM编码
2. 或使用Excel格式代替CSV
3. 用记事本打开CSV，另存为UTF-8编码

### 问题2: Excel文件过大无法打开

**原因**: 数据量超过Excel限制

**解决方法**:
1. 使用分批导出
2. 或使用CSV格式
3. 或导出到数据库

### 问题3: Google Sheets同步失败

**原因**: 授权过期或网络问题

**解决方法**:
1. 重新授权Google账户
2. 检查网络连接
3. 查看错误日志
4. 减少同步频率

### 问题4: 导出数据不完整

**原因**: 过滤条件或字段选择问题

**解决方法**:
1. 检查过滤条件
2. 确认字段选择
3. 查看导出日志
4. 重新导出

## 最佳实践

### 1. 选择合适的格式

✅ **推荐**:
- 小数据量用Excel
- 大数据量用CSV
- API对接用JSON
- 团队协作用Google Sheets

### 2. 定期备份

✅ **推荐**:
- 设置自动导出
- 多格式备份
- 云端存储备份
- 定期清理旧数据

### 3. 数据验证

✅ **推荐**:
- 导出前检查数据
- 验证字段完整性
- 检查数据格式
- 测试导入流程

### 4. 性能优化

✅ **推荐**:
- 大数据分批导出
- 压缩后再传输
- 异步导出处理
- 合理设置缓存

## 下一步

- [定时任务](./scheduled-tasks) - 设置自动化采集和导出
- [数据处理](./data-processing) - 了解数据清洗和转换
- [三方集成](./integrations) - 集成更多外部服务
- [实战教程](./tutorials) - 完整的数据采集和导出案例
