import request from '@/utils/request'

export interface User {
  id: number
  username: string
  email: string
  status: string
  created_at: string
  credits: number
  invite_code: string
  invited_by?: number
  edition: string
  email_subscribed: boolean
}

export interface UserListParams {
  /** 与后端 summer_sea_orm::Pagination 一致；配置 one_indexed 时与 UI 页码同为从 1 开始 */
  page?: number
  size?: number
  keyword?: string
  user_id?: number
}

export interface UserListResponse {
  content: User[]
  size: number
  page: number
  total_elements: number
  total_pages: number
}

export const getUserList = (params: UserListParams): Promise<UserListResponse> => {
  return request.get('/admin/user/list', { params })
}

export const createUser = (data: any) => {
  return request.post('/admin/user/create', data)
}

export const updateUser = (id: number, data: any) => {
  return request.put(`/admin/user/${id}`, data)
}

export const deleteUser = (id: number) => {
  return request.delete(`/admin/user/${id}`)
}

export interface AdjustCreditsData {
  amount: number
  description: string
}

export const adjustUserCredits = (id: number, data: AdjustCreditsData) => {
  return request.post(`/admin/user/${id}/adjust-credits`, data)
}

export interface UpdateUserEditionData {
  edition: string
  description: string
}

export const updateUserEdition = (id: number, data: UpdateUserEditionData) => {
  return request.post(`/admin/user/${id}/update-edition`, data)
}

export interface SendMarketingEmailData {
  subject: string
  html_body: string
  user_ids: number[]
}

export interface SendMarketingEmailResponse {
  sent: number
  skipped_unsubscribed: number
  not_found: number
  failed: string[]
}

export const sendMarketingEmail = (
  data: SendMarketingEmailData
): Promise<SendMarketingEmailResponse> => {
  return request.post('/admin/user/send-marketing-email', data)
}
