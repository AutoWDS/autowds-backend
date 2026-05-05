# Frequently Asked Questions

## Installation and Configuration

### Which browsers does the plugin support?

AutoWDS currently supports the following browsers:
- Chrome 90+
- Microsoft Edge 90+
- Brave
- Other Chromium-based browsers

**Note**: Firefox and Safari are not currently supported.

### Can't find the plugin icon after installation?

1. Open browser extension management page (chrome://extensions/)
2. Confirm AutoWDS is enabled
3. Click the "Pin" button next to the extension icon to pin it to the toolbar
4. If still not visible, try restarting the browser

### What permissions does the plugin need?

The plugin requires the following permissions to function properly:
- **tabs**: Access tab information
- **activeTab**: Interact with current active tab
- **scripting**: Inject scripts into web pages
- **storage**: Save collection rules and data
- **alarms**: Schedule tasks
- **notifications**: Send task notifications
- **host_permissions**: Access website content

All permissions are used for data collection functionality and will not collect or upload your personal information.

## Account and Registration

### Is account registration required?

Basic features can be used without registration, but registering an account provides:
- Cloud rule synchronization
- Larger data storage space
- Scheduled task functionality
- Technical support services
- Access to advanced features

### What if I forget my password?

1. Click the "Forgot Password" link on the login page
2. Enter the email used during registration
3. Check for password reset email
4. Click the link in the email to set a new password

### How to modify account information?

1. Click the plugin icon to open the main interface
2. Go to "User" menu
3. Click "Account Information"
4. Modify the information you want to update
5. Click "Save" button

## Data Collection

### Why can't I collect data?

Possible reasons and solutions:

**1. Selector Failed**
- Website updated page structure
- Solution: Re-select elements and update selectors

**2. Dynamic Content**
- Content loaded dynamically via JavaScript
- Solution: Add wait time or use scroll trigger

**3. Login Required**
- Website requires login to view content
- Solution: Login in browser first, then collect

**4. Anti-scraping Restrictions**
- Website detected automated access
- Solution: Reduce collection frequency, add random delays

### How to collect from websites requiring login?

Method 1: Manual Login
1. Login to the website normally in browser
2. Keep logged in
3. Use plugin to collect

Method 2: Configure Login Steps
1. Add login steps in rules
2. Configure username and password input
3. Set login button click
4. Wait for login completion before collecting

### Collection speed is too slow?

Optimization suggestions:
1. **Reduce unnecessary fields**: Only collect truly needed data
2. **Optimize selectors**: Use more precise CSS selectors
3. **Adjust concurrency**: Increase concurrent collection count in settings
4. **Disable image loading**: If images not needed, disable image loading
5. **Use incremental collection**: Only collect new or changed data

### How to handle CAPTCHAs?

The plugin currently does not support automatic CAPTCHA recognition. Suggestions:
1. Complete CAPTCHA verification manually first
2. Collect while logged in
3. Use API interfaces instead of web scraping (if website provides)

## Rule Configuration

### What's the difference between CSS selectors and XPath?

**CSS Selectors**:
- Concise syntax, easy to understand
- Native browser support, good performance
- Suitable for most scenarios
- Example: `div.product > h2.title`

**XPath**:
- More powerful, supports complex queries
- Can search upward for parent elements
- Supports text content matching
- Example: `//div[@class='product']/h2[@class='title']`

Recommend using CSS selectors first, consider XPath for complex scenarios.

### How to choose stable selectors?

Good selectors should:
1. **Be unique**: Accurately locate target element
2. **Be structurally stable**: Don't rely on easily changing attributes (like dynamically generated classes)
3. **Have moderate hierarchy**: Not too deep or too shallow
4. **Be semantically clear**: Use meaningful class or id

Recommended priority:
1. `id` attribute (if stable)
2. Semantic `class` names
3. `data-*` custom attributes
4. Structural selectors (like `:nth-child()`)

### How to handle dynamic content?

For content dynamically loaded via JavaScript:

**Method 1: Add Wait Time**
```
Add "Wait" action in steps
Set wait time (e.g., 2 seconds)
```

**Method 2: Wait for Element**
```
Configure "Wait for Element" condition
Specify element selector to wait for
Set maximum wait time
```

**Method 3: Scroll to Trigger Loading**
```
Configure scroll action
Set scroll distance or scroll to bottom
Wait for content to load
```

## Pagination and Navigation

### What pagination methods are supported?

The plugin supports the following pagination methods:

1. **Click Next Page**: Click "Next" button or page number links
2. **Infinite Scroll**: Scroll to bottom to auto-load more
3. **Load More Button**: Click "Load More" button
4. **URL Parameter Pagination**: Modify page number parameter in URL

### How to set pagination limit?

In pagination configuration:
1. Find "Maximum Pages" setting
2. Enter maximum number of pages to collect
3. Or set "Maximum Data Count" limit
4. Automatically stops when limit reached

### Duplicate data after pagination?

Enable deduplication:
1. Enable "Deduplication" in data save settings
2. Select deduplication fields (like product ID, URL, etc.)
3. Set deduplication strategy (keep first or latest)

## Data Export

### What export formats are supported?

- **Excel (.xlsx)**: Suitable for data analysis and reports
- **CSV (.csv)**: Universal format, can be imported to databases
- **JSON (.json)**: Suitable for program processing and API integration
- **Google Sheets**: Cloud collaboration and real-time sync

### Exported Excel file has garbled characters?

This is usually an encoding issue:
1. When opening CSV file with Excel, select UTF-8 encoding
2. Or export directly as Excel format (.xlsx)
3. If still problematic, try opening CSV with Notepad, save as UTF-8 encoding

### How to export to Google Sheets?

1. Select "Google Sheets" in data export settings
2. Click "Authorize" button to login to Google account
3. Select or create target worksheet
4. Configure field mapping
5. Choose save mode (append/overwrite)
6. Start export

### How to export large amounts of data?

For large data volumes:
1. **Batch export**: Export in batches by time or category
2. **Use CSV format**: More suitable for large data than Excel
3. **Save directly to database**: Configure database connection for direct save
4. **Use data pipeline**: Configure data flow processing pipeline

## Scheduled Tasks

### How to set up scheduled tasks?

1. Create or edit collection task
2. Click "Schedule Settings"
3. Select execution frequency or enter Cron expression
4. Enable scheduled task
5. Save settings

### How to write Cron expressions?

Cron expression format: `minute hour day month weekday`

Common examples:
- `0 9 * * *` - Every day at 9 AM
- `0 */2 * * *` - Every 2 hours
- `0 9 * * 1-5` - Weekdays at 9 AM
- `0 0 1 * *` - 1st of every month at midnight
- `*/30 * * * *` - Every 30 minutes

You can use online Cron expression generators for assistance.

### Scheduled task not executing?

Check the following:
1. Confirm task is enabled
2. Check if Cron expression is correct
3. Ensure browser is open at execution time
4. Check error logs in task execution history
5. Check for permission restrictions

### Will scheduled tasks execute after computer shutdown?

No. Scheduled tasks require:
1. Browser to be open
2. Plugin to be enabled
3. Computer to be powered on

For 24/7 operation, consider:
- Deploy on cloud server
- Or use our cloud collection service

## Performance and Limitations

### How much data can be collected at once?

Theoretically no hard limit, but recommended:
- Single task should not exceed 10,000 records
- Collect in batches if exceeding
- Monitor browser memory usage

### Will collection get blocked by websites?

Possibly. To avoid blocking:
1. **Control collection frequency**: Set reasonable request intervals
2. **Simulate human behavior**: Add random delays
3. **Use proxy IPs**: Rotate IP addresses (advanced feature)
4. **Respect robots.txt**: Honor website's crawler protocol
5. **Avoid peak hours**: Choose low-traffic periods

### Plugin using too much memory?

Optimization suggestions:
1. Reduce concurrent collection count
2. Clean up historical data promptly
3. Close unnecessary tasks
4. Restart browser to free memory
5. Upgrade browser to latest version

## Error Handling

### Common Error Codes

**ERR_SELECTOR_NOT_FOUND**
- Cause: Cannot find specified element
- Solution: Check if selector is correct, if page has loaded

**ERR_TIMEOUT**
- Cause: Operation timeout
- Solution: Increase timeout duration, check network connection

**ERR_NAVIGATION_FAILED**
- Cause: Page navigation failed
- Solution: Check if URL is correct, if website is accessible

**ERR_PERMISSION_DENIED**
- Cause: Insufficient permissions
- Solution: Check plugin permission settings, reinstall plugin

### How to view error logs?

1. Open plugin main interface
2. Go to "Task Management"
3. Select the failed task
4. Click "Execution History"
5. View detailed error information

### How to report issues?

1. **Check documentation**: Review relevant docs and FAQ first
2. **Search community**: Search for similar issues in user community
3. **Submit ticket**: Submit technical support ticket via official website
4. **Send email**: Send detailed problem description to support@dtiku.cn
5. **Join community**: Join user exchange group for help

When reporting, please provide:
- Plugin version number
- Browser version
- Target website URL (if publicly shareable)
- Error screenshots or logs
- Steps to reproduce

## Advanced Features

### How to use data pipelines?

Data pipelines process collected data:
1. Enable "Data Pipeline" in task settings
2. Add processing nodes (filter, transform, validate, etc.)
3. Configure processing rules for each node
4. Set output target
5. Test and save

### How to integrate third-party services?

Supported integration methods:
1. **Webhook**: Configure HTTP callback URL
2. **API Push**: Configure API interface information
3. **Database Connection**: Configure database connection info
4. **Cloud Storage**: Configure OSS/S3 cloud storage

For specific configuration methods, refer to [Third-party Integration](./integrations) documentation.

### How to use rule templates?

Using existing templates:
1. Go to "Task Management"
2. Click "Create from Template"
3. Select appropriate template
4. Modify template parameters
5. Save as new task

Creating your own templates:
1. Create a collection rule
2. Test to confirm rule is correct
3. Click "Save as Template"
4. Fill in template information
5. Optionally share with other users

## Other Questions

### Is the plugin open source?

Part of the plugin code is open source, you can view and contribute on GitHub. The core collection engine is commercial.

### Is data secure?

Yes, we take data security very seriously:
- All data processing completed locally
- Will not upload your collected data to servers
- Account passwords stored encrypted
- Supports local data encryption

### Can it be used for commercial purposes?

Yes. But please note:
1. Comply with target website's terms of service
2. Respect data copyright and privacy
3. Do not use for illegal purposes
4. Commercial use recommended to purchase commercial license

### How to upgrade to professional version?

1. Login to account
2. Go to "Membership Services"
3. Select appropriate plan
4. Complete payment
5. Immediately enjoy professional features

Professional features include:
- Higher collection frequency
- Larger data storage
- Priority technical support
- Access to advanced features
- Cloud collection service

---

If your question is not answered here, please contact our technical support team and we will respond as soon as possible.
