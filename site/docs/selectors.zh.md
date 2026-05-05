# 元素选择器

元素选择器是数据采集的基础，用于精确定位网页中的元素。掌握选择器的编写技巧，能够大大提高采集的准确性和稳定性。

## 选择器类型

八爪鱼爬虫支持两种主要的选择器类型：CSS选择器和XPath选择器。

### CSS选择器

CSS选择器是最常用的选择器类型，语法简洁，性能优秀。

#### 基本选择器

**标签选择器**:
```css
/* 选择所有div元素 */
div

/* 选择所有p元素 */
p

/* 选择所有a元素 */
a
```

**类选择器**:
```css
/* 选择class为product的元素 */
.product

/* 选择class为title的元素 */
.title

/* 选择同时有多个class的元素 */
.product.featured
```

**ID选择器**:
```css
/* 选择id为header的元素 */
#header

/* 选择id为main-content的元素 */
#main-content
```

**属性选择器**:
```css
/* 选择有data-id属性的元素 */
[data-id]

/* 选择data-id等于123的元素 */
[data-id="123"]

/* 选择href以https开头的元素 */
[href^="https"]

/* 选择href以.pdf结尾的元素 */
[href$=".pdf"]

/* 选择class包含product的元素 */
[class*="product"]
```

#### 组合选择器

**后代选择器**:
```css
/* 选择.product内的所有.title */
.product .title

/* 选择#content内的所有p */
#content p
```

**子选择器**:
```css
/* 选择.product的直接子元素.title */
.product > .title

/* 选择ul的直接子元素li */
ul > li
```

**相邻兄弟选择器**:
```css
/* 选择h2后面紧邻的p */
h2 + p

/* 选择.title后面紧邻的.price */
.title + .price
```

**通用兄弟选择器**:
```css
/* 选择h2后面所有的p */
h2 ~ p

/* 选择.title后面所有的div */
.title ~ div
```

#### 伪类选择器

**结构伪类**:
```css
/* 第一个子元素 */
li:first-child

/* 最后一个子元素 */
li:last-child

/* 第n个子元素 */
li:nth-child(2)

/* 奇数元素 */
li:nth-child(odd)

/* 偶数元素 */
li:nth-child(even)

/* 每3个元素 */
li:nth-child(3n)

/* 倒数第二个 */
li:nth-last-child(2)
```

**类型伪类**:
```css
/* 第一个p元素 */
p:first-of-type

/* 最后一个p元素 */
p:last-of-type

/* 第二个p元素 */
p:nth-of-type(2)
```

**状态伪类**:
```css
/* 选择被hover的元素 */
a:hover

/* 选择被选中的元素 */
input:checked

/* 选择被禁用的元素 */
input:disabled

/* 选择空元素 */
div:empty

/* 选择非空元素 */
div:not(:empty)
```

#### 伪元素选择器

```css
/* 元素的第一行 */
p::first-line

/* 元素的第一个字母 */
p::first-letter

/* 元素之前插入的内容 */
.price::before

/* 元素之后插入的内容 */
.price::after
```

### XPath选择器

XPath是更强大的选择器，支持复杂的查询和向上查找。

#### 基本语法

**选择节点**:
```xpath
/* 选择所有div */
//div

/* 选择根节点下的div */
/div

/* 选择当前节点下的div */
./div

/* 选择父节点 */
..
```

**按属性选择**:
```xpath
/* 选择有class属性的div */
//div[@class]

/* 选择class为product的div */
//div[@class='product']

/* 选择class包含product的div */
//div[contains(@class, 'product')]

/* 选择data-id为123的元素 */
//*[@data-id='123']
```

**按文本选择**:
```xpath
/* 选择文本为"标题"的元素 */
//*[text()='标题']

/* 选择包含"价格"文本的元素 */
//*[contains(text(), '价格')]

/* 选择以"商品"开头的文本 */
//*[starts-with(text(), '商品')]
```

#### 轴选择

**向下选择**:
```xpath
/* 所有后代 */
//div[@class='product']//span

/* 直接子元素 */
//div[@class='product']/span

/* 所有后代中的特定元素 */
//div[@class='product']//descendant::span
```

