# Data Export

Data export is the functionality to save collected data as different format files or sync to external services. AutoWDS supports multiple export formats and targets to meet different data usage needs.

## Supported Export Formats

### Excel (.xlsx)

The most commonly used data export format, suitable for data analysis and report creation.

**Features**:
- Supports multiple worksheets
- Preserves data format and styles
- Supports formulas and charts
- Good compatibility

**Use Cases**:
- Data analysis and statistics
- Creating reports
- Data sharing and presentation
- Import to other systems

**Export Example**:
```
Filename: products_2024-01-15.xlsx
Worksheet: Sheet1
Data rows: 1000
Fields: Title, Price, Image, Link, Rating
```

### CSV (.csv)

Universal text format, suitable for large data volumes and program processing.

**Features**:
- Small file size
- Fast loading speed
- Excellent compatibility
- Easy for program processing

**Use Cases**:
- Large data volume export
- Database import
- Program batch processing
- Cross-platform usage

**Encoding Options**:
- UTF-8 (recommended)
- GBK (compatible with old Excel versions)
- UTF-8 with BOM

**Export Example**:
```csv
Title,Price,Image,Link,Rating
"iPhone 15 Pro",9999,"https://...","/product/123",4.9
"Samsung S24",8999,"https://...","/product/456",4.8
```

### JSON (.json)

Structured data format, suitable for API integration and program processing.

**Features**:
- Clear structure
- Supports nested data
- Easy to parse
- Suitable for API transmission

**Use Cases**:
- API data integration
- Frontend application usage
- Data backup
- Inter-program data exchange

**Format Options**:
- Standard JSON
- JSON Lines (one JSON object per line)
- Pretty format (with indentation)
- Compressed format (no spaces)

**Export Example**:
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

Direct export to Google Sheets, supports cloud collaboration.

**Features**:
- Real-time sync
- Multi-user collaboration
- Automatic backup
- Online access

**Use Cases**:
- Team collaboration
- Real-time data monitoring
- Remote access
- Data sharing

## Export Operations

### Method 1: Export After Task Completion

1. **Collection Complete**
   - Wait for task execution to complete
   - View collected data preview

2. **Select Export**
   - Click "Export" button
   - Select export format

3. **Configure Options**
   - Select fields to export
   - Set filename
   - Choose save location

4. **Start Export**
   - Click "Confirm" button
   - Wait for export completion
   - Open exported file

### Method 2: Export from Data Management

1. **Open Data Management**
   - Click plugin icon
   - Enter "Data Management" module

2. **Select Data**
   - Browse collected data
   - Select dataset to export
   - Can filter and sort

3. **Export Data**
   - Click "Export" button
   - Select format and options
   - Complete export

### Method 3: Auto Export

Configure task to automatically export collection results.

1. **Edit Task Settings**
   - Open task configuration
   - Enter "Export Settings"

2. **Configure Auto Export**
   ```json
   {
     "autoExport": true,
     "format": "xlsx",
     "filename": "products_{{date}}",
     "location": "~/Downloads/crawler-data/"
   }
   ```

3. **Save Settings**
   - Auto export after task completion
   - No manual operation needed

## Export Configuration

### Field Selection

Select fields and order to export.

**All Fields**:
```
✓ Title
✓ Price
✓ Image
✓ Link
✓ Rating
✓ Sales
✓ Shop
```

**Custom Fields**:
```
✓ Title
✓ Price
✓ Link
✗ Image (not exported)
✗ Rating (not exported)
```

**Field Renaming**:
```
Original field name → Export field name
title → Product Title
price → Price(USD)
rating → User Rating
```

### Data Filtering

Filter data before export.

**Filter by Conditions**:
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

**Filter by Time**:
```json
{
  "dateRange": {
    "field": "collectTime",
    "start": "2024-01-01",
    "end": "2024-01-31"
  }
}
```

**Deduplication**:
```json
{
  "deduplication": {
    "enabled": true,
    "fields": ["title", "link"],
    "keepFirst": true
  }
}
```

### Data Sorting

Set sorting method for exported data.

