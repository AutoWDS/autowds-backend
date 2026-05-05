# 实战教程

通过实际案例学习如何使用八爪鱼爬虫进行数据采集。本教程涵盖从简单到复杂的各种采集场景。

## 教程1: 采集电商商品列表

### 目标

采集某电商网站的手机商品列表，包括商品名称、价格、评分、销量等信息。

### 步骤

#### 1. 分析目标网站

**URL**: `https://example.com/search?q=手机`

**页面结构**:
```html
<div class="product-list">
  <div class="product-item">
    <img class="product-img" src="...">
    <h3 class="product-title">iPhone 15 Pro</h3>
    <span class="product-price">¥9999</span>
    <span class="product-rating">4.9分</span>
    <span class="product-sales">已售10000+</span>
    <a class="product-link" href="/product/123">查看详情</a>
  </div>
  <!-- 更多商品 -->
</div>
<div class="pagination">
  <a class="next-page">下一页</a>
</div>
```

#### 2. 创建采集规则

**使用智能采集**:
1. 打开目标页面
2. 右键选择"智能采集"
3. AI自动识别字段
4. 确认并调整字段

**或使用可视化规则**:

**起始节点配置**:
```json
{
  "url": "https://example.com/search?q=手机",
  "viewport": {
    "width": 1920,
    "height": 1080
  }
}
```

**列表节点配置**:
```json
{
  "listSelector": ".product-list .product-item",
  "fields": [
    {
      "name": "title",
      "selector": ".product-title",
      "attr": "innerText"
    },
    {
      "name": "price",
      "selector": ".product-price",
      "attr": "innerText",
      "extractor": {
        "type": "regex",
        "code": "\\d+"
      }
    },
    {
      "name": "rating",
      "selector": ".product-rating",
      "attr": "innerText",
      "extractor": {
        "type": "regex",
        "code": "\\d+\\.\\d+"
      }
    },
    {
      "name": "sales",
      "selector": ".product-sales",
      "attr": "innerText",
      "extractor": {
        "type": "regex",
        "code": "\\d+"
      }
    },
    {
      "name": "image",
      "selector": ".product-img",
      "attr": "src"
    },
    {
      "name": "link",
      "selector": ".product-link",
      "attr": "href"
    }
  ],
  "pagination": {
    "type": "click_next",
    "config": {
      "selector": ".pagination .next-page",
      "maxPages": 10,
      "interval": 3000
    }
  }
}
```

#### 3. 测试和运行

1. 点击"测试"按钮
2. 查看采集结果
3. 确认数据正确
4. 点击"运行"开始采集

#### 4. 导出数据

采集完成后导出为Excel:
```
文件名: 手机商品列表_2024-01-15.xlsx
数据量: 500条
字段: 标题, 价格, 评分, 销量, 图片, 链接
```

### 预期结果

```csv
标题,价格,评分,销量,图片,链接
iPhone 15 Pro,9999,4.9,10000,https://...,/product/123
Samsung S24,8999,4.8,8000,https://...,/product/456
小米14,3999,4.7,15000,https://...,/product/789
```

---

## 教程2: 深度采集商品详情

### 目标

在教程1的基础上，进入每个商品的详情页，采集更详细的信息。

### 步骤

#### 1. 扩展规则

在列表节点后添加页面节点和详情节点。

**流程**:
```
起始节点 → 列表节点 → 页面节点 → 详情节点
```

#### 2. 配置页面节点

```json
{
  "type": "click_element",
  "value": ".product-link"
}
```

#### 3. 配置详情节点

```json
{
  "fields": [
    {
      "name": "description",
      "selector": ".product-description",
      "attr": "innerText"
    },
    {
      "name": "brand",
      "selector": ".product-brand",
      "attr": "innerText"
    },
    {
      "name": "model",
      "selector": ".product-model",
      "attr": "innerText"
    },
    {
      "name": "specifications",
      "selector": ".product-specs",
      "attr": "innerText"
    },
    {
      "name": "images",
      "selector": ".product-gallery img",
      "attr": "src",
      "multiple": true
    }
  ]
}
```

#### 4. 数据合并

列表页和详情页的数据会自动合并：

```json
{
  // 列表页数据
  "title": "iPhone 15 Pro",
  "price": "9999",
  "rating": "4.9",
  
  // 详情页数据
  "description": "全新A17 Pro芯片...",
  "brand": "Apple",
  "model": "A2848",
  "specifications": "6.7英寸屏幕...",
  "images": ["img1.jpg", "img2.jpg", "img3.jpg"]
}
```

### 注意事项

- 设置合理的访问间隔（建议3-5秒）
- 添加错误处理机制
- 监控采集进度
- 定期保存数据

---

## 教程3: 采集新闻文章

### 目标

采集新闻网站的最新文章，包括标题、作者、发布时间、正文内容。

### 步骤

#### 1. 分析页面结构

