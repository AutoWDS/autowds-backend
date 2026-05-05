# 翻页配置

翻页是数据采集中的重要功能，用于自动浏览多页内容以采集更多数据。八爪鱼爬虫支持多种翻页方式，适应不同网站的分页机制。

## 翻页类型

### 1. 点击下一页 (click_next)

最常见的翻页方式，通过点击"下一页"按钮或页码链接实现翻页。

#### 适用场景

- 传统的数字分页导航
- "上一页/下一页"按钮
- 页码链接列表
- 文本链接翻页

#### 配置方式

**基本配置**:
```json
{
  "type": "click_next",
  "config": {
    "selector": ".pagination .next"
  }
}
```

**完整配置**:
```json
{
  "type": "click_next",
  "config": {
    "selector": ".pagination .next",
    "maxPages": 10,
    "interval": 2000,
    "waitAfterClick": 3000,
    "stopWhenDisabled": true
  }
}
```

#### 配置参数

| 参数 | 类型 | 说明 | 默认值 |
|------|------|------|--------|
| selector | string | 下一页按钮的选择器 | 必填 |
| maxPages | number | 最大翻页数 | 无限制 |
| interval | number | 翻页间隔(毫秒) | 2000 |
| waitAfterClick | number | 点击后等待时间(毫秒) | 3000 |
| stopWhenDisabled | boolean | 按钮禁用时停止 | true |

#### 常见选择器示例

**数字分页**:
```css
/* 下一页按钮 */
.pagination .next
.page-next
a[rel="next"]
button.next-page

/* 页码链接 */
.pagination a:last-child
.page-item:last-child a
```

**文本链接**:
```css
/* 包含"下一页"文本 */
a:contains("下一页")
a:contains("Next")
a:contains("»")

/* 使用XPath */
//a[text()='下一页']
//a[contains(text(), 'Next')]
```

#### 停止条件

翻页会在以下情况停止：

1. **达到最大页数**: 翻页次数达到maxPages设置
2. **按钮不存在**: 找不到下一页按钮
3. **按钮禁用**: 按钮被禁用(disabled)
4. **无新数据**: 当前页没有采集到新数据
5. **手动停止**: 用户手动停止任务

### 2. 无限滚动 (scroll)

通过滚动页面触发内容加载，常见于社交媒体和信息流网站。

#### 适用场景

- 社交媒体信息流 (微博、Twitter)
- 瀑布流布局 (Pinterest、花瓣)
- 新闻资讯网站
- 视频网站列表

#### 配置方式

**基本配置**:
```json
{
  "type": "scroll",
  "config": {
    "selector": "body"
  }
}
```

**完整配置**:
```json
{
  "type": "scroll",
  "config": {
    "selector": "body",
    "maxScrolls": 20,
    "interval": 1000,
    "scrollDistance": 1000,
    "scrollToBottom": true,
    "waitForNewContent": 3000,
    "stopWhenNoNewContent": true
  }
}
```

#### 配置参数

| 参数 | 类型 | 说明 | 默认值 |
|------|------|------|--------|
| selector | string | 滚动容器选择器 | body |
| maxScrolls | number | 最大滚动次数 | 无限制 |
| interval | number | 滚动间隔(毫秒) | 1000 |
| scrollDistance | number | 每次滚动距离(像素) | 1000 |
| scrollToBottom | boolean | 是否滚动到底部 | true |
| waitForNewContent | number | 等待新内容加载(毫秒) | 3000 |
| stopWhenNoNewContent | boolean | 无新内容时停止 | true |

#### 滚动策略

**滚动到底部**:
```javascript
// 每次滚动到页面底部
window.scrollTo(0, document.body.scrollHeight)
```

**固定距离滚动**:
```javascript
// 每次向下滚动1000像素
window.scrollBy(0, 1000)
```

**平滑滚动**:
```javascript
// 平滑滚动效果
window.scrollTo({
  top: document.body.scrollHeight,
  behavior: 'smooth'
})
```

#### 检测新内容

插件会自动检测页面是否加载了新内容：

1. 记录滚动前的元素数量
2. 执行滚动操作
3. 等待新内容加载
4. 对比元素数量
5. 如果没有新元素，停止滚动

### 3. 加载更多 (load_more)

通过点击"加载更多"或"查看更多"按钮加载新内容。

#### 适用场景

- 评论列表
- 商品推荐
- 搜索结果
- 文章列表

#### 配置方式

**基本配置**:
```json
{
  "type": "load_more",
  "config": {
    "selector": ".load-more-btn"
  }
}
```

