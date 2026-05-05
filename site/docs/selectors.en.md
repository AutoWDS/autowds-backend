# Element Selectors

Element selectors are the foundation of data collection, used to precisely locate elements in web pages. Mastering selector writing techniques can greatly improve the accuracy and stability of collection.

## Selector Types

AutoWDS supports two main types of selectors: CSS selectors and XPath selectors.

### CSS Selectors

CSS selectors are the most commonly used selector type, with concise syntax and excellent performance.

#### Basic Selectors

**Tag Selectors**:
```css
/* Select all div elements */
div

/* Select all p elements */
p

/* Select all a elements */
a
```

**Class Selectors**:
```css
/* Select elements with class product */
.product

/* Select elements with class title */
.title

/* Select elements with multiple classes */
.product.featured
```

**ID Selectors**:
```css
/* Select element with id header */
#header

/* Select element with id main-content */
#main-content
```

**Attribute Selectors**:
```css
/* Select elements with data-id attribute */
[data-id]

/* Select elements with data-id equal to 123 */
[data-id="123"]

/* Select elements with href starting with https */
[href^="https"]

/* Select elements with href ending with .pdf */
[href$=".pdf"]

/* Select elements with class containing product */
[class*="product"]
```

#### Combination Selectors

**Descendant Selectors**:
```css
/* Select all .title inside .product */
.product .title

/* Select all p inside #content */
#content p
```

**Child Selectors**:
```css
/* Select direct child .title of .product */
.product > .title

/* Select direct child li of ul */
ul > li
```

**Adjacent Sibling Selectors**:
```css
/* Select p immediately following h2 */
h2 + p

/* Select .price immediately following .title */
.title + .price
```

**General Sibling Selectors**:
```css
/* Select all p following h2 */
h2 ~ p

/* Select all div following .title */
.title ~ div
```

#### Pseudo-class Selectors

**Structural Pseudo-classes**:
```css
/* First child element */
li:first-child

/* Last child element */
li:last-child

/* Nth child element */
li:nth-child(2)

/* Odd elements */
li:nth-child(odd)

/* Even elements */
li:nth-child(even)

/* Every 3rd element */
li:nth-child(3n)

/* Second to last */
li:nth-last-child(2)
```

**Type Pseudo-classes**:
```css
/* First p element */
p:first-of-type

/* Last p element */
p:last-of-type

/* Second p element */
p:nth-of-type(2)
```

**State Pseudo-classes**:
```css
/* Select hovered elements */
a:hover

/* Select checked elements */
input:checked

/* Select disabled elements */
input:disabled

/* Select empty elements */
div:empty

/* Select non-empty elements */
div:not(:empty)
```

#### Pseudo-element Selectors

```css
/* First line of element */
p::first-line

/* First letter of element */
p::first-letter

/* Content inserted before element */
.price::before

/* Content inserted after element */
.price::after
```

### XPath Selectors

XPath is a more powerful selector that supports complex queries and upward searching.

#### Basic Syntax

**Select Nodes**:
```xpath
/* Select all div */
//div

/* Select div under root node */
/div

/* Select div under current node */
./div

/* Select parent node */
..
```

**Select by Attributes**:
```xpath
/* Select div with class attribute */
//div[@class]

/* Select div with class product */
//div[@class='product']

/* Select div with class containing product */
//div[contains(@class, 'product')]

/* Select element with data-id 123 */
//*[@data-id='123']
```

**Select by Text**:
```xpath
/* Select element with text "Title" */
//*[text()='Title']

/* Select element containing "Price" text */
//*[contains(text(), 'Price')]

/* Select text starting with "Product" */
//*[starts-with(text(), 'Product')]
```

#### Axis Selection

**Downward Selection**:
```xpath
/* All descendants */
//div[@class='product']//span

/* Direct children */
//div[@class='product']/span

/* All descendants of specific element */
//div[@class='product']//descendant::span
```

**Upward Selection**:
```xpath
/* Parent element */
//span[@class='title']/parent::div

/* Ancestor element */
//span[@class='title']/ancestor::div

/* Specific ancestor */
//span[@class='title']/ancestor::div[@class='product']
```

