# 基本概念

理解八爪鱼爬虫的核心概念，将帮助您更好地使用插件进行数据采集。

## 规则（Rule）

### 什么是规则？

规则是定义如何从网页中提取数据的配置文件。它包含了采集的完整流程，从打开网页、定位元素、提取数据到保存结果的所有步骤。

### 规则的组成

一个完整的规则包含以下部分：

**1. 起始配置**
- 目标网站URL
- 浏览器窗口大小
- HTTP请求头
- 初始化步骤

**2. 采集流程**
- 页面导航逻辑
- 数据提取节点
- 翻页处理
- 深度采集配置

**3. 字段定义**
- 字段名称和类型
- 元素选择器
- 数据提取方式
- 数据处理规则

**4. 保存设置**
- 数据去重规则
- 导出格式
- 保存位置

### 规则类型

**智能规则**
- 由AI自动生成
- 适合标准化页面
- 快速创建，无需配置
- 可以手动调整优化

**可视化规则**
- 通过图形界面创建
- 支持复杂采集场景
- 完全自定义控制
- 可重用和分享

## 节点（Node）

### 什么是节点？

节点是规则流程中的基本单元，每个节点代表一个特定的操作或数据提取步骤。多个节点通过连线组成完整的采集流程。

### 节点类型

#### 1. 起始节点（Start Node）

流程的入口点，定义采集的起始状态。

**配置项：**
- **URL**: 起始页面地址
- **视口大小**: 浏览器窗口尺寸（宽×高）
- **HTTP Headers**: 自定义请求头
- **初始步骤**: 页面加载后的操作（如登录、点击等）

**示例配置：**
```json
{
  "url": "https://example.com/products",
  "viewport": {
    "width": 1920,
    "height": 1080
  },
  "httpHeaders": [
    {
      "header": "User-Agent",
      "value": "Mozilla/5.0..."
    }
  ]
}
```

#### 2. 页面节点（Page Node）

打开新页面或标签页。

**触发方式：**
- **点击元素**: 点击链接或按钮打开新页面
- **打开URL**: 直接访问指定URL

**配置项：**
- **类型**: click_element 或 open_url
- **值**: 元素选择器或URL地址
- **操作步骤**: 页面打开后的操作

**使用场景：**
- 从列表页进入详情页
- 打开搜索结果链接
- 导航到不同分类页面

#### 3. 列表节点（List Node）

批量提取列表中的多条数据。

**配置项：**
- **列表容器选择器**: 包含所有列表项的容器
- **列表项选择器**: 单个列表项的选择器
- **字段配置**: 要提取的字段定义
- **翻页配置**: 如何翻页获取更多数据

**示例：**
```json
{
  "listSelector": ".product-list .product-item",
  "fields": [
    {
      "name": "title",
      "selector": ".title",
      "attr": "innerText"
    },
    {
      "name": "price",
      "selector": ".price",
      "attr": "innerText"
    }
  ],
  "pagination": {
    "type": "click_next",
    "config": {
      "selector": ".next-page"
    }
  }
}
```

#### 4. 详情节点（Detail Node）

提取单个页面的详细信息。

**配置项：**
- **字段配置**: 要提取的字段定义
- **数据处理**: 数据清洗和转换规则

**使用场景：**
- 提取商品详情页信息
- 采集文章完整内容
- 获取用户详细资料

### 节点连接

节点通过边（Edge）连接，形成数据流：

```
起始节点 → 列表节点 → 页面节点 → 详情节点
```

**执行顺序：**
1. 从起始节点开始
2. 按照连线顺序执行
3. 列表节点会为每条数据执行后续节点
4. 所有路径执行完成后保存数据

## 选择器（Selector）

### 什么是选择器？

选择器是用于定位网页元素的表达式。就像地址一样，告诉插件要提取哪个元素的数据。

### CSS选择器

最常用的选择器类型，语法简洁直观。

**基本选择器：**
```css
/* 标签选择器 */
div
h1
span

/* 类选择器 */
.product
.title
.price

/* ID选择器 */
#header
#main-content

/* 属性选择器 */
[data-id]
[href^="https"]
[class*="product"]
```

**组合选择器：**
```css
/* 后代选择器 */
.product .title

/* 子选择器 */
.product > .title

/* 相邻兄弟选择器 */
h2 + p

/* 通用兄弟选择器 */
h2 ~ p
```

