# Visual Rule Development

Visual rule development is an advanced feature of AutoWDS that creates complex data collection rules through a graphical interface, supporting multi-level deep collection, conditional judgment, data processing and other advanced features.

## What are Visual Rules?

Visual rules use flowcharts to display collection logic, where each node represents an operation step, and nodes are connected by lines to indicate execution order. This approach makes complex collection processes intuitive and easy to understand.

### Differences from Intelligent Collection

| Feature | Intelligent Collection | Visual Rules |
|---------|----------------------|--------------|
| Creation Method | AI auto-generated | Manual configuration |
| Applicable Scenarios | Standardized pages | Complex scenarios |
| Flexibility | Medium | Very high |
| Learning Curve | Low | Medium |
| Customization Level | Limited | Fully customizable |

## Visual Editor Interface

### Interface Layout

```
┌─────────────────────────────────────────────────────┐
│  Toolbar: [Save] [Run] [Debug] [Export]            │
├──────────┬──────────────────────────────────────────┤
│          │                                          │
│  Node    │          Canvas Area                     │
│  Library │      (Drag nodes to create flow)        │
│          │                                          │
│  ├Start  │                                          │
│  ├Page   │                                          │
│  ├List   │                                          │
│  └Detail │                                          │
│          │                                          │
├──────────┴──────────────────────────────────────────┤
│  Properties Panel: Node configuration and fields    │
└─────────────────────────────────────────────────────┘
```

### Main Components

**1. Toolbar**
- Save rules
- Run tests
- Debug mode
- Export rules

**2. Node Library**
- Start node
- Page node
- List node
- Detail node

**3. Canvas Area**
- Drag to create nodes
- Connect nodes
- Adjust layout
- Zoom view

**4. Properties Panel**
- Node configuration
- Field settings
- Data processing
- Advanced options

## Creating Visual Rules

### Step 1: Create New Rule

1. Open plugin main interface
2. Click "New Task"
3. Select "Advanced Collection Mode"
4. Enter visual editor

### Step 2: Add Start Node

The start node is the entry point of the rule and must be configured first.

**Configuration:**

```yaml
URL: https://example.com/products
Viewport:
  Width: 1920
  Height: 1080
HTTP Headers:
  - User-Agent: Mozilla/5.0...
  - Accept-Language: en-US
Initial Steps:
  - Wait for page load
  - Close popup ads
```

**Example Configuration:**

1. Start node exists automatically in canvas
2. Click start node
3. Configure in properties panel:
   - Enter target URL
   - Set browser window size
   - Add custom HTTP headers (optional)
   - Configure initialization steps (optional)

### Step 3: Add Data Collection Nodes

Add appropriate nodes based on collection requirements.

#### Scenario 1: Simple List Collection

**Goal**: Collect product information from product list page

**Flow**:
```
Start Node → List Node
```

**List Node Configuration**:

1. **List Selector**
   ```css
   .product-list .product-item
   ```

2. **Field Configuration**
   ```json
   [
     {
       "name": "title",
       "selector": ".title",
       "attr": "innerText"
     },
     {
       "name": "price",
       "selector": ".price",
       "attr": "innerText",
       "extractor": {
         "type": "regex",
         "code": "\\d+\\.\\d+"
       }
     },
     {
       "name": "image",
       "selector": "img",
       "attr": "src"
     }
   ]
   ```

3. **Pagination Configuration**
   ```json
   {
     "type": "click_next",
     "config": {
       "selector": ".next-page"
     }
   }
   ```

#### Scenario 2: List + Detail Deep Collection

**Goal**: First collect list, then enter each detail page to collect more information

**Flow**:
```
Start Node → List Node → Page Node → Detail Node
```

**Configuration Steps**:

1. **List Node**: Collect basic information and detail links
   ```json
   {
     "fields": [
       {
         "name": "title",
         "selector": ".title",
         "attr": "innerText"
       },
       {
         "name": "detailUrl",
         "selector": "a.detail-link",
         "attr": "href"
       }
     ]
   }
   ```

2. **Page Node**: Open detail page
   ```json
   {
     "type": "click_element",
     "value": "a.detail-link"
   }
   ```

3. **Detail Node**: Collect detailed information
   ```json
   {
     "fields": [
       {
         "name": "description",
         "selector": ".description",
         "attr": "innerText"
       },
       {
         "name": "specifications",
         "selector": ".specs",
         "attr": "innerText"
       }
     ]
   }
   ```

