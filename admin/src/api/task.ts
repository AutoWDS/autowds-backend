import request from '@/utils/request'

export interface Task {
  id: number
  name: string
  status: string
  user_id: number
  creator_name?: string
  created_at: string
  updated_at: string
}

export interface TaskListParams {
  /** 与后端 summer_sea_orm::Pagination 一致；配置 one_indexed 时与 UI 页码同为从 1 开始 */
  page?: number
  size?: number
  status?: string
}

export interface TaskListResponse {
  content: Task[]
  size: number
  page: number
  total_elements: number
  total_pages: number
}

export const getTaskList = (params: TaskListParams): Promise<TaskListResponse> => {
  return request.get('/admin/task/list', { params })
}

export const createTask = (data: any) => {
  return request.post('/admin/task/create', data)
}

export const updateTask = (id: number, data: any) => {
  return request.put(`/admin/task/${id}`, data)
}

export const deleteTask = (id: number) => {
  return request.delete(`/admin/task/${id}`)
}

export const startTask = (id: number) => {
  return request.post(`/admin/task/${id}/start`)
}

export const stopTask = (id: number) => {
  return request.post(`/admin/task/${id}/stop`)
}