**Sibling Selection**:
```xpath
/* Following siblings */
//div[@class='title']/following-sibling::div

/* Preceding siblings */
//div[@class='price']/preceding-sibling::div

/* First following sibling */
//div[@class='title']/following-sibling::div[1]
```

#### Position Selection

```xpath
/* First element */
(//div[@class='product'])[1]

/* Last element */
(//div[@class='product'])[last()]

/* Elements 2 to 5 */
//div[@class='product'][position()>=2 and position()<=5]

/* First 3 elements */
//div[@class='product'][position()<=3]
```

#### Conditional Selection

```xpath
/* AND condition */
//div[@class='product' and @data-id='123']

/* OR condition */
//div[@class='product' or @class='item']

/* NOT condition */
//div[not(@class='hidden')]

/* Multiple conditions */
//div[@class='product' and contains(@data-category, 'phone') and @data-price>1000]
```

## Selector Best Practices

### 1. Choose Stable Attributes

✅ **Recommended**:
```css
/* ID (if stable) */
#product-123

/* Semantic classes */
.product-title
.product-price

/* data-* attributes */
[data-product-id="123"]
[data-testid="product-card"]

/* Semantic tags */
article
section
nav
```

❌ **Avoid**:
```css
/* Dynamically generated classes */
.css-1a2b3c4d
.MuiBox-root-123

/* Style-related classes */
.text-red
.mt-4
.flex

/* Overly generic selectors */
div
span
```

### 2. Appropriate Hierarchy Depth

✅ **Recommended**:
```css
/* 2-3 levels, clear and specific */
.product-list .product-item .title

/* Use child selectors to limit hierarchy */
.product-list > .product-item > .title
```

❌ **Avoid**:
```css
/* Too deep hierarchy */
body > div > div > div > div > div > span

/* Too shallow, not specific enough */
.title
```

### 3. Use Combination Selectors

✅ **Recommended**:
```css
/* Combine multiple conditions */
div.product[data-id="123"]

/* Use attribute selectors */
a[href^="/product/"]

/* Use pseudo-classes */
.product-item:first-child
```

### 4. Avoid Position Dependency

✅ **Recommended**:
```css
/* Use semantic selectors */
.product-title
[data-field="title"]
```

❌ **Avoid**:
```css
/* Position dependent */
div:nth-child(5)
.container > div:nth-child(3) > span:nth-child(2)
```

## Selector Tools

### Browser Developer Tools

**Chrome DevTools**:
1. Right-click on element
2. Select "Inspect"
3. View element in Elements panel
4. Right-click element → Copy → Copy selector

**Test Selectors**:
```javascript
// Test CSS selector in Console
document.querySelector('.product-title')
document.querySelectorAll('.product-item')

// Test XPath
$x('//div[@class="product"]')
```

### Plugin Built-in Tools

**Element Selector**:
1. Click "Select Element" button
2. Hover mouse over target element
3. Element highlights
4. Click to select element
5. Automatically generate selector

**Selector Validation**:
1. Enter selector
2. Click "Validate" button
3. View number of matching elements
4. Highlight matching elements

## Common Scenarios

### Scenario 1: List Item Selection

**HTML Structure**:
```html
<ul class="product-list">
  <li class="product-item">
    <h3 class="title">Product 1</h3>
    <span class="price">$99</span>
  </li>
  <li class="product-item">
    <h3 class="title">Product 2</h3>
    <span class="price">$199</span>
  </li>
</ul>
```

**Selectors**:
```css
/* List container */
.product-list

/* List items */
.product-list .product-item

/* Titles */
.product-item .title

/* Prices */
.product-item .price
```

### Scenario 2: Table Data Selection

**HTML Structure**:
```html
<table class="data-table">
  <thead>
    <tr>
      <th>Name</th>
      <th>Age</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>John</td>
      <td>25</td>
    </tr>
  </tbody>
</table>
```