**列表页**:
```html
<div class="news-list">
  <article class="news-item">
    <h2 class="news-title">
      <a href="/article/123">新闻标题</a>
    </h2>
    <span class="news-author">作者名</span>
    <time class="news-time">2024-01-15 10:30</time>
    <p class="news-summary">文章摘要...</p>
  </article>
</div>
```

**详情页**:
```html
<article class="article">
  <h1 class="article-title">新闻标题</h1>
  <div class="article-meta">
    <span class="author">作者名</span>
    <time class="publish-time">2024-01-15 10:30</time>
  </div>
  <div class="article-content">
    <p>正文内容...</p>
  </div>
</article>
```

#### 2. 配置采集规则

**列表节点**:
```json
{
  "listSelector": ".news-list .news-item",
  "fields": [
    {
      "name": "title",
      "selector": ".news-title a",
      "attr": "innerText"
    },
    {
      "name": "author",
      "selector": ".news-author",
      "attr": "innerText"
    },
    {
      "name": "publishTime",
      "selector": ".news-time",
      "attr": "innerText"
    },
    {
      "name": "summary",
      "selector": ".news-summary",
      "attr": "innerText"
    },
    {
      "name": "articleUrl",
      "selector": ".news-title a",
      "attr": "href"
    }
  ]
}
```

**详情节点**:
```json
{
  "fields": [
    {
      "name": "content",
      "selector": ".article-content",
      "attr": "innerText",
      "extractor": {
        "type": "js",
        "code": "return value.trim().replace(/\\s+/g, ' ')"
      }
    }
  ]
}
```

#### 3. 数据清洗

**清洗时间格式**:
```javascript
// 输入: "2小时前"
// 输出: "2024-01-15 14:30:00"
const now = new Date()
const match = value.match(/(\d+)小时前/)
if (match) {
  now.setHours(now.getHours() - parseInt(match[1]))
  return now.toISOString()
}
return value
```

**清洗正文内容**:
```javascript
// 去除多余空白和HTML标签
return value
  .replace(/<[^>]+>/g, '')
  .replace(/\s+/g, ' ')
  .trim()
```

---

## 教程4: 采集招聘信息

### 目标

采集招聘网站的职位信息，包括职位名称、公司、薪资、要求等。

### 步骤

#### 1. 配置搜索

**起始节点添加搜索步骤**:
```json
{
  "url": "https://jobs.example.com",
  "steps": [
    {
      "type": "change",
      "selectors": [["input.search-keyword"]],
      "value": "前端工程师"
    },
    {
      "type": "change",
      "selectors": [["input.search-city"]],
      "value": "北京"
    },
    {
      "type": "click",
      "selectors": [["button.search-btn"]]
    },
    {
      "type": "wait",
      "timeout": 2000
    }
  ]
}
```

#### 2. 采集列表

```json
{
  "listSelector": ".job-list .job-item",
  "fields": [
    {
      "name": "jobTitle",
      "selector": ".job-title",
      "attr": "innerText"
    },
    {
      "name": "company",
      "selector": ".company-name",
      "attr": "innerText"
    },
    {
      "name": "salary",
      "selector": ".job-salary",
      "attr": "innerText"
    },
    {
      "name": "location",
      "selector": ".job-location",
      "attr": "innerText"
    },
    {
      "name": "experience",
      "selector": ".job-experience",
      "attr": "innerText"
    },
    {
      "name": "education",
      "selector": ".job-education",
      "attr": "innerText"
    }
  ]
}
```

#### 3. 数据处理

**薪资范围提取**:
```javascript
// 输入: "15K-25K·14薪"
// 输出: { min: 15000, max: 25000, months: 14 }
const match = value.match(/(\d+)K-(\d+)K·(\d+)薪/)
if (match) {
  return {
    min: parseInt(match[1]) * 1000,
    max: parseInt(match[2]) * 1000,
    months: parseInt(match[3])
  }
}
```

---

## 教程5: 批量关键词采集

### 目标

批量搜索多个关键词并采集结果。

### 步骤

#### 1. 准备关键词列表

创建CSV文件 `keywords.csv`:
```csv
keyword
iPhone 15
Samsung S24
小米14
华为Mate60
OPPO Find X7
```

#### 2. 配置批量任务

```json
{
  "batchInput": {
    "type": "csv",
    "file": "keywords.csv",
    "field": "keyword"
  },
  "urlTemplate": "https://example.com/search?q={{keyword}}"
}
```

#### 3. 执行批量采集

插件会自动：
1. 读取关键词列表
2. 依次搜索每个关键词
3. 采集每个关键词的结果
4. 合并所有数据

#### 4. 结果分组

按关键词分组导出：
```
iPhone_15_results.xlsx
Samsung_S24_results.xlsx
小米14_results.xlsx
...
```

---

## 教程6: 定时监控价格变化

### 目标

每天定时采集商品价格，监控价格变化。

### 步骤

#### 1. 创建采集规则

配置商品列表采集规则（参考教程1）。

#### 2. 配置定时任务

```json
{
  "schedule": {
    "enabled": true,
    "cron": "0 9 * * *",
    "timezone": "Asia/Shanghai"
  }
}
```