#### Scenario 3: Multi-level Category Collection

**Goal**: Traverse all categories, collect products under each category

**Flow**:
```
Start Node → List Node(Categories) → Page Node → List Node(Products)
```

**Configuration Description**:

1. **First List Node**: Collect all category links
2. **Page Node**: Enter each category page
3. **Second List Node**: Collect products under that category

### Step 4: Configure Fields

Configure fields to extract for each collection node.

#### Adding Fields

**Method 1: Click to Add**
1. Click "Add Field" button
2. Click target element in page preview
3. System automatically generates selector
4. Enter field name
5. Select extraction attribute

**Method 2: Manual Add**
1. Click "Add Field" button
2. Manually enter field name
3. Enter CSS selector or XPath
4. Select extraction attribute
5. Configure data processing (optional)

#### Field Configuration Options

**Basic Configuration**:
```json
{
  "name": "price",           // Field name
  "selector": ".price",      // Element selector
  "attr": "innerText",       // Extraction attribute
  "required": true,          // Is required
  "defaultValue": "0"        // Default value
}
```

**Advanced Configuration**:
```json
{
  "name": "price",
  "selector": ".price",
  "attr": "innerText",
  "extractor": {
    "type": "regex",         // Extractor type
    "code": "\\d+\\.\\d+"    // Extraction rule
  },
  "transformer": {
    "type": "js",            // Transformer type
    "code": "return parseFloat(value)"
  }
}
```

### Step 5: Configure Pagination

Configure pagination rules for list nodes.

#### Click Next Page

```json
{
  "type": "click_next",
  "config": {
    "selector": ".pagination .next",
    "maxPages": 10,
    "interval": 2000
  }
}
```

#### Infinite Scroll

```json
{
  "type": "scroll",
  "config": {
    "selector": "body",
    "maxScrolls": 20,
    "interval": 1000
  }
}
```

#### Load More

```json
{
  "type": "load_more",
  "config": {
    "selector": ".load-more-btn",
    "maxClicks": 10,
    "interval": 1500
  }
}
```

### Step 6: Add Operation Steps

Add automated operation steps in nodes.

#### Common Steps

**Click Operation**:
```json
{
  "type": "click",
  "selectors": [["button.close-ad"]]
}
```

**Input Text**:
```json
{
  "type": "change",
  "selectors": [["input.search"]],
  "value": "search keyword"
}
```

**Wait for Loading**:
```json
{
  "type": "wait",
  "timeout": 3000
}
```

**Scroll Page**:
```json
{
  "type": "scroll",
  "x": 0,
  "y": 1000
}
```

#### Step Combination Example

**Login Operation**:
```json
{
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
      "timeout": 2000
    }
  ]
}
```

**Search Operation**:
```json
{
  "steps": [
    {
      "type": "change",
      "selectors": [["input.search-box"]],
      "value": "iPhone 15"
    },
    {
      "type": "keyDown",
      "key": "Enter"
    },
    {
      "type": "wait",
      "timeout": 2000
    }
  ]
}
```

### Step 7: Test and Debug

Test to ensure rules are correct before saving.

#### Run Test

1. Click "Run" button in toolbar
2. Observe collection process
3. View collection results
4. Check data completeness

#### Debug Mode

Enabling debug mode allows:
- Step-by-step node execution
- View intermediate data
- Check selector matching
- View detailed logs

#### Common Issue Troubleshooting

**Issue 1: Selector cannot find element**
- Check if selector is correct
- Confirm page has loaded completely
- Try using simpler selector

**Issue 2: Data extraction incomplete**
- Check field configuration
- Confirm element attributes are correct
- Check if waiting for loading is needed

**Issue 3: Pagination not working**
- Check pagination selector
- Confirm pagination type is correct
- Check for anti-scraping restrictions

### Step 8: Save and Run

Save rule and run after testing passes.

1. Click "Save" button
2. Enter rule name and description
3. Select save location (local/cloud)
4. Click "Run" to start collection

## Advanced Techniques

### 1. Using Variables

Use variables in rules for dynamic configuration.

**Define Variables**:
```json
{
  "variables": {
    "keyword": "iPhone",
    "maxPages": 10,
    "category": "phones"
  }
}
```

**Use Variables**:
```json
{
  "url": "https://example.com/search?q={{keyword}}",
  "pagination": {
    "maxPages": "{{maxPages}}"
  }
}
```

