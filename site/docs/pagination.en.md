# Pagination Configuration

Pagination is an important feature in data collection, used to automatically browse multi-page content to collect more data. AutoWDS supports multiple pagination methods to adapt to different website pagination mechanisms.

## Pagination Types

### 1. Click Next (click_next)

The most common pagination method, achieved by clicking "Next Page" button or page number links.

#### Applicable Scenarios

- Traditional numeric pagination navigation
- "Previous/Next" buttons
- Page number link lists
- Text link pagination

#### Configuration Method

**Basic Configuration**:
```json
{
  "type": "click_next",
  "config": {
    "selector": ".pagination .next"
  }
}
```

**Complete Configuration**:
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

#### Configuration Parameters

| Parameter | Type | Description | Default |
|-----------|------|-------------|---------|
| selector | string | Next page button selector | Required |
| maxPages | number | Maximum pages to paginate | Unlimited |
| interval | number | Pagination interval (ms) | 2000 |
| waitAfterClick | number | Wait time after click (ms) | 3000 |
| stopWhenDisabled | boolean | Stop when button disabled | true |

#### Common Selector Examples

**Numeric Pagination**:
```css
/* Next page button */
.pagination .next
.page-next
a[rel="next"]
button.next-page

/* Page number links */
.pagination a:last-child
.page-item:last-child a
```

**Text Links**:
```css
/* Contains "Next" text */
a:contains("Next")
a:contains("»")

/* Using XPath */
//a[text()='Next']
//a[contains(text(), 'Next')]
```

#### Stop Conditions

Pagination will stop in the following situations:

1. **Reached Maximum Pages**: Pagination count reaches maxPages setting
2. **Button Not Found**: Cannot find next page button
3. **Button Disabled**: Button is disabled
4. **No New Data**: Current page has no new data collected
5. **Manual Stop**: User manually stops task

### 2. Infinite Scroll (scroll)

Triggers content loading by scrolling the page, common on social media and feed websites.

#### Applicable Scenarios

- Social media feeds (Twitter, Weibo)
- Waterfall layouts (Pinterest)
- News and information websites
- Video website lists

#### Configuration Method

**Basic Configuration**:
```json
{
  "type": "scroll",
  "config": {
    "selector": "body"
  }
}
```

**Complete Configuration**:
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

#### Configuration Parameters

| Parameter | Type | Description | Default |
|-----------|------|-------------|---------|
| selector | string | Scroll container selector | body |
| maxScrolls | number | Maximum scroll count | Unlimited |
| interval | number | Scroll interval (ms) | 1000 |
| scrollDistance | number | Scroll distance per scroll (px) | 1000 |
| scrollToBottom | boolean | Whether to scroll to bottom | true |
| waitForNewContent | number | Wait for new content (ms) | 3000 |
| stopWhenNoNewContent | boolean | Stop when no new content | true |

#### Scroll Strategies

**Scroll to Bottom**:
```javascript
// Scroll to page bottom each time
window.scrollTo(0, document.body.scrollHeight)
```

**Fixed Distance Scroll**:
```javascript
// Scroll down 1000 pixels each time
window.scrollBy(0, 1000)
```

**Smooth Scroll**:
```javascript
// Smooth scroll effect
window.scrollTo({
  top: document.body.scrollHeight,
  behavior: 'smooth'
})
```

#### Detect New Content

The plugin automatically detects if the page has loaded new content:

1. Record element count before scrolling
2. Execute scroll operation
3. Wait for new content to load
4. Compare element count
5. If no new elements, stop scrolling

### 3. Load More (load_more)

Loads new content by clicking "Load More" or "View More" buttons.

#### Applicable Scenarios

- Comment lists
- Product recommendations
- Search results
- Article lists

#### Configuration Method

**Basic Configuration**:
```json
{
  "type": "load_more",
  "config": {
    "selector": ".load-more-btn"
  }
}
```

**Complete Configuration**:
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

#### Configuration Parameters

| Parameter | Type | Description | Default |
|-----------|------|-------------|---------|
| selector | string | Load more button selector | Required |
| maxClicks | number | Maximum click count | Unlimited |
| interval | number | Click interval (ms) | 1500 |
| waitAfterClick | number | Wait time after click (ms) | 2000 |
| stopWhenHidden | boolean | Stop when button hidden | true |
| stopWhenDisabled | boolean | Stop when button disabled | true |

#### Common Selectors

```css
/* Buttons */
button.load-more
.load-more-btn
#loadMore

/* Links */
a.more-link
.view-more

/* Contains specific text */
button:contains("Load More")
a:contains("View More")
```

### 4. URL Parameter Pagination

Achieves pagination by modifying page number parameters in URL.

#### Applicable Scenarios

- URLs containing page parameters
- RESTful style pagination
- API interface pagination

#### Configuration Method

**Basic Configuration**:
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

**Complete Configuration**:
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

#### URL Pattern Examples

**Query Parameters**:
```
https://example.com/products?page=1
https://example.com/products?page=2
https://example.com/products?page=3
```