**伪类选择器：**
```css
/* 第一个子元素 */
li:first-child

/* 最后一个子元素 */
li:last-child

/* 第n个子元素 */
li:nth-child(2)

/* 偶数元素 */
li:nth-child(even)
```

### XPath选择器

功能更强大的选择器，支持复杂查询。

**基本语法：**
```xpath
/* 选择所有div元素 */
//div

/* 选择特定class的div */
//div[@class='product']

/* 选择包含特定文本的元素 */
//div[contains(text(), '商品')]

/* 选择特定属性的元素 */
//a[@href='/detail']

/* 选择父元素 */
//div[@class='title']/parent::div

/* 选择兄弟元素 */
//div[@class='title']/following-sibling::div
```

**高级用法：**
```xpath
/* 选择第一个元素 */
(//div[@class='product'])[1]

/* 选择最后一个元素 */
(//div[@class='product'])[last()]

/* 条件选择 */
//div[@class='product' and @data-id='123']

/* 文本匹配 */
//div[text()='精确匹配']
//div[contains(text(), '包含匹配')]
```

### 选择器优先级

选择器的稳定性从高到低：

1. **ID选择器** - 最稳定（如果ID不变）
2. **data-*属性** - 通常很稳定
3. **语义化class** - 较稳定
4. **结构选择器** - 中等稳定
5. **动态class** - 不稳定

**推荐做法：**
```css
/* 好的选择器 */
#product-123
[data-product-id="123"]
.product-title

/* 避免使用 */
.css-1a2b3c4d  /* 动态生成的class */
div > div > div > span  /* 过深的层级 */
```

## 字段（Field）

### 什么是字段？

字段定义了要提取的具体数据项，包括字段名称、数据来源和处理方式。

### 字段配置

**基本属性：**
```json
{
  "id": "field_001",
  "name": "title",
  "selector": ".product-title",
  "attr": "innerText"
}
```

**属性说明：**
- **id**: 字段唯一标识
- **name**: 字段名称（导出时的列名）
- **selector**: 元素选择器
- **attr**: 要提取的属性

### 提取属性

**常用属性：**

| 属性 | 说明 | 示例 |
|------|------|------|
| innerText | 元素文本内容 | 商品标题 |
| innerHTML | 元素HTML内容 | `<span>标题</span>` |
| href | 链接地址 | https://example.com |
| src | 图片/资源地址 | https://example.com/img.jpg |
| value | 表单输入值 | 用户输入的文本 |
| title | 标题属性 | 鼠标悬停提示 |
| data-* | 自定义数据属性 | data-id="123" |

**提取示例：**
```javascript
// 提取文本
selector: ".title"
attr: "innerText"
// 结果: "商品标题"

// 提取链接
selector: "a.detail-link"
attr: "href"
// 结果: "https://example.com/product/123"

// 提取图片
selector: "img.product-img"
attr: "src"
// 结果: "https://example.com/images/product.jpg"

// 提取自定义属性
selector: ".product"
attr: "data-id"
// 结果: "123"
```

### 数据提取器（Extractor）

对提取的原始数据进行进一步处理。

#### 正则表达式提取器

使用正则表达式从文本中提取特定模式。

**配置：**
```json
{
  "type": "regex",
  "code": "\\d+\\.\\d+"
}
```

**示例：**
```
原始数据: "价格：¥199.99元"
正则表达式: \d+\.\d+
提取结果: "199.99"
```

#### Sed替换提取器

使用sed命令进行文本替换。

**配置：**
```json
{
  "type": "sed",
  "code": "s/¥//g;s/元//g"
}
```

**示例：**
```
原始数据: "¥199.99元"
Sed命令: s/¥//g;s/元//g
提取结果: "199.99"
```

#### JavaScript提取器

使用JavaScript代码进行复杂处理。

**配置：**
```json
{
  "type": "js",
  "code": "return value.replace(/[^0-9.]/g, '')"
}
```

**示例：**
```javascript
// 清理价格数据
return value.replace(/[^0-9.]/g, '')

// 转换日期格式
return new Date(value).toISOString()

// 计算折扣
const original = parseFloat(data.originalPrice)
const current = parseFloat(value)
return ((1 - current/original) * 100).toFixed(2) + '%'
```

## 翻页（Pagination）

### 什么是翻页？

翻页是指自动浏览多页内容以采集更多数据的功能。

### 翻页类型

#### 1. 点击下一页（click_next）

点击"下一页"按钮或页码链接。

**配置：**
```json
{
  "type": "click_next",
  "config": {
    "selector": ".next-page"
  }
}
```