### 2. Conditional Judgment

Execute different collection logic based on conditions.

```json
{
  "condition": {
    "field": "price",
    "operator": ">",
    "value": 1000
  },
  "then": {
    // Operations when price > 1000
  },
  "else": {
    // Operations when price <= 1000
  }
}
```

### 3. Data Transformation

Transform collected data.

**Price Conversion**:
```javascript
// Input: "¥1,999.00"
// Output: 1999.00
return parseFloat(value.replace(/[^0-9.]/g, ''))
```

**Date Conversion**:
```javascript
// Input: "2 hours ago"
// Output: "2024-01-15 14:30:00"
const now = new Date()
const hours = parseInt(value)
now.setHours(now.getHours() - hours)
return now.toISOString()
```

**Text Cleaning**:
```javascript
// Remove extra whitespace and newlines
return value.trim().replace(/\s+/g, ' ')
```

### 4. Loop Collection

Execute same operations for each item in list.

```json
{
  "loop": {
    "selector": ".product-item",
    "actions": [
      {
        "type": "click",
        "selector": ".detail-btn"
      },
      {
        "type": "extract",
        "fields": [...]
      },
      {
        "type": "back"
      }
    ]
  }
}
```

### 5. Error Handling

Configure error handling strategies.

```json
{
  "errorHandling": {
    "onSelectorNotFound": "skip",     // Skip
    "onTimeout": "retry",             // Retry
    "onNetworkError": "abort",        // Abort
    "maxRetries": 3,
    "retryInterval": 5000
  }
}
```

## Practical Cases

### Case 1: E-commerce Product Collection

**Requirement**: Collect mobile phone product information from e-commerce website

**Rule Design**:

```
Start Node(Search Page) 
  → List Node(Product List) 
  → Page Node(Detail Page) 
  → Detail Node(Detailed Info)
```

**Configuration Points**:
1. Start node configures search URL
2. List node collects product title, price, link
3. Page node opens detail page
4. Detail node collects detailed parameters, reviews, etc.
5. Configure pagination for multi-page data

### Case 2: News Article Collection

**Requirement**: Collect latest articles from news website

**Rule Design**:

```
Start Node(Homepage) 
  → List Node(Article List) 
  → Page Node(Article Page) 
  → Detail Node(Article Content)
```

**Configuration Points**:
1. List node collects article title, summary, link
2. Detail node collects complete text, author, time
3. Handle relative time conversion
4. Clean HTML tags

### Case 3: Job Information Collection

**Requirement**: Collect job postings from recruitment website

**Rule Design**:

```
Start Node(Search Page) 
  → List Node(Job List) 
  → Page Node(Job Details) 
  → Detail Node(Detailed Requirements)
```

**Configuration Points**:
1. Configure search keywords and location
2. Collect job title, company, salary
3. Collect detailed job description and requirements
4. Process salary range data

## Best Practices

### 1. Rule Design Principles

✅ **Recommended**:
- Keep rules simple and clear
- Reasonably divide node responsibilities
- Use meaningful field names
- Add comments and explanations

❌ **Avoid**:
- Overly complex nesting
- Duplicate configurations
- Vague naming
- Lack of error handling

### 2. Selector Writing

✅ **Recommended**:
- Use stable selectors
- Prioritize ID and semantic classes
- Appropriate hierarchy depth
- Test selector uniqueness

❌ **Avoid**:
- Dynamically generated classes
- Too deep hierarchy nesting
- Position-dependent elements
- Unstable attributes

### 3. Performance Optimization

✅ **Recommended**:
- Only collect necessary fields
- Set reasonable wait times
- Use incremental collection
- Control concurrency count

❌ **Avoid**:
- Collect large amounts of useless data
- Too short request intervals
- Repeatedly collect same data
- Unlimited pagination

### 4. Data Quality

✅ **Recommended**:
- Validate data completeness
- Clean and format data
- Set default values
- Record collection logs

❌ **Avoid**:
- Don't validate data
- Keep raw dirty data
- Ignore outliers
- Don't record errors

## Next Steps

- [Data Export](./data-export) - Learn how to export collected data
- [Scheduled Tasks](./scheduled-tasks) - Set up automated collection
- [Batch Collection](./batch-collection) - Batch collect multiple targets
- [Tutorials](./tutorials) - Complete data collection and export cases