# Intelligent Collection

Intelligent Collection is one of the core features of AutoWDS. Based on AI technology, it automatically identifies web page structure and extracts data, making data collection unprecedentedly simple.

## What is Intelligent Collection?

Intelligent Collection uses machine learning algorithms to automatically analyze the DOM structure of web pages, identify collectible data fields, without manually configuring complex selectors and rules. Simply right-click on the page and select "Intelligent Collection", and the plugin will automatically complete data extraction.

### Core Advantages

- **Zero Configuration**: No need to write selectors or rules
- **Quick Start**: Start collecting in seconds
- **Smart Recognition**: Automatically identifies common fields like titles, prices, images
- **Adaptive**: Adapts to different website page structures
- **Adjustable**: AI-generated rules can be manually optimized

## Using Intelligent Collection

### Method 1: Context Menu

1. **Open Target Page**
   - Visit the page you want to collect data from in your browser
   - For example: Product list page on e-commerce website

2. **Start Intelligent Collection**
   - Right-click on blank area of the page
   - Select "Intelligent Collection" menu item
   - Or use shortcut `Ctrl+Shift+I` (Windows) / `Cmd+Shift+I` (Mac)

3. **AI Analyzes Page**
   - Plugin automatically analyzes page structure
   - Identifies collectible data fields
   - Generates collection rules

4. **Confirm Fields**
   - Review AI-identified field list
   - Can add, delete or modify fields
   - Adjust field names and types

5. **Start Collection**
   - Click "Start Collection" button
   - View collection progress in real-time
   - View data after collection completes

### Method 2: Plugin Panel

1. **Open Plugin**
   - Click plugin icon in browser toolbar
   - Enter main interface

2. **Create Intelligent Task**
   - Click "New Task"
   - Select "Intelligent Collection Mode"
   - Enter target page URL

3. **Configure and Execute**
   - Plugin automatically opens target page
   - AI analyzes and generates rules
   - Start collection after confirmation

## Intelligently Recognized Field Types

Intelligent Collection can automatically recognize the following common field types:

### Text Fields

**Title Types**
- Product titles
- Article titles
- News headlines
- Product names

**Description Types**
- Product descriptions
- Article summaries
- Introduction information
- Detailed descriptions

**Category Tags**
- Product categories
- Article tags
- Keywords
- Brand information

### Numeric Fields

**Price Types**
- Current price
- Original price
- Discount price
- Member price

**Quantity Types**
- Stock quantity
- Sales volume
- Comment count
- View count

**Rating Types**
- Product rating
- User rating
- Star rating

### Link Fields

**Page Links**
- Detail page links
- Product links
- Article links
- Download links

**Image Links**
- Main image
- Thumbnail
- Detail images
- Carousel images

### Time Fields

**Publication Time**
- Article publication time
- Product listing time
- Comment time
- Update time

**Format Recognition**
- Relative time (e.g., "2 hours ago")
- Absolute time (e.g., "2024-01-01")
- Automatic conversion to standard format

### Structured Data

**List Data**
- Product specifications
- Attribute parameters
- Tag lists
- Option lists

**Table Data**
- Parameter tables
- Price tables
- Comparison tables

## Intelligent Collection Examples

### Example 1: E-commerce Product List

**Target Page**: E-commerce website product list page

**AI Auto-identified Fields**:
```
✓ Product Title (title)
✓ Price (price)
✓ Original Price (originalPrice)
✓ Product Image (image)
✓ Detail Link (detailUrl)
✓ Sales (sales)
✓ Rating (rating)
✓ Shop Name (shopName)
```

**Collection Result**:
```json
[
  {
    "title": "Apple iPhone 15 Pro Max",
    "price": "9999",
    "originalPrice": "10999",
    "image": "https://example.com/img/iphone.jpg",
    "detailUrl": "https://example.com/product/123",
    "sales": "10000+",
    "rating": "4.9",
    "shopName": "Apple Official Store"
  }
]
```

