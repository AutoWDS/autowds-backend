import ajax from "utils/ajax";

const statistics = () => ajax("/statistics");

export interface TaskStatistics {
  total: number;
  undeployed: number;
  scheduled: number;
  completed: number;
}

export interface InstanceStatistics {
  totalCount: number;
  failedCount: number;
}

export interface TimeSeriesData {
  date: string;
  value: number;
}

export interface UsageStatistics {
  tableCount: number;
  storageSize: number;
  timeSeries: TimeSeriesData[];
}

export interface StatisticsSummary {
  taskStats: TaskStatistics;
  instanceStats: InstanceStatistics;
  usageStats: UsageStatistics;
}

export async function getStatistics(): Promise<StatisticsSummary> {
  return statistics().get() as Promise<StatisticsSummary>;
}