#### 3. 配置增量采集

```json
{
  "incremental": {
    "enabled": true,
    "compareField": "productId",
    "trackChanges": ["price", "stock"]
  }
}
```

#### 4. 配置价格变化通知

```json
{
  "notification": {
    "enabled": true,
    "conditions": [
      {
        "field": "price",
        "change": "decreased",
        "threshold": 100
      }
    ],
    "channels": ["email", "webhook"]
  }
}
```

#### 5. 数据分析

导出历史数据，分析价格趋势：
```csv
日期,商品ID,商品名称,价格,变化
2024-01-15,123,iPhone 15,9999,0
2024-01-16,123,iPhone 15,9899,-100
2024-01-17,123,iPhone 15,9899,0
```

---

## 教程7: 采集社交媒体内容

### 目标

采集社交媒体平台的帖子内容。

### 步骤

#### 1. 登录处理

```json
{
  "url": "https://social.example.com/login",
  "steps": [
    {
      "type": "change",
      "selectors": [["input[name='username']"]],
      "value": "your_username"
    },
    {
      "type": "change",
      "selectors": [["input[name='password']"]],
      "value": "your_password"
    },
    {
      "type": "click",
      "selectors": [["button[type='submit']"]]
    },
    {
      "type": "wait",
      "timeout": 3000
    },
    {
      "type": "navigate",
      "url": "https://social.example.com/feed"
    }
  ]
}
```

#### 2. 无限滚动采集

```json
{
  "listSelector": ".feed-item",
  "fields": [
    {
      "name": "author",
      "selector": ".author-name",
      "attr": "innerText"
    },
    {
      "name": "content",
      "selector": ".post-content",
      "attr": "innerText"
    },
    {
      "name": "likes",
      "selector": ".like-count",
      "attr": "innerText"
    },
    {
      "name": "comments",
      "selector": ".comment-count",
      "attr": "innerText"
    },
    {
      "name": "postTime",
      "selector": ".post-time",
      "attr": "innerText"
    }
  ],
  "pagination": {
    "type": "scroll",
    "config": {
      "maxScrolls": 50,
      "interval": 2000,
      "waitForNewContent": 3000
    }
  }
}
```

---

## 教程8: 采集表格数据

### 目标

采集网页中的表格数据。

### 步骤

#### 1. 分析表格结构

```html
<table class="data-table">
  <thead>
    <tr>
      <th>姓名</th>
      <th>年龄</th>
      <th>城市</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>张三</td>
      <td>25</td>
      <td>北京</td>
    </tr>
  </tbody>
</table>
```

#### 2. 配置采集规则

```json
{
  "listSelector": ".data-table tbody tr",
  "fields": [
    {
      "name": "name",
      "selector": "td:nth-child(1)",
      "attr": "innerText"
    },
    {
      "name": "age",
      "selector": "td:nth-child(2)",
      "attr": "innerText"
    },
    {
      "name": "city",
      "selector": "td:nth-child(3)",
      "attr": "innerText"
    }
  ]
}
```

#### 3. 动态表头处理

如果表头是动态的：

```javascript
// 先采集表头
const headers = Array.from(
  document.querySelectorAll('.data-table thead th')
).map(th => th.textContent.trim())

// 根据表头动态生成字段配置
const fields = headers.map((header, index) => ({
  name: header,
  selector: `td:nth-child(${index + 1})`,
  attr: 'innerText'
}))
```

---

## 常见问题和解决方案

### 问题1: 采集速度慢

**解决方案**:
1. 减少不必要的字段
2. 优化选择器
3. 调整并发设置
4. 使用增量采集

### 问题2: 数据不完整

**解决方案**:
1. 增加等待时间
2. 检查选择器
3. 处理动态加载
4. 添加错误重试

### 问题3: 被网站封禁

**解决方案**:
1. 降低采集频率
2. 添加随机延迟
3. 使用代理IP
4. 模拟人工操作

### 问题4: 数据格式不统一

**解决方案**:
1. 使用数据提取器
2. 配置数据转换
3. 统一数据格式
4. 数据验证

## 最佳实践总结

### 1. 采集前准备

✅ **推荐**:
- 分析目标网站结构
- 确定采集字段
- 测试选择器
- 制定采集计划

### 2. 规则配置

✅ **推荐**:
- 使用稳定的选择器
- 设置合理的等待时间
- 配置错误处理
- 添加数据验证

### 3. 执行采集

✅ **推荐**:
- 先小范围测试
- 监控采集过程
- 及时处理错误
- 定期保存数据

### 4. 数据处理

✅ **推荐**:
- 清洗数据
- 验证完整性
- 统一格式
- 及时导出

## 下一步

- [智能采集](./intelligent-collection) - 使用AI快速创建规则
- [可视化规则开发](./visual-rule-development) - 创建复杂规则
- [定时任务](./scheduled-tasks) - 自动化采集
- [数据导出](./data-export) - 导出和处理数据
