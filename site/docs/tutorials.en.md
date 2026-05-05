# Tutorials

Learn how to use AutoWDS for data collection through practical cases. This tutorial covers various collection scenarios from simple to complex.

## Tutorial 1: Collecting E-commerce Product Lists

### Objective

Collect mobile phone product lists from an e-commerce website, including product names, prices, ratings, sales volume, etc.

### Steps

#### 1. Analyze Target Website

**URL**: `https://example.com/search?q=mobile`

**Page Structure**:
```html
<div class="product-list">
  <div class="product-item">
    <img class="product-img" src="...">
    <h3 class="product-title">iPhone 15 Pro</h3>
    <span class="product-price">$9999</span>
    <span class="product-rating">4.9 stars</span>
    <span class="product-sales">Sold 10000+</span>
    <a class="product-link" href="/product/123">View Details</a>
  </div>
  <!-- More products -->
</div>
<div class="pagination">
  <a class="next-page">Next Page</a>
</div>
```

#### 2. Create Collection Rules

**Using Intelligent Collection**:
1. Open target page
2. Right-click and select "Intelligent Collection"
3. AI automatically identifies fields
4. Confirm and adjust fields

**Or Use Visual Rules**:

**Start Node Configuration**:
```json
{
  "url": "https://example.com/search?q=mobile",
  "viewport": {
    "width": 1920,
    "height": 1080
  }
}
```

**List Node Configuration**:
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

#### 3. Test and Run

1. Click "Test" button
2. View collection results
3. Confirm data is correct
4. Click "Run" to start collection

#### 4. Export Data

Export as Excel after collection completes:
```
Filename: mobile_products_2024-01-15.xlsx
Data count: 500 records
Fields: Title, Price, Rating, Sales, Image, Link
```

### Expected Results

```csv
Title,Price,Rating,Sales,Image,Link
iPhone 15 Pro,9999,4.9,10000,https://...,/product/123
Samsung S24,8999,4.8,8000,https://...,/product/456
Xiaomi 14,3999,4.7,15000,https://...,/product/789
```

---

## Tutorial 2: Deep Collection of Product Details

### Objective

Based on Tutorial 1, enter each product's detail page to collect more detailed information.

### Steps

#### 1. Extend Rules

Add page node and detail node after list node.

**Flow**:
```
Start Node → List Node → Page Node → Detail Node
```

#### 2. Configure Page Node

```json
{
  "type": "click_element",
  "value": ".product-link"
}
```

#### 3. Configure Detail Node

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

#### 4. Data Merging

List page and detail page data will be automatically merged:

```json
{
  // List page data
  "title": "iPhone 15 Pro",
  "price": "9999",
  "rating": "4.9",
  
  // Detail page data
  "description": "New A17 Pro chip...",
  "brand": "Apple",
  "model": "A2848",
  "specifications": "6.7-inch screen...",
  "images": ["img1.jpg", "img2.jpg", "img3.jpg"]
}
```

### Notes

- Set reasonable access intervals (recommended 3-5 seconds)
- Add error handling mechanisms
- Monitor collection progress
- Save data regularly

---

## Tutorial 3: Collecting News Articles

### Objective

Collect latest articles from news websites, including titles, authors, publication time, article content.

### Steps

#### 1. Analyze Page Structure

**List Page**:
```html
<div class="news-list">
  <article class="news-item">
    <h2 class="news-title">
      <a href="/article/123">News Title</a>
    </h2>
    <span class="news-author">Author Name</span>
    <time class="news-time">2024-01-15 10:30</time>
    <p class="news-summary">Article summary...</p>
  </article>
</div>
```

**Detail Page**:
```html
<article class="article">
  <h1 class="article-title">News Title</h1>
  <div class="article-meta">
    <span class="author">Author Name</span>
    <time class="publish-time">2024-01-15 10:30</time>
  </div>
  <div class="article-content">
    <p>Article content...</p>
  </div>
</article>
```

#### 2. Configure Collection Rules

**List Node**:
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

**Detail Node**:
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

#### 3. Data Cleaning

**Clean Time Format**:
```javascript
// Input: "2 hours ago"
// Output: "2024-01-15 14:30:00"
const now = new Date()
const match = value.match(/(\d+) hours ago/)
if (match) {
  now.setHours(now.getHours() - parseInt(match[1]))
  return now.toISOString()
}
return value
```

**Clean Article Content**:
```javascript
// Remove extra whitespace and HTML tags
return value
  .replace(/<[^>]+>/g, '')
  .replace(/\s+/g, ' ')
  .trim()
```

---

## Tutorial 4: Collecting Job Postings

### Objective

Collect job information from recruitment websites, including job titles, companies, salaries, requirements, etc.

### Steps

#### 1. Configure Search

