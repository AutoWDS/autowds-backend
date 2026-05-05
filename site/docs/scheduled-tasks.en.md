# Scheduled Tasks

Scheduled tasks allow you to automatically execute data collection according to preset times, achieving unattended automated data collection. AutoWDS supports flexible scheduling configuration to meet various automated collection needs.

## What are Scheduled Tasks?

Scheduled tasks are functionality that automatically executes collection tasks according to set time rules. You can set tasks to run automatically at specific times, daily, weekly, or at custom time intervals.

### Application Scenarios

**Price Monitoring**
- Daily scheduled collection of product prices
- Monitor price change trends
- Send notifications when prices are abnormal

**Content Updates**
- Scheduled collection of news articles
- Monitor website content updates
- Automatically archive historical data

**Data Synchronization**
- Regularly sync data to database
- Update Google Sheets
- Generate periodic reports

**Competitive Analysis**
- Scheduled collection of competitor information
- Track market dynamics
- Generate analysis reports

## Creating Scheduled Tasks

### Method 1: Set When Creating New Task

1. **Create Collection Task**
   - Configure collection rules
   - Test to confirm rules are correct

2. **Enable Scheduling**
   - Find "Scheduled Execution" in task settings
   - Enable scheduling function

3. **Configure Execution Time**
   - Select execution frequency
   - Or enter Cron expression
   - Set start and end times

4. **Save Task**
   - Save task configuration
   - Task will execute automatically on schedule

### Method 2: Add Scheduling to Existing Task

1. **Open Task List**
   - Enter "Task Management"
   - Find task to schedule

2. **Edit Task**
   - Click task "Edit" button
   - Enter task settings

3. **Configure Scheduling**
   - Find "Schedule Settings" option
   - Configure execution rules

4. **Enable and Save**
   - Enable scheduling function
   - Save settings

## Scheduling Configuration Methods

### Simple Mode

Use preset common time rules.

**Daily Execution**:
```
Execution Time: Daily 09:00
Description: Execute once daily at 9 AM
```

**Weekly Execution**:
```
Execution Time: Monday 09:00
Description: Execute Monday at 9 AM
```

**Monthly Execution**:
```
Execution Time: 1st of month 00:00
Description: Execute at midnight on 1st of each month
```

**Interval Execution**:
```
Execution Interval: Every 2 hours
Description: Execute every 2 hours
```

### Cron Expression Mode

Use Cron expressions for more flexible scheduling configuration.

#### Cron Expression Format

```
* * * * *
│ │ │ │ │
│ │ │ │ └─── Day of week (0-7, both 0 and 7 represent Sunday)
│ │ │ └───── Month (1-12)
│ │ └─────── Day (1-31)
│ └───────── Hour (0-23)
└─────────── Minute (0-59)
```

#### Common Expression Examples

**Daily Scheduling**:
```cron
# Daily at 9 AM
0 9 * * *

# Daily at 2:30 PM
30 14 * * *

# Daily at noon and 6 PM
0 12,18 * * *
```

**Weekly Scheduling**:
```cron
# Monday at 9 AM
0 9 * * 1

# Weekdays at 9 AM
0 9 * * 1-5

# Weekends at 10 AM
0 10 * * 0,6
```

**Monthly Scheduling**:
```cron
# 1st of month at midnight
0 0 1 * *

# 15th of month at noon
0 12 15 * *

# Last day of month
0 0 L * *
```

**Interval Execution**:
```cron
# Every hour
0 * * * *

# Every 2 hours
0 */2 * * *

# Every 30 minutes
*/30 * * * *

# Every 15 minutes
*/15 * * * *
```

**Specific Time Periods**:
```cron
# Weekdays 9 AM to 6 PM, every hour
0 9-18 * * 1-5

# Daily 6 AM to 10 PM, every 2 hours
0 6-22/2 * * *
```

#### Cron Expression Special Characters