**适用场景：**
- 传统分页导航
- 页码链接
- "下一页"按钮

#### 2. 无限滚动（scroll）

滚动到页面底部触发加载更多。

**配置：**
```json
{
  "type": "scroll",
  "config": {
    "selector": "body"
  }
}
```

**适用场景：**
- 社交媒体信息流
- 瀑布流布局
- 自动加载更多的列表

#### 3. 加载更多（load_more）

点击"加载更多"按钮。

**配置：**
```json
{
  "type": "load_more",
  "config": {
    "selector": ".load-more-btn"
  }
}
```

**适用场景：**
- 需要点击才加载的列表
- "查看更多"按钮
- "展开全部"功能

## 步骤（Steps）

### 什么是步骤？

步骤是在页面上执行的自动化操作序列，如点击、输入、滚动等。

### 步骤类型

#### 点击（Click）

点击页面元素。

**配置：**
```json
{
  "type": "click",
  "selectors": [["button.submit"]]
}
```

#### 双击（DoubleClick）

双击页面元素。

**配置：**
```json
{
  "type": "doubleClick",
  "selectors": [[".item"]]
}
```

#### 输入（Change）

在输入框中输入文本。

**配置：**
```json
{
  "type": "change",
  "selectors": [["input[name='keyword']"]],
  "value": "搜索关键词"
}
```

#### 按键（KeyDown/KeyUp）

模拟键盘按键。

**配置：**
```json
{
  "type": "keyDown",
  "key": "Enter"
}
```

#### 导航（Navigate）

跳转到指定URL。

**配置：**
```json
{
  "type": "navigate",
  "url": "https://example.com/page2"
}
```

#### 滚动（Scroll）

滚动页面。

**配置：**
```json
{
  "type": "scroll",
  "x": 0,
  "y": 1000
}
```

### 步骤组合

多个步骤可以组合成复杂的操作流程：

**示例：搜索并采集**
```json
{
  "steps": [
    {
      "type": "change",
      "selectors": [["input.search"]],
      "value": "关键词"
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

## 工作原理

### 采集流程

```
1. 加载起始页面
   ↓
2. 执行初始步骤（如登录）
   ↓
3. 定位列表容器
   ↓
4. 提取列表数据
   ↓
5. 对每条数据：
   - 打开详情页（如果需要）
   - 提取详细信息
   - 合并数据
   ↓
6. 翻页（如果有更多页）
   ↓
7. 重复步骤3-6
   ↓
8. 保存所有数据
```

### 数据流转

```
网页元素 → 选择器定位 → 属性提取 → 数据处理 → 字段值 → 数据记录 → 导出文件
```

### 执行上下文

每个节点执行时都有一个上下文（Context），包含：

- **当前页面**: 正在操作的浏览器页面
- **当前节点**: 正在执行的节点配置
- **数据路径**: 从起始到当前的数据累积
- **日志记录**: 执行过程的日志信息

## 最佳实践

### 1. 选择器编写

✅ **推荐：**
```css
/* 使用稳定的属性 */
[data-product-id]
.product-title
#main-content

/* 适当的层级 */
.product-list > .item
```

❌ **避免：**
```css
/* 动态生成的class */
.css-1a2b3c

/* 过深的层级 */
body > div > div > div > div > span

/* 依赖位置 */
div:nth-child(5)
```

### 2. 数据提取

✅ **推荐：**
- 提取原始数据，后期处理
- 使用合适的提取器清洗数据
- 设置默认值处理缺失数据

❌ **避免：**
- 提取包含大量HTML标签的内容
- 不处理特殊字符
- 忽略数据验证

### 3. 翻页处理

✅ **推荐：**
- 设置合理的翻页延迟
- 添加翻页次数限制
- 检测是否还有更多数据

❌ **避免：**
- 无限制翻页
- 翻页过快导致被封
- 不处理翻页失败

### 4. 错误处理

✅ **推荐：**
- 添加等待时间确保页面加载
- 设置超时时间
- 记录详细的错误日志

❌ **避免：**
- 假设元素一定存在
- 不处理网络错误
- 忽略异常情况

## 下一步

现在您已经了解了基本概念，可以继续学习：

- [管理界面](./management-interface) - 了解插件的各个功能模块
- [智能采集](./intelligent-collection) - 使用AI快速创建采集规则
- [可视化规则开发](./visual-rule-development) - 创建复杂的采集规则
- [实战教程](./tutorials) - 通过实例学习数据采集