**向上选择**:
```xpath
/* 父元素 */
//span[@class='title']/parent::div

/* 祖先元素 */
//span[@class='title']/ancestor::div

/* 特定祖先 */
//span[@class='title']/ancestor::div[@class='product']
```

**兄弟选择**:
```xpath
/* 后面的兄弟 */
//div[@class='title']/following-sibling::div

/* 前面的兄弟 */
//div[@class='price']/preceding-sibling::div

/* 后面的第一个兄弟 */
//div[@class='title']/following-sibling::div[1]
```

#### 位置选择

```xpath
/* 第一个元素 */
(//div[@class='product'])[1]

/* 最后一个元素 */
(//div[@class='product'])[last()]

/* 第2到第5个元素 */
//div[@class='product'][position()>=2 and position()<=5]

/* 前3个元素 */
//div[@class='product'][position()<=3]
```

#### 条件选择

```xpath
/* AND条件 */
//div[@class='product' and @data-id='123']

/* OR条件 */
//div[@class='product' or @class='item']

/* NOT条件 */
//div[not(@class='hidden')]

/* 多个条件 */
//div[@class='product' and contains(@data-category, 'phone') and @data-price>1000]
```

## 选择器最佳实践

### 1. 选择稳定的属性

✅ **推荐使用**:
```css
/* ID（如果不变） */
#product-123

/* 语义化的class */
.product-title
.product-price

/* data-*属性 */
[data-product-id="123"]
[data-testid="product-card"]

/* 语义化标签 */
article
section
nav
```

❌ **避免使用**:
```css
/* 动态生成的class */
.css-1a2b3c4d
.MuiBox-root-123

/* 样式相关的class */
.text-red
.mt-4
.flex

/* 过于通用的选择器 */
div
span
```

### 2. 适当的层级深度

✅ **推荐**:
```css
/* 2-3层，清晰明确 */
.product-list .product-item .title

/* 使用子选择器限制层级 */
.product-list > .product-item > .title
```

❌ **避免**:
```css
/* 层级过深 */
body > div > div > div > div > div > span

/* 层级过浅，不够具体 */
.title
```

### 3. 使用组合选择器

✅ **推荐**:
```css
/* 组合多个条件 */
div.product[data-id="123"]

/* 使用属性选择器 */
a[href^="/product/"]

/* 使用伪类 */
.product-item:first-child
```

### 4. 避免位置依赖

✅ **推荐**:
```css
/* 使用语义化选择器 */
.product-title
[data-field="title"]
```

❌ **避免**:
```css
/* 依赖位置 */
div:nth-child(5)
.container > div:nth-child(3) > span:nth-child(2)
```

## 选择器工具

### 浏览器开发者工具

**Chrome DevTools**:
1. 右键点击元素
2. 选择"检查"
3. 在Elements面板中查看元素
4. 右键元素 → Copy → Copy selector

**测试选择器**:
```javascript
// 在Console中测试CSS选择器
document.querySelector('.product-title')
document.querySelectorAll('.product-item')

// 测试XPath
$x('//div[@class="product"]')
```

### 插件内置工具

**元素选择器**:
1. 点击"选择元素"按钮
2. 鼠标悬停在目标元素上
3. 元素高亮显示
4. 点击选择元素
5. 自动生成选择器

**选择器验证**:
1. 输入选择器
2. 点击"验证"按钮
3. 查看匹配的元素数量
4. 高亮显示匹配的元素

## 常见场景

### 场景1: 列表项选择

**HTML结构**:
```html
<ul class="product-list">
  <li class="product-item">
    <h3 class="title">商品1</h3>
    <span class="price">¥99</span>
  </li>
  <li class="product-item">
    <h3 class="title">商品2</h3>
    <span class="price">¥199</span>
  </li>
</ul>
```

**选择器**:
```css
/* 列表容器 */
.product-list

/* 列表项 */
.product-list .product-item

/* 标题 */
.product-item .title

/* 价格 */
.product-item .price
```

### 场景2: 表格数据选择

**HTML结构**:
```html
<table class="data-table">
  <thead>
    <tr>
      <th>姓名</th>
      <th>年龄</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>张三</td>
      <td>25</td>
    </tr>
  </tbody>
</table>
```