### Example 2: News Article List

**Target Page**: News website homepage

**AI Auto-identified Fields**:
```
✓ Article Title (title)
✓ Summary (summary)
✓ Author (author)
✓ Publish Time (publishTime)
✓ Article Link (articleUrl)
✓ Cover Image (coverImage)
✓ Category (category)
✓ Views (views)
```

**Collection Result**:
```json
[
  {
    "title": "Tech News Title",
    "summary": "This is article summary content...",
    "author": "John Doe",
    "publishTime": "2024-01-15 10:30:00",
    "articleUrl": "https://news.example.com/article/456",
    "coverImage": "https://news.example.com/img/cover.jpg",
    "category": "Technology",
    "views": "12K"
  }
]
```

### Example 3: Job Postings

**Target Page**: Job website position list

**AI Auto-identified Fields**:
```
✓ Job Title (jobTitle)
✓ Company Name (companyName)
✓ Salary Range (salary)
✓ Location (location)
✓ Experience (experience)
✓ Education (education)
✓ Job Link (jobUrl)
✓ Post Time (postTime)
```

## Adjusting Intelligent Collection Results

While AI can automatically identify most fields, sometimes manual adjustment is needed for better results.

### Adding Fields

If AI missed some fields:

1. Click "Add Field" button
2. Click element to collect on page
3. Enter field name
4. Select data type
5. Save field configuration

### Deleting Fields

If AI identified unwanted fields:

1. Find the field in field list
2. Click delete icon on right side of field
3. Confirm deletion

### Modifying Fields

Adjust field configuration:

1. Click field to enter edit mode
2. Modify field name
3. Adjust selector (if needed)
4. Change data type
5. Configure data processing rules
6. Save modifications

### Field Renaming

```
Original field name: price_text
↓ Change to
New field name: price
```

### Data Type Conversion

```
Original type: Text "¥199.99"
↓ Convert to
New type: Number 199.99
```

## Intelligent Pagination

Intelligent Collection not only recognizes data fields but also automatically identifies pagination methods.

### Auto-identify Pagination Types

**Numeric Pagination**
```html
<div class="pagination">
  <a>1</a>
  <a>2</a>
  <a class="next">Next</a>
</div>
```
AI identifies: Click "Next" button

**Infinite Scroll**
```javascript
// Auto-load when scrolling to bottom
window.addEventListener('scroll', loadMore)
```
AI identifies: Scroll to bottom trigger

**Load More Button**
```html
<button class="load-more">Load More</button>
```
AI identifies: Click "Load More" button

### Configure Pagination Parameters

**Maximum Pages**
```
Setting: Collect first 10 pages
Result: Automatically stops at page 10
```

**Maximum Data Count**
```
Setting: Collect 1000 records
Result: Stops after reaching 1000 records
```

**Pagination Interval**
```
Setting: 2 seconds interval per page
Purpose: Avoid being blocked by too fast requests
```

## Intelligent Deep Collection

For scenarios requiring entering detail pages to collect more information, Intelligent Collection can also handle automatically.

### Auto-identify Detail Links

AI automatically identifies detail page links in lists:

```
List page identification:
✓ Product title links
✓ "View Details" buttons
✓ Product image links
```

### Detail Page Field Recognition

After entering detail page, AI continues to identify more fields:

```
Additional detail page fields:
✓ Detailed description
✓ Product parameters
✓ User reviews
✓ More images
```

### Data Merging

List page and detail page data automatically merged:

```json
{
  // List page data
  "title": "Product Title",
  "price": "199.99",
  
  // Detail page data
  "description": "Detailed description...",
  "specifications": {
    "brand": "Brand Name",
    "model": "Model Number"
  }
}
```

## Intelligent Data Cleaning

AI not only identifies data but also automatically cleans it.

### Price Cleaning