**完整配置**:
```json
{
  "type": "load_more",
  "config": {
    "selector": ".load-more-btn",
    "maxClicks": 10,
    "interval": 1500,
    "waitAfterClick": 2000,
    "stopWhenHidden": true,
    "stopWhenDisabled": true
  }
}
```

#### 配置参数

| 参数 | 类型 | 说明 | 默认值 |
|------|------|------|--------|
| selector | string | 加载更多按钮选择器 | 必填 |
| maxClicks | number | 最大点击次数 | 无限制 |
| interval | number | 点击间隔(毫秒) | 1500 |
| waitAfterClick | number | 点击后等待时间(毫秒) | 2000 |
| stopWhenHidden | boolean | 按钮隐藏时停止 | true |
| stopWhenDisabled | boolean | 按钮禁用时停止 | true |

#### 常见选择器

```css
/* 按钮 */
button.load-more
.load-more-btn
#loadMore

/* 链接 */
a.more-link
.view-more

/* 包含特定文本 */
button:contains("加载更多")
a:contains("查看更多")
```

### 4. URL参数翻页

通过修改URL中的页码参数实现翻页。

#### 适用场景

- URL中包含page参数
- RESTful风格的分页
- API接口分页

#### 配置方式

**基本配置**:
```json
{
  "type": "url_param",
  "config": {
    "paramName": "page",
    "startPage": 1,
    "maxPages": 10
  }
}
```

**完整配置**:
```json
{
  "type": "url_param",
  "config": {
    "paramName": "page",
    "startPage": 1,
    "maxPages": 10,
    "increment": 1,
    "urlTemplate": "https://example.com/products?page={{page}}",
    "interval": 2000
  }
}
```

#### URL模式示例

**查询参数**:
```
https://example.com/products?page=1
https://example.com/products?page=2
https://example.com/products?page=3
```

**路径参数**:
```
https://example.com/products/page/1
https://example.com/products/page/2
https://example.com/products/page/3
```

**RESTful风格**:
```
https://api.example.com/v1/products?offset=0&limit=20
https://api.example.com/v1/products?offset=20&limit=20
https://api.example.com/v1/products?offset=40&limit=20
```

## 高级翻页配置

### 组合翻页

某些网站可能需要组合多种翻页方式。

**示例：先滚动再点击**:
```json
{
  "pagination": [
    {
      "type": "scroll",
      "config": {
        "maxScrolls": 5
      }
    },
    {
      "type": "click_next",
      "config": {
        "selector": ".next-page"
      }
    }
  ]
}
```

### 条件翻页

根据条件决定是否继续翻页。

```json
{
  "type": "click_next",
  "config": {
    "selector": ".next",
    "conditions": [
      {
        "type": "dataCount",
        "operator": "<",
        "value": 1000
      },
      {
        "type": "pageNumber",
        "operator": "<=",
        "value": 10
      }
    ]
  }
}
```

### 智能等待

根据页面加载情况智能调整等待时间。

```json
{
  "type": "click_next",
  "config": {
    "selector": ".next",
    "smartWait": {
      "enabled": true,
      "waitForSelector": ".product-item",
      "minWait": 1000,
      "maxWait": 10000,
      "timeout": 30000
    }
  }
}
```

### 错误重试

翻页失败时自动重试。

```json
{
  "type": "click_next",
  "config": {
    "selector": ".next",
    "retry": {
      "enabled": true,
      "maxAttempts": 3,
      "interval": 5000,
      "backoff": "exponential"
    }
  }
}
```

## 翻页性能优化

### 1. 设置合理的间隔

```json
{
  "interval": 2000,  // 2秒间隔
  "randomDelay": {
    "enabled": true,
    "min": 1000,
    "max": 3000
  }
}
```

**建议**:
- 普通网站: 2-3秒
- 大型网站: 3-5秒
- 严格限制的网站: 5-10秒

### 2. 并发控制

```json
{
  "concurrency": {
    "enabled": false,  // 翻页不建议并发
    "maxConcurrent": 1
  }
}
```

### 3. 缓存策略

```json
{
  "cache": {
    "enabled": true,
    "skipVisitedPages": true,
    "cacheExpiry": 86400
  }
}
```

### 4. 增量翻页

只翻页到有新数据的页面。

```json
{
  "incremental": {
    "enabled": true,
    "compareField": "id",
    "stopWhenDuplicate": true
  }
}
```

## 常见问题