**选择器**:
```css
/* 表格 */
.data-table

/* 表头 */
.data-table thead th

/* 数据行 */
.data-table tbody tr

/* 单元格 */
.data-table tbody td

/* 第一列 */
.data-table tbody td:nth-child(1)

/* 第二列 */
.data-table tbody td:nth-child(2)
```

### 场景3: 嵌套结构选择

**HTML结构**:
```html
<div class="card">
  <div class="card-header">
    <h2>标题</h2>
  </div>
  <div class="card-body">
    <p>内容</p>
  </div>
  <div class="card-footer">
    <a href="#">链接</a>
  </div>
</div>
```

**选择器**:
```css
/* 卡片 */
.card

/* 标题 */
.card .card-header h2

/* 内容 */
.card .card-body p

/* 链接 */
.card .card-footer a
```

### 场景4: 动态内容选择

**HTML结构**:
```html
<div class="content" data-loaded="true">
  <div class="item" data-id="123">
    <span class="label">标签</span>
    <span class="value">值</span>
  </div>
</div>
```

**选择器**:
```css
/* 已加载的内容 */
.content[data-loaded="true"]

/* 特定ID的项 */
.item[data-id="123"]

/* 标签 */
.item .label

/* 值 */
.item .value
```

## 选择器调试

### 问题1: 选择器匹配多个元素

**原因**: 选择器不够具体

**解决方法**:
```css
/* 不够具体 */
.title

/* 添加父级限定 */
.product-item .title

/* 添加属性限定 */
.title[data-type="product"]

/* 使用更具体的class */
.product-title
```

### 问题2: 选择器找不到元素

**原因**: 
- 选择器错误
- 元素动态加载
- 元素在iframe中

**解决方法**:
```javascript
// 1. 验证选择器
document.querySelector('.your-selector')

// 2. 等待元素加载
await page.waitForSelector('.your-selector')

// 3. 检查iframe
const frame = page.frames().find(f => f.name() === 'content')
await frame.waitForSelector('.your-selector')
```

### 问题3: 选择器不稳定

**原因**: 使用了动态生成的属性

**解决方法**:
```css
/* 不稳定 */
.css-1a2b3c4d

/* 改用稳定属性 */
[data-testid="product-card"]
.product-card
#product-123
```

## 选择器性能

### 性能对比

| 选择器类型 | 性能 | 说明 |
|-----------|------|------|
| ID选择器 | ⭐⭐⭐⭐⭐ | 最快 |
| 类选择器 | ⭐⭐⭐⭐ | 很快 |
| 标签选择器 | ⭐⭐⭐ | 较快 |
| 属性选择器 | ⭐⭐ | 一般 |
| 伪类选择器 | ⭐⭐ | 一般 |
| XPath | ⭐ | 较慢 |

### 优化建议

✅ **推荐**:
```css
/* 使用ID */
#product-123

/* 使用class */
.product-title

/* 限定范围 */
.product-list .product-item
```

❌ **避免**:
```css
/* 通用选择器 */
*

/* 过于复杂的选择器 */
div > div > div > span:nth-child(2)

/* 不必要的后代选择器 */
body div.container div.content div.item span.title
```

## 实战技巧

### 技巧1: 使用浏览器复制选择器

1. 右键点击元素
2. 选择"检查"
3. 在Elements面板中右键元素
4. Copy → Copy selector
5. 粘贴到插件中

### 技巧2: 使用$0快速测试

```javascript
// 在Elements面板选中元素后
// 在Console中使用$0引用
$0.className
$0.getAttribute('data-id')
$0.textContent
```

### 技巧3: 批量测试选择器

```javascript
// 测试选择器匹配数量
document.querySelectorAll('.product-item').length

// 查看所有匹配元素
document.querySelectorAll('.product-item').forEach(el => {
  console.log(el.textContent)
})
```

### 技巧4: 使用contains选择文本

```xpath
/* XPath选择包含特定文本的元素 */
//div[contains(text(), '价格')]
//span[contains(text(), '¥')]
//a[contains(text(), '详情')]
```

## 下一步

- [数据采集](./data-collection) - 使用选择器提取数据
- [翻页配置](./pagination) - 配置翻页选择器
- [可视化规则开发](./visual-rule-development) - 在规则中使用选择器
- [实战教程](./tutorials) - 选择器实战案例