| Character | Description | Example |
|-----------|-------------|---------|
| * | Any value | `* * * * *` every minute |
| , | List values | `0 9,12,18 * * *` at 9, 12, 18 |
| - | Range values | `0 9-17 * * *` 9 AM to 5 PM |
| / | Interval values | `*/10 * * * *` every 10 minutes |
| L | Last | `0 0 L * *` last day of month |
| W | Weekday | `0 0 15W * *` weekday closest to 15th |
| # | Nth occurrence | `0 0 * * 1#2` 2nd Monday of month |

### Advanced Configuration

**Execution Window**:
```json
{
  "cron": "0 9 * * *",
  "window": {
    "start": "2024-01-01",
    "end": "2024-12-31"
  }
}
```

**Execution Count Limit**:
```json
{
  "cron": "0 */2 * * *",
  "maxExecutions": 10
}
```

**Skip Holidays**:
```json
{
  "cron": "0 9 * * 1-5",
  "skipHolidays": true,
  "holidayCalendar": "US"
}
```

## Task Execution Management

### View Execution Schedule

View task's next execution time and execution history.

**Execution Schedule Info**:
```
Task Name: Product Price Monitor
Cron Expression: 0 9 * * *
Next Execution: 2024-01-16 09:00:00
Last Execution: 2024-01-15 09:00:00
Execution Status: Success
```

### Execution History

View task's historical execution records.

**History Records Include**:
- Execution time
- Execution status (success/failure)
- Collected data count
- Execution duration
- Error information (if failed)

**Example**:
```
2024-01-15 09:00:00 | Success | 1000 records | 5m30s
2024-01-14 09:00:00 | Success | 980 records | 5m15s
2024-01-13 09:00:00 | Failed | 0 records | Network timeout
2024-01-12 09:00:00 | Success | 1020 records | 5m45s
```

### Manual Trigger

Even with scheduling configured, you can manually execute tasks anytime.

1. Open task list
2. Find task to execute
3. Click "Execute Now" button
4. Task starts running

**Note**: Manual execution doesn't affect scheduled plan.

### Pause and Resume

Temporarily pause scheduled tasks, resume when needed.

**Pause Task**:
1. Open task settings
2. Turn off "Enable Scheduling" switch
3. Save settings

**Resume Task**:
1. Open task settings
2. Turn on "Enable Scheduling" switch
3. Save settings

### Delete Scheduling

Completely remove task's scheduling configuration.

1. Open task settings
2. Enter "Schedule Settings"
3. Click "Delete Schedule"
4. Confirm deletion

**Note**: Deleting schedule doesn't delete the task itself.

## Execution Notifications

### Notification Types

**Task Start Notification**:
```
Title: Scheduled Task Started
Content: Task "Product Price Monitor" started execution
Time: 2024-01-15 09:00:00
```

**Task Completion Notification**:
```
Title: Scheduled Task Completed Successfully
Content: Task "Product Price Monitor" completed execution
Data: Collected 1000 records
Duration: 5m30s
```

**Task Failure Notification**:
```
Title: Scheduled Task Failed
Content: Task "Product Price Monitor" execution failed
Error: Network connection timeout
Suggestion: Please check network connection and retry
```

### Notification Methods

**Browser Notification**:
- Chrome native notifications
- Requires notification permission authorization
- Visible when browser is open

**Email Notification**:
```json
{
  "notification": {
    "email": {
      "enabled": true,
      "to": "your@email.com",
      "onSuccess": false,
      "onFailure": true
    }
  }
}
```

**Webhook Notification**:
```json
{
  "notification": {
    "webhook": {
      "enabled": true,
      "url": "https://your-webhook-url.com",
      "method": "POST",
      "headers": {
        "Authorization": "Bearer your-token"
      }
    }
  }
}
```