**Single Field Sort**:
```json
{
  "sort": {
    "field": "price",
    "order": "desc"
  }
}
```

**Multi-field Sort**:
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

### File Naming

Customize export filename.

**Using Variables**:
```
Template: products_{{date}}_{{time}}
Result: products_2024-01-15_143000.xlsx

Template: {{taskName}}_{{count}}records
Result: ProductCollection_1000records.xlsx
```

**Available Variables**:
- `{{date}}` - Date (YYYY-MM-DD)
- `{{time}}` - Time (HHMMSS)
- `{{datetime}}` - Date and time
- `{{taskName}}` - Task name
- `{{count}}` - Data count
- `{{timestamp}}` - Timestamp

## Export to Google Sheets

### Initial Setup

1. **Authorize Google Account**
   - Click "Connect Google Account"
   - Login to Google account
   - Authorize access permissions

2. **Select Worksheet**
   - Select existing worksheet
   - Or create new worksheet

3. **Configure Mapping**
   - Map fields to table columns
   - Set header names

### Export Modes

**Append Mode**:
```
Existing data: 100 rows
New data: 50 rows
Result: 150 rows (new data appended to end)
```

**Overwrite Mode**:
```
Existing data: 100 rows
New data: 50 rows
Result: 50 rows (clear and write new data)
```

**Update Mode**:
```
Update existing data based on key fields
New data updates matching rows
Non-matching rows appended to end
```

### Real-time Sync

Configure real-time sync to Google Sheets.

```json
{
  "googleSheets": {
    "enabled": true,
    "spreadsheetId": "1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgvE2upms",
    "sheetName": "Product Data",
    "mode": "append",
    "syncInterval": 60
  }
}
```

**Sync Options**:
- Real-time sync (sync one record at a time)
- Batch sync (sync every N records)
- Scheduled sync (sync every N minutes)

## Advanced Export Features

### Batch Export

Export large data volumes in batches.

**Batch by Count**:
```json
{
  "batchExport": {
    "enabled": true,
    "batchSize": 1000,
    "fileNamePattern": "products_batch_{{batchNumber}}.xlsx"
  }
}
```

**Result**:
```
products_batch_1.xlsx (1-1000)
products_batch_2.xlsx (1001-2000)
products_batch_3.xlsx (2001-3000)
```

**Batch by Date**:
```json
{
  "batchExport": {
    "enabled": true,
    "groupBy": "date",
    "dateField": "collectTime"
  }
}
```

**Result**:
```
products_2024-01-15.xlsx
products_2024-01-16.xlsx
products_2024-01-17.xlsx
```

### Multi-format Export

Export multiple formats simultaneously.

```json
{
  "multiFormat": {
    "enabled": true,
    "formats": ["xlsx", "csv", "json"]
  }
}
```

**Result**:
```
products_2024-01-15.xlsx
products_2024-01-15.csv
products_2024-01-15.json
```

### Data Transformation

Transform data format before export.

**Price Formatting**:
```javascript
// Input: 9999
// Output: $9,999.00
return '$' + value.toLocaleString('en-US', {
  minimumFractionDigits: 2,
  maximumFractionDigits: 2
})
```

**Date Formatting**:
```javascript
// Input: 2024-01-15T10:30:00Z
// Output: January 15, 2024 10:30 AM
const date = new Date(value)
return date.toLocaleString('en-US')
```

**URL Conversion**:
```javascript
// Convert relative URL to absolute URL
if (value.startsWith('/')) {
  return 'https://example.com' + value
}
return value
```

### Template Export

Export using custom templates.

**Excel Template**:
1. Create Excel template file
2. Define data areas and formats
3. Upload template to plugin
4. Apply template when exporting

