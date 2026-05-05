import request from '@/utils/request'

export interface LoginParams {
  email: string
  passwd: string
}

export interface LoginResponse {
  id: number
  is_admin: boolean
  name: string
  email: string
  edition: string
  token: string
}

export const login = (data: LoginParams) => {
  return request.post<any, LoginResponse>('/token', data)
}

export const logout = () => {
  return request.post('/token/logout')
}

export const getUserInfo = () => {
  return request.get('/user/info')
}
