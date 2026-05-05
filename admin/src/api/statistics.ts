import request from '@/utils/request'

export interface StatisticsOverview {
  user_count: number
  task_count: number
  template_count: number
}

export interface TaskStatistics {
  pending: number
  running: number
  completed: number
  failed: number
  deleted: number
}

export const getStatistics = (): Promise<StatisticsOverview> => {
  return request.get('/admin/statistics/overview')
}

export const getTaskStatistics = (params?: any): Promise<TaskStatistics> => {
  return request.get('/admin/statistics/tasks', { params })
}