**HTML Template**:
```html
<!DOCTYPE html>
<html>
<head>
  <title>Collection Data Report</title>
  <style>
    table { border-collapse: collapse; }
    th, td { border: 1px solid #ddd; padding: 8px; }
  </style>
</head>
<body>
  <h1>{{taskName}} - Collection Report</h1>
  <p>Collection Time: {{datetime}}</p>
  <p>Data Count: {{count}}</p>
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

## Export to Database

### Supported Databases

- MySQL
- PostgreSQL
- MongoDB
- SQLite

### Configure Database Connection

**MySQL Example**:
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

### Field Mapping

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

### Insert Modes

**Insert New Data**:
```sql
INSERT INTO products (product_name, price, image_url)
VALUES ('iPhone 15', 9999, 'https://...')
```

**Update Existing Data**:
```sql
INSERT INTO products (id, product_name, price)
VALUES (123, 'iPhone 15', 9999)
ON DUPLICATE KEY UPDATE
  product_name = VALUES(product_name),
  price = VALUES(price)
```

**Ignore Duplicates**:
```sql
INSERT IGNORE INTO products (product_name, price)
VALUES ('iPhone 15', 9999)
```

## Export to Cloud Storage

### Supported Cloud Storage Services

- Alibaba Cloud OSS
- Tencent Cloud COS
- AWS S3
- Google Cloud Storage

### Configuration Example

**Alibaba Cloud OSS**:
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

**Upload Options**:
- Auto upload
- Compress before upload
- Encrypted upload
- Set access permissions

## Data Security

### Sensitive Data Processing

**Data Masking**:
```javascript
// Phone number masking
// 13812345678 → 138****5678
return value.replace(/(\d{3})\d{4}(\d{4})/, '$1****$2')

// Email masking
// user@example.com → u***@example.com
return value.replace(/(.{1}).*(@.*)/, '$1***$2')

// ID number masking
// 110101199001011234 → 110101********1234
return value.replace(/(\d{6})\d{8}(\d{4})/, '$1********$2')
```

### Encrypted Export

**File Encryption**:
```json
{
  "encryption": {
    "enabled": true,
    "algorithm": "AES-256",
    "password": "your-password"
  }
}
```

**Compressed Encryption**:
```json
{
  "compression": {
    "enabled": true,
    "format": "zip",
    "password": "your-password"
  }
}
```

## Export Logs

### View Export History

1. Enter "Data Management"
2. Click "Export History"
3. View all export records

**Record Information**:
- Export time
- Filename
- Data count
- File size
- Export format
- Export status

### Re-export

For historical data, can re-export:

1. Find record in export history
2. Click "Re-export"
3. Select new format or configuration
4. Complete export

## Troubleshooting

### Issue 1: Exported file shows garbled characters

**Cause**: Encoding issue

**Solution**:
1. Use UTF-8 with BOM encoding for CSV files
2. Or use Excel format instead of CSV
3. Open CSV with Notepad, save as UTF-8 encoding

### Issue 2: Excel file too large to open

**Cause**: Data volume exceeds Excel limits

**Solution**:
1. Use batch export
2. Or use CSV format
3. Or export to database

### Issue 3: Google Sheets sync failed

**Cause**: Authorization expired or network issues

**Solution**:
1. Re-authorize Google account
2. Check network connection
3. View error logs
4. Reduce sync frequency

### Issue 4: Exported data incomplete

**Cause**: Filter conditions or field selection issues

**Solution**:
1. Check filter conditions
2. Confirm field selection
3. View export logs
4. Re-export

## Best Practices

### 1. Choose Appropriate Format

✅ **Recommended**:
- Use Excel for small data volumes
- Use CSV for large data volumes
- Use JSON for API integration
- Use Google Sheets for team collaboration

### 2. Regular Backup

✅ **Recommended**:
- Set up auto export
- Multi-format backup
- Cloud storage backup
- Regular cleanup of old data

### 3. Data Validation

✅ **Recommended**:
- Check data before export
- Validate field completeness
- Check data format
- Test import process

### 4. Performance Optimization

✅ **Recommended**:
- Export large data in batches
- Compress before transmission
- Asynchronous export processing
- Set reasonable caching

## Next Steps

- [Scheduled Tasks](./scheduled-tasks) - Set up automated collection and export
- [Data Processing](./data-processing) - Learn about data cleaning and transformation
- [Third-party Integration](./integrations) - Integrate more external services
- [Tutorials](./tutorials) - Complete data collection and export cases