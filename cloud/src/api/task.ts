import { Graph } from "types/NodeTypes";
import { Page } from "../types/Page";
import { ScheduleConfig, ScraperTaskData, Task } from "../types/Task";
import ajax from "../utils/ajax";

const task = () => ajax("/task");

/** 获取任务详情（含 `rule`、`data`，与后端 `GET /task/{id}` 一致） */
export async function getTask(id: string): Promise<Task> {
  return task().path(id).get() as Promise<Task>;
}

/** 更新任务（须同时传 `rule`，避免覆盖为空；`data` 与后端 `ScraperTaskData` 信封一致） */
export async function updateTask(
  id: string,
  body: { data?: ScraperTaskData | null; rule: unknown }
): Promise<Task> {
  return task().path(id).payload(body).put() as Promise<Task>;
}

export async function queryTask(
  name: string,
  page: number,
  size: number,
  startTime?: string,
  endTime?: string
) {
  return task()
    .query({ name, page, size, startTime, endTime })
    .get() as Promise<Page<Task>>;
}

export async function addTask(partialTask: Partial<Task>): Promise<Task> {
  return task().payload(partialTask).post() as Promise<Task>;
}

export async function getRule(taskId: string) {
  return task().path(taskId).path("/rule").get() as Promise<Graph>;
}

export async function renameTask(id: string, name: string) {
  return task().path(id).payload({ name }).put() as Promise<Task>;
}

export async function deleteTask(id: string) {
  return task().path(id).delete();
}

export async function updateRemoteCron(id: string, cron: string) {
  return task().path(id).path("cron").payload(cron).patch() as Promise<Task>;
}

export async function getScheduleConfig(id: string) {
  return task()
    .path(id)
    .path("schedule")
    .get() as Promise<ScheduleConfig>;
}

export async function saveScheduleConfig(id: string, config: ScheduleConfig) {
  return task()
    .path(id)
    .path("schedule")
    .payload(config)
    .patch();
}