**DingTalk/Enterprise WeChat**:
```json
{
  "notification": {
    "dingtalk": {
      "enabled": true,
      "webhook": "https://oapi.dingtalk.com/robot/send?access_token=xxx",
      "atMobiles": ["13812345678"]
    }
  }
}
```

### Notification Conditions

Configure when to send notifications.

**Always Notify**:
```json
{
  "notifyOn": "always"
}
```

**Only on Failure**:
```json
{
  "notifyOn": "failure"
}
```

**Only on Success**:
```json
{
  "notifyOn": "success"
}
```

**Conditional Notification**:
```json
{
  "notifyOn": "condition",
  "conditions": [
    {
      "type": "dataCount",
      "operator": "<",
      "value": 100,
      "message": "Collected data count less than expected"
    },
    {
      "type": "duration",
      "operator": ">",
      "value": 600,
      "message": "Execution time exceeded 10 minutes"
    }
  ]
}
```

## Error Handling

### Auto Retry

Automatically retry when task execution fails.

```json
{
  "retry": {
    "enabled": true,
    "maxAttempts": 3,
    "interval": 300,
    "backoff": "exponential"
  }
}
```

**Retry Strategies**:
- **Fixed Interval**: Same interval for each retry
- **Exponential Backoff**: Gradually increasing retry intervals (5min, 10min, 20min)
- **Linear Increase**: Linearly increasing retry intervals (5min, 10min, 15min)

### Failure Handling

Configure handling after task failure.

**Continue Execution**:
```json
{
  "onFailure": "continue",
  "skipToNext": true
}
```

**Pause Task**:
```json
{
  "onFailure": "pause",
  "requireManualResume": true
}
```

**Send Alert**:
```json
{
  "onFailure": "alert",
  "alertChannels": ["email", "webhook"]
}
```

### Timeout Settings

Set maximum execution time for tasks.

```json
{
  "timeout": {
    "enabled": true,
    "duration": 1800,
    "action": "abort"
  }
}
```

**Timeout Actions**:
- **abort**: Abort task
- **continue**: Continue execution
- **savePartial**: Save collected data

## Concurrency Control

### Prevent Duplicate Execution

Ensure same task doesn't execute multiple times simultaneously.

```json
{
  "concurrency": {
    "preventOverlap": true,
    "skipIfRunning": true
  }
}
```

**Handling Strategies**:
- **Skip**: Skip current execution if task is running
- **Queue**: Wait for current execution to complete before executing
- **Abort**: Abort current execution and start new one

### Task Priority

Set execution priority for multiple scheduled tasks.

```json
{
  "priority": "high",
  "maxConcurrent": 3
}
```

**Priority Levels**:
- **high**: High priority, execute first
- **normal**: Normal priority
- **low**: Low priority, execute last

## Resource Management

### Execution Time Window Restrictions

Restrict tasks to execute only during specific time periods.

```json
{
  "executionWindow": {
    "allowedHours": [0, 1, 2, 3, 4, 5, 22, 23],
    "timezone": "America/New_York"
  }
}
```

**Use Cases**:
- Avoid website peak hours
- Utilize nighttime periods for collection
- Comply with website access restrictions

### Resource Limits

Limit system resources used by tasks.

```json
{
  "resources": {
    "maxMemory": "2GB",
    "maxCPU": "50%",
    "maxDuration": "30m"
  }
}
```

## Advanced Features

### Task Chains

Execute multiple tasks in sequence.

```json
{
  "taskChain": {
    "enabled": true,
    "tasks": [
      {
        "taskId": "task-1",
        "waitForCompletion": true
      },
      {
        "taskId": "task-2",
        "waitForCompletion": true
      },
      {
        "taskId": "task-3",
        "waitForCompletion": false
      }
    ]
  }
}
```

**Execution Flow**:
```
Task1 → Wait for completion → Task2 → Wait for completion → Task3 → Don't wait
```

### Conditional Execution

Decide whether to execute task based on conditions.