**Add Search Steps to Start Node**:
```json
{
  "url": "https://jobs.example.com",
  "steps": [
    {
      "type": "change",
      "selectors": [["input.search-keyword"]],
      "value": "Frontend Engineer"
    },
    {
      "type": "change",
      "selectors": [["input.search-city"]],
      "value": "New York"
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

#### 2. Collect List

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

#### 3. Data Processing

**Salary Range Extraction**:
```javascript
// Input: "$60K-$80K·12 months"
// Output: { min: 60000, max: 80000, months: 12 }
const match = value.match(/\$(\d+)K-\$(\d+)K·(\d+) months/)
if (match) {
  return {
    min: parseInt(match[1]) * 1000,
    max: parseInt(match[2]) * 1000,
    months: parseInt(match[3])
  }
}
```

---

## Tutorial 5: Batch Keyword Collection

### Objective

Batch search multiple keywords and collect results.

### Steps

#### 1. Prepare Keyword List

Create CSV file `keywords.csv`:
```csv
keyword
iPhone 15
Samsung S24
Xiaomi 14
Huawei Mate60
OPPO Find X7
```

#### 2. Configure Batch Task

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

#### 3. Execute Batch Collection

Plugin will automatically:
1. Read keyword list
2. Search each keyword sequentially
3. Collect results for each keyword
4. Merge all data

#### 4. Group Results

Export grouped by keyword:
```
iPhone_15_results.xlsx
Samsung_S24_results.xlsx
Xiaomi_14_results.xlsx
...
```

---

## Tutorial 6: Scheduled Price Monitoring

### Objective

Daily scheduled collection of product prices to monitor price changes.

### Steps

#### 1. Create Collection Rules

Configure product list collection rules (refer to Tutorial 1).

#### 2. Configure Scheduled Task

```json
{
  "schedule": {
    "enabled": true,
    "cron": "0 9 * * *",
    "timezone": "America/New_York"
  }
}
```

#### 3. Configure Incremental Collection

```json
{
  "incremental": {
    "enabled": true,
    "compareField": "productId",
    "trackChanges": ["price", "stock"]
  }
}
```

#### 4. Configure Price Change Notifications

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

#### 5. Data Analysis

Export historical data to analyze price trends:
```csv
Date,Product ID,Product Name,Price,Change
2024-01-15,123,iPhone 15,9999,0
2024-01-16,123,iPhone 15,9899,-100
2024-01-17,123,iPhone 15,9899,0
```

---

## Tutorial 7: Collecting Social Media Content

### Objective

Collect post content from social media platforms.

### Steps

#### 1. Handle Login

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

#### 2. Infinite Scroll Collection

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

## Tutorial 8: Collecting Table Data

### Objective

Collect table data from web pages.

### Steps

#### 1. Analyze Table Structure

```html
<table class="data-table">
  <thead>
    <tr>
      <th>Name</th>
      <th>Age</th>
      <th>City</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>John</td>
      <td>25</td>
      <td>New York</td>
    </tr>
  </tbody>
</table>
```

#### 2. Configure Collection Rules

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

#### 3. Dynamic Header Handling

If headers are dynamic:

```javascript
// First collect headers
const headers = Array.from(
  document.querySelectorAll('.data-table thead th')
).map(th => th.textContent.trim())

// Generate field configuration based on headers
const fields = headers.map((header, index) => ({
  name: header,
  selector: `td:nth-child(${index + 1})`,
  attr: 'innerText'
}))
```

---

## Common Issues and Solutions

### Issue 1: Slow Collection Speed

**Solutions**:
1. Reduce unnecessary fields
2. Optimize selectors
3. Adjust concurrency settings
4. Use incremental collection

### Issue 2: Incomplete Data

**Solutions**:
1. Increase wait time
2. Check selectors
3. Handle dynamic loading
4. Add error retry

### Issue 3: Website Blocking

**Solutions**:
1. Reduce collection frequency
2. Add random delays
3. Use proxy IPs
4. Simulate human operations

### Issue 4: Inconsistent Data Format

**Solutions**:
1. Use data extractors
2. Configure data transformation
3. Standardize data formats
4. Data validation

## Best Practices Summary

### 1. Pre-collection Preparation

✅ **Recommended**:
- Analyze target website structure
- Determine collection fields
- Test selectors
- Create collection plan

### 2. Rule Configuration

✅ **Recommended**:
- Use stable selectors
- Set reasonable wait times
- Configure error handling
- Add data validation

### 3. Execute Collection

✅ **Recommended**:
- Test on small scale first
- Monitor collection process
- Handle errors promptly
- Save data regularly

### 4. Data Processing

✅ **Recommended**:
- Clean data
- Validate completeness
- Standardize formats
- Export promptly

## Next Steps

- [Intelligent Collection](./intelligent-collection) - Use AI to quickly create rules
- [Visual Rule Development](./visual-rule-development) - Create complex rules
- [Scheduled Tasks](./scheduled-tasks) - Automated collection
- [Data Export](./data-export) - Export and process data