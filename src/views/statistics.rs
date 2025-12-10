use schemars::JsonSchema;
use serde::Serialize;

/// # 任务统计
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskStatistics {
    /// # 总任务数
    pub total: i64,
    /// # 未部署任务数（没有调度配置）
    pub undeployed: i64,
    /// # 调度中任务数（有调度配置）
    pub scheduled: i64,
    /// # 调度结束任务数（已删除）
    pub completed: i64,
}

/// # 实例统计
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct InstanceStatistics {
    /// # 调度次数
    pub total_count: i64,
    /// # 失败次数
    pub failed_count: i64,
}

/// # 使用统计
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UsageStatistics {
    /// # 数据表数量
    pub table_count: i64,
    /// # 数据占用空间（MB）
    pub storage_size: f64,
    /// # 时间序列数据（用于图表）
    pub time_series: Vec<TimeSeriesData>,
}

/// # 时间序列数据点
#[derive(Debug, Serialize, JsonSchema)]
pub struct TimeSeriesData {
    /// # 日期（格式：YYYY/MM/DD）
    pub date: String,
    /// # 数值
    pub value: f64,
}

/// # 统计汇总
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StatisticsSummary {
    /// # 任务统计
    pub task_stats: TaskStatistics,
    /// # 实例统计
    pub instance_stats: InstanceStatistics,
    /// # 使用统计
    pub usage_stats: UsageStatistics,
}