**Selectors**:
```css
/* Table */
.data-table

/* Headers */
.data-table thead th

/* Data rows */
.data-table tbody tr

/* Cells */
.data-table tbody td

/* First column */
.data-table tbody td:nth-child(1)

/* Second column */
.data-table tbody td:nth-child(2)
```

### Scenario 3: Nested Structure Selection

**HTML Structure**:
```html
<div class="card">
  <div class="card-header">
    <h2>Title</h2>
  </div>
  <div class="card-body">
    <p>Content</p>
  </div>
  <div class="card-footer">
    <a href="#">Link</a>
  </div>
</div>
```

**Selectors**:
```css
/* Card */
.card

/* Title */
.card .card-header h2

/* Content */
.card .card-body p

/* Link */
.card .card-footer a
```

### Scenario 4: Dynamic Content Selection

**HTML Structure**:
```html
<div class="content" data-loaded="true">
  <div class="item" data-id="123">
    <span class="label">Label</span>
    <span class="value">Value</span>
  </div>
</div>
```

**Selectors**:
```css
/* Loaded content */
.content[data-loaded="true"]

/* Specific ID item */
.item[data-id="123"]

/* Label */
.item .label

/* Value */
.item .value
```

## Selector Debugging

### Issue 1: Selector matches multiple elements

**Cause**: Selector not specific enough

**Solution**:
```css
/* Not specific enough */
.title

/* Add parent limitation */
.product-item .title

/* Add attribute limitation */
.title[data-type="product"]

/* Use more specific class */
.product-title
```

### Issue 2: Selector cannot find element

**Causes**: 
- Incorrect selector
- Element dynamically loaded
- Element in iframe

**Solutions**:
```javascript
// 1. Validate selector
document.querySelector('.your-selector')

// 2. Wait for element to load
await page.waitForSelector('.your-selector')

// 3. Check iframe
const frame = page.frames().find(f => f.name() === 'content')
await frame.waitForSelector('.your-selector')
```

### Issue 3: Selector unstable

**Cause**: Using dynamically generated attributes

**Solution**:
```css
/* Unstable */
.css-1a2b3c4d

/* Use stable attributes */
[data-testid="product-card"]
.product-card
#product-123
```

## Selector Performance

### Performance Comparison

| Selector Type | Performance | Description |
|---------------|-------------|-------------|
| ID Selector | ⭐⭐⭐⭐⭐ | Fastest |
| Class Selector | ⭐⭐⭐⭐ | Very fast |
| Tag Selector | ⭐⭐⭐ | Fast |
| Attribute Selector | ⭐⭐ | Average |
| Pseudo-class Selector | ⭐⭐ | Average |
| XPath | ⭐ | Slower |

### Optimization Recommendations

✅ **Recommended**:
```css
/* Use ID */
#product-123

/* Use class */
.product-title

/* Limit scope */
.product-list .product-item
```

❌ **Avoid**:
```css
/* Universal selector */
*

/* Overly complex selectors */
div > div > div > span:nth-child(2)

/* Unnecessary descendant selectors */
body div.container div.content div.item span.title
```

## Practical Tips

### Tip 1: Use browser copy selector

1. Right-click on element
2. Select "Inspect"
3. Right-click element in Elements panel
4. Copy → Copy selector
5. Paste into plugin

### Tip 2: Use $0 for quick testing

```javascript
// After selecting element in Elements panel
// Use $0 reference in Console
$0.className
$0.getAttribute('data-id')
$0.textContent
```

### Tip 3: Batch test selectors

```javascript
// Test selector match count
document.querySelectorAll('.product-item').length

// View all matching elements
document.querySelectorAll('.product-item').forEach(el => {
  console.log(el.textContent)
})
```

### Tip 4: Use contains to select text

```xpath
/* XPath select elements containing specific text */
//div[contains(text(), 'Price')]
//span[contains(text(), '$')]
//a[contains(text(), 'Details')]
```

## Next Steps

- [Data Collection](./data-collection) - Use selectors to extract data
- [Pagination Configuration](./pagination) - Configure pagination selectors
- [Visual Rule Development](./visual-rule-development) - Use selectors in rules
- [Tutorials](./tutorials) - Selector practical cases