export interface Task {
  id: string;
  userId: string;
  name: string;
  created: string;
  modified: string;
  rule: any;
  /** 与后端 `ScraperTaskData` 一致：可仅含 `dataQuality`，`schedule` 可选 */
  data: ScraperTaskData | null;
}

/** 对应后端 `scraper_task.data` JSON（camelCase） */
export interface ScraperTaskData {
  schedule?: ScheduleConfig;
  dataQuality?: DataQualityConfig;
}

export interface DataQualityConfig {
  dedupeRuleVersion?: number;
  dedupeJsonPaths?: string[];
}

/** 扩展/本地缓存用（非后端 `data` 主结构） */
export interface TaskData {
  localCron?: LocalCron;
  tabId?: number;
  cron?: string;
}

export interface LocalCron {
  id?: string;
  cron: string;
}

export enum TaskStatusDesc {
  WAITING = "排队中",
  RUNNING = "运行中",
  SUCCESSFUL = "执行成功",
  FAILED = "执行失败",
  CANCELLED = "已取消",
}

export type TaskStatus = keyof TaskStatusDesc;

export enum InstanceStatusDesc {
  RUNNING = "运行中",
  SUCCESS = "执行成功",
  FAILED = "执行失败",
}

export interface TaskInstance {
  id: string;
  taskId: string;
  status: keyof InstanceStatusDesc;
  created: string;
  modified: string;
  dataCount: number;
  logKey?: string;
  errorMessage?: string;
}

/** GET `/instance/data` 单条采集记录（与实例侧分表字段一致） */
export interface TaskInstanceCaptureItem {
  id: number;
  taskId: number;
  taskInstanceId: number;
  dedupeRuleVersion: number;
  dedupeKey?: string | null;
  payload: Record<string, unknown>;
  createdAt: string;
}

export enum SchedulerTypeDesc {
  FAST = "快速HTTP调度",
  BROWSER = "浏览器调度",
}

export type SchedulerType = keyof SchedulerTypeDesc;

export interface ScheduleConfig {
  type: SchedulerType;
  cron: string;
  storeId: string;
  fieldMapping: FieldMapping;
  proxyId: string;
}

export interface FieldMapping {
  [id: string]: string;
}