**Path Parameters**:
```
https://example.com/products/page/1
https://example.com/products/page/2
https://example.com/products/page/3
```

**RESTful Style**:
```
https://api.example.com/v1/products?offset=0&limit=20
https://api.example.com/v1/products?offset=20&limit=20
https://api.example.com/v1/products?offset=40&limit=20
```

## Advanced Pagination Configuration

### Combined Pagination

Some websites may require combining multiple pagination methods.

**Example: Scroll then Click**:
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

### Conditional Pagination

Decide whether to continue pagination based on conditions.

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

### Smart Wait

Intelligently adjust wait time based on page loading conditions.

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

### Error Retry

Automatically retry when pagination fails.

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

## Pagination Performance Optimization

### 1. Set Reasonable Intervals

```json
{
  "interval": 2000,  // 2 second interval
  "randomDelay": {
    "enabled": true,
    "min": 1000,
    "max": 3000
  }
}
```

**Recommendations**:
- Regular websites: 2-3 seconds
- Large websites: 3-5 seconds
- Strictly restricted websites: 5-10 seconds

### 2. Concurrency Control

```json
{
  "concurrency": {
    "enabled": false,  // Pagination not recommended for concurrency
    "maxConcurrent": 1
  }
}
```

### 3. Caching Strategy

```json
{
  "cache": {
    "enabled": true,
    "skipVisitedPages": true,
    "cacheExpiry": 86400
  }
}
```

### 4. Incremental Pagination

Only paginate to pages with new data.

```json
{
  "incremental": {
    "enabled": true,
    "compareField": "id",
    "stopWhenDuplicate": true
  }
}
```

## Common Issues

### Issue 1: Cannot find pagination button

**Causes**:
- Incorrect selector
- Button dynamically loaded
- Button in iframe

**Solutions**:
```javascript
// 1. Check selector
document.querySelector('.next-page')

// 2. Wait for button to appear
await page.waitForSelector('.next-page')

// 3. Check iframe
const frame = page.frames().find(f => f.name() === 'content')
await frame.waitForSelector('.next-page')
```

### Issue 2: No pagination after click

**Causes**:
- Click event intercepted
- Insufficient wait time
- Page uses JavaScript navigation

**Solutions**:
```json
{
  "config": {
    "selector": ".next",
    "clickMethod": "javascript",  // Use JS click
    "waitAfterClick": 5000,       // Increase wait time
    "waitForNavigation": true     // Wait for page navigation
  }
}
```

### Issue 3: Infinite scroll not loading new content

**Causes**:
- Scrolling too fast
- Need to scroll to specific position
- Website detects automation

**Solutions**:
```json
{
  "config": {
    "scrollDistance": 500,        // Reduce scroll distance
    "interval": 2000,             // Increase interval
    "waitForNewContent": 5000,    // Increase wait time
    "scrollBehavior": "smooth"    // Use smooth scroll
  }
}
```

### Issue 4: Duplicate pagination data

**Causes**:
- Not correctly identifying new data
- Duplicate page content
- Incorrect deduplication configuration

**Solutions**:
```json
{
  "deduplication": {
    "enabled": true,
    "fields": ["id", "url"],
    "scope": "global"
  }
}
```

## Pagination Debugging

### Debug Mode

Enable debug mode to view pagination process.

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

### Log Output

```
[Pagination] Starting pagination, currently on page 1
[Pagination] Found next page button: .pagination .next
[Pagination] Clicking next page button
[Pagination] Waiting for page to load...
[Pagination] Page loading completed, currently on page 2
[Pagination] Collected 50 new records
[Pagination] Continuing pagination...
```

### Performance Monitoring

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

## Best Practices

### 1. Choose Appropriate Pagination Type

✅ **Recommended**:
- Choose based on actual website conditions
- Prioritize simple methods
- Test to confirm pagination works

❌ **Avoid**:
- Blindly using complex configurations
- Going live without testing
- Ignoring website characteristics

### 2. Set Reasonable Limits

✅ **Recommended**:
- Set maximum pages
- Set maximum data count
- Set timeout duration

❌ **Avoid**:
- Unlimited pagination
- Not setting stop conditions
- Ignoring performance impact

### 3. Control Pagination Speed

✅ **Recommended**:
- Set reasonable intervals
- Add random delays
- Simulate human operations

❌ **Avoid**:
- Paginating too fast
- Fixed intervals
- Obviously robotic behavior

### 4. Handle Exceptions

✅ **Recommended**:
- Configure error retry
- Record pagination logs
- Monitor pagination status

❌ **Avoid**:
- Not handling errors
- Not recording logs
- Ignoring exceptions

## Practical Cases

### Case 1: E-commerce Product List

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

### Case 2: Social Media Feed

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

### Case 3: News Website

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

## Next Steps

- [Element Selectors](./selectors) - Learn how to write selectors
- [Data Collection](./data-collection) - Configure data extraction rules
- [Batch Collection](./batch-collection) - Batch collect multiple targets
- [Tutorials](./tutorials) - Complete collection cases