```json
{
  "conditionalExecution": {
    "enabled": true,
    "conditions": [
      {
        "type": "dataChange",
        "source": "task-1",
        "threshold": 10
      },
      {
        "type": "timeRange",
        "start": "09:00",
        "end": "18:00"
      }
    ],
    "operator": "AND"
  }
}
```

### Incremental Collection

Only collect new or changed data.

```json
{
  "incremental": {
    "enabled": true,
    "compareField": "id",
    "lastRunData": "stored"
  }
}
```

**Working Principle**:
1. Record data from last collection
2. Compare during current collection
3. Only save new or changed data

## Monitoring and Logging

### Execution Monitoring

Real-time monitoring of task execution status.

**Monitoring Metrics**:
- Execution status
- Collection progress
- Data count statistics
- Resource usage
- Error information

### Log Recording

Detailed recording of task execution process.

**Log Levels**:
- **DEBUG**: Debug information
- **INFO**: General information
- **WARN**: Warning information
- **ERROR**: Error information

**Log Content**:
```
[2024-01-15 09:00:00] INFO Task execution started
[2024-01-15 09:00:05] INFO Page loading completed
[2024-01-15 09:00:10] INFO Started collecting list data
[2024-01-15 09:02:30] INFO Collected page 1, got 50 records
[2024-01-15 09:05:00] INFO Collected page 2, got 48 records
[2024-01-15 09:05:30] INFO Task execution completed, collected 1000 records total
```

### Performance Analysis

Analyze task execution performance.

**Performance Metrics**:
- Total execution time
- Page loading time
- Data extraction time
- Network request time
- Data processing time

## Best Practices

### 1. Set Reasonable Execution Frequency

✅ **Recommended**:
- Set based on data update frequency
- Avoid overly frequent execution
- Consider website access restrictions
- Stagger peak hours

❌ **Avoid**:
- Execute every minute
- Execute many tasks simultaneously
- Ignore website load
- Ignore anti-scraping restrictions

### 2. Configure Appropriate Notifications

✅ **Recommended**:
- Send notifications on failure
- Send alerts for abnormal situations
- Send periodic execution reports
- Use multiple notification methods

❌ **Avoid**:
- Notify on every execution
- Unclear notification information
- Use only single notification method
- Ignore notification failures

### 3. Handle Errors Properly

✅ **Recommended**:
- Configure auto retry
- Set reasonable timeouts
- Record detailed error logs
- Regularly check execution status

❌ **Avoid**:
- Don't handle errors
- Infinite retries
- Ignore timeouts
- Don't record logs

### 4. Monitor Task Execution

✅ **Recommended**:
- Regularly check execution history
- Analyze performance metrics
- Optimize execution efficiency
- Handle exceptions promptly

❌ **Avoid**:
- Set and forget
- Don't check execution results
- Ignore performance issues
- Don't handle failed tasks

## Troubleshooting

### Issue 1: Scheduled task not executing

**Possible Causes**:
- Task not enabled
- Incorrect Cron expression
- Browser not open
- Incorrect system time

**Solutions**:
1. Check if task is enabled
2. Validate Cron expression
3. Ensure browser is open
4. Check system time settings

### Issue 2: Task execution failed

**Possible Causes**:
- Network connection issues
- Website structure changes
- Selector failures
- Anti-scraping restrictions

**Solutions**:
1. Check network connection
2. Update collection rules
3. Adjust selectors
4. Reduce collection frequency

### Issue 3: Not receiving notifications

**Possible Causes**:
- Notifications not enabled
- Incorrect notification configuration
- Permissions not granted
- Network issues

**Solutions**:
1. Check notification settings
2. Verify configuration information
3. Grant notification permissions
4. Test notification functionality

## Next Steps

- [Batch Collection](./batch-collection) - Batch collect multiple targets
- [Data Processing](./data-processing) - Process collected data
- [Third-party Integration](./integrations) - Integrate external services
- [Tutorials](./tutorials) - Complete automated collection cases