### 问题1: 翻页按钮找不到

**原因**:
- 选择器不正确
- 按钮动态加载
- 按钮在iframe中

**解决方法**:
```javascript
// 1. 检查选择器
document.querySelector('.next-page')

// 2. 等待按钮出现
await page.waitForSelector('.next-page')

// 3. 检查iframe
const frame = page.frames().find(f => f.name() === 'content')
await frame.waitForSelector('.next-page')
```

### 问题2: 点击后没有翻页

**原因**:
- 点击事件被拦截
- 需要等待时间不够
- 页面使用JavaScript跳转

**解决方法**:
```json
{
  "config": {
    "selector": ".next",
    "clickMethod": "javascript",  // 使用JS点击
    "waitAfterClick": 5000,       // 增加等待时间
    "waitForNavigation": true     // 等待页面导航
  }
}
```

### 问题3: 无限滚动不加载新内容

**原因**:
- 滚动速度太快
- 需要滚动到特定位置
- 网站检测到自动化

**解决方法**:
```json
{
  "config": {
    "scrollDistance": 500,        // 减小滚动距离
    "interval": 2000,             // 增加间隔
    "waitForNewContent": 5000,    // 增加等待时间
    "scrollBehavior": "smooth"    // 使用平滑滚动
  }
}
```

### 问题4: 翻页数据重复

**原因**:
- 没有正确识别新数据
- 页面内容重复
- 去重配置不正确

**解决方法**:
```json
{
  "deduplication": {
    "enabled": true,
    "fields": ["id", "url"],
    "scope": "global"
  }
}
```

## 翻页调试

### 调试模式

启用调试模式查看翻页过程。

```json
{
  "debug": {
    "enabled": true,
    "logLevel": "verbose",
    "screenshot": true,
    "pauseBetweenPages": true
  }
}
```

### 日志输出

```
[翻页] 开始翻页，当前第1页
[翻页] 找到下一页按钮: .pagination .next
[翻页] 点击下一页按钮
[翻页] 等待页面加载...
[翻页] 页面加载完成，当前第2页
[翻页] 采集到50条新数据
[翻页] 继续翻页...
```

### 性能监控

```json
{
  "monitoring": {
    "enabled": true,
    "metrics": [
      "pageLoadTime",
      "clickResponseTime",
      "dataLoadTime"
    ]
  }
}
```

## 最佳实践

### 1. 选择合适的翻页类型

✅ **推荐**:
- 根据网站实际情况选择
- 优先使用简单的方式
- 测试确认翻页有效

❌ **避免**:
- 盲目使用复杂配置
- 不测试就上线
- 忽略网站特性

### 2. 设置合理的限制

✅ **推荐**:
- 设置最大页数
- 设置最大数据量
- 设置超时时间

❌ **避免**:
- 无限制翻页
- 不设置停止条件
- 忽略性能影响

### 3. 控制翻页速度

✅ **推荐**:
- 设置合理间隔
- 添加随机延迟
- 模拟人工操作

❌ **避免**:
- 翻页过快
- 固定间隔
- 明显的机器行为

### 4. 处理异常情况

✅ **推荐**:
- 配置错误重试
- 记录翻页日志
- 监控翻页状态

❌ **避免**:
- 不处理错误
- 不记录日志
- 忽略异常

## 实战案例

### 案例1: 电商商品列表

```json
{
  "type": "click_next",
  "config": {
    "selector": ".pagination .next",
    "maxPages": 20,
    "interval": 3000,
    "waitAfterClick": 2000,
    "stopConditions": [
      {
        "type": "noNewData",
        "consecutivePages": 2
      }
    ]
  }
}
```

### 案例2: 社交媒体信息流

```json
{
  "type": "scroll",
  "config": {
    "selector": ".feed-container",
    "maxScrolls": 50,
    "interval": 2000,
    "scrollToBottom": true,
    "waitForNewContent": 3000,
    "stopWhenNoNewContent": true,
    "minNewItems": 5
  }
}
```

### 案例3: 新闻网站

```json
{
  "type": "load_more",
  "config": {
    "selector": "button.load-more",
    "maxClicks": 15,
    "interval": 2000,
    "waitAfterClick": 3000,
    "stopWhenHidden": true
  }
}
```

## 下一步

- [元素选择器](./selectors) - 了解如何编写选择器
- [数据采集](./data-collection) - 配置数据提取规则
- [批量采集](./batch-collection) - 批量采集多个目标
- [实战教程](./tutorials) - 完整的采集案例
