import { Page } from "../types/Page";
import { TaskInstance, TaskInstanceCaptureItem } from "../types/Task";
import ajax from "../utils/ajax";

const instance = () => ajax("/instance");

export interface InstanceQuery {
  taskId?: string;
  status?: string;
  createStart?: string;
  createEnd?: string;
  page?: number;
  size?: number;
}

export async function queryInstance(query: InstanceQuery) {
  return instance().query(query).get() as Promise<Page<TaskInstance>>;
}

export interface InstanceCaptureQuery {
  taskId: string;
  instanceId: string;
  page?: number;
  size?: number;
}

/** 某次任务实例的采集记录（分表 `task_instance_record_{userId}`，由后端 sqlx 查询） */
export async function queryInstanceCaptureData(query: InstanceCaptureQuery) {
  return instance().path("data").query(query).get() as Promise<Page<TaskInstanceCaptureItem>>;
}