```
Raw data: "¥199.99元"
Cleaned: "199.99"

Raw data: "$1,999.00"
Cleaned: "1999.00"
```

### Number Cleaning

```
Raw data: "Sales: 10000+"
Cleaned: "10000"

Raw data: "12K"
Cleaned: "12000"
```

### Time Cleaning

```
Raw data: "2 hours ago"
Cleaned: "2024-01-15 14:30:00"

Raw data: "Yesterday 15:30"
Cleaned: "2024-01-14 15:30:00"
```

### Text Cleaning

```
Raw data: "  Title text  \n\n"
Cleaned: "Title text"

Raw data: "Title<span>tag</span>"
Cleaned: "Titletag"
```

## Limitations of Intelligent Collection

While Intelligent Collection is powerful, it has some limitations:

### Applicable Scenarios

✅ **Suitable**:
- Structured list pages
- Standard detail pages
- Common e-commerce, news websites
- Standard HTML structure

❌ **Not Suitable**:
- Highly customized pages
- Complex single-page applications (SPA)
- Pages requiring complex interactions
- Non-standard data structures

### Recognition Accuracy

- **High Accuracy** (>90%): E-commerce products, news articles, job postings
- **Medium Accuracy** (70-90%): Social media, forum posts
- **Low Accuracy** (<70%): Highly customized pages, complex tables

### Performance Considerations

- AI analysis takes a few seconds
- Complex pages may take longer
- Recommended to test on small scale first

## From Intelligent Collection to Visual Rules

Rules generated by Intelligent Collection can be converted to visual rules for finer adjustments:

1. **Complete Intelligent Collection**
   - Use Intelligent Collection to create initial rules

2. **Convert to Visual Rules**
   - Click "Convert to Advanced Rules"
   - Enter visual editor

3. **Fine Adjustments**
   - Modify selectors
   - Add data processing logic
   - Configure complex pagination rules
   - Add conditional judgments

4. **Save and Reuse**
   - Save as rule template
   - Use for similar website collection

## Best Practices

### 1. Choose Appropriate Pages

✅ **Recommended**:
- Choose pages with complete data
- Ensure page is fully loaded
- Avoid popup and ad interference

### 2. Validate Collection Results

✅ **Recommended**:
- Test with small amount of data first
- Check if fields are correct
- Verify data completeness
- Confirm data format

### 3. Handle Special Cases

✅ **Recommended**:
- For login pages, login manually first
- For dynamic loading, wait for content to load
- For CAPTCHAs, complete verification manually

### 4. Optimize Collection Performance

✅ **Recommended**:
- Only collect necessary fields
- Set reasonable pagination intervals
- Use incremental collection to avoid duplicates
- Clean historical data regularly

## Troubleshooting

### Issue 1: AI Cannot Identify Fields

**Possible Causes**:
- Non-standard page structure
- Dynamic content loading
- Hidden elements

**Solutions**:
1. Wait for page to fully load
2. Scroll page to trigger content loading
3. Manually add missing fields
4. Convert to visual rules for manual configuration

### Issue 2: Identified Fields Inaccurate

**Possible Causes**:
- Page contains ads or recommended content
- Mixed data structures
- Non-standard field naming

**Solutions**:
1. Delete unwanted fields
2. Modify field selectors
3. Adjust data extraction rules
4. Use data filtering

### Issue 3: Incomplete Collected Data

**Possible Causes**:
- Pagination identification error
- Dynamic data loading
- Network request failure

**Solutions**:
1. Check pagination configuration
2. Increase wait time
3. Manually configure pagination rules
4. Check error logs

## Next Steps

- [Visual Rule Development](./visual-rule-development) - Create more complex collection rules
- [Data Export](./data-export) - Learn how to export collected data
- [Scheduled Tasks](./scheduled-tasks) - Set up automated collection tasks
- [Tutorials](./tutorials) - Learn data collection through examples
