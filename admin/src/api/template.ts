import request from '@/utils/request'

// 模板主题枚举
export enum TemplateTopic {
  Bidding = 'Bidding',
  ECommerce = 'ECommerce',
  LocalLife = 'LocalLife',
  Media = 'Media',
  ResearchEducation = 'ResearchEducation',
  SearchEngine = 'SearchEngine',
  SocialNetwork = 'SocialNetwork',
  Other = 'Other',
}

// 产品版本枚举
export enum ProductEdition {
  L0 = 'L0',
  L1 = 'L1',
  L2 = 'L2',
  L3 = 'L3',
}

export interface Template {
  id: number
  name: string
  description: string
  rule: any
  topic: TemplateTopic
  edition: ProductEdition
  img: string
  lang: string
  params?: any
  created_at: string
}

export interface CreateTemplateRequest {
  name: string
  description?: string
  rule?: any
  topic: TemplateTopic
  edition: ProductEdition
  img?: string
  lang: string
  params?: any
}

export interface UpdateTemplateRequest {
  name: string
  description?: string
  rule?: any
  topic: TemplateTopic
  edition: ProductEdition
  img?: string
  lang: string
  params?: any
}

export interface TemplateListParams {
  /** 与后端 summer_sea_orm::Pagination 一致；配置 one_indexed 时与 UI 页码同为从 1 开始 */
  page?: number
  size?: number
}

export interface TemplateListResponse {
  content: Template[]
  size: number
  page: number
  total_elements: number
  total_pages: number
}

export const getTemplateList = (params?: TemplateListParams): Promise<TemplateListResponse> => {
  return request.get('/admin/template/list', { params })
}

export const createTemplate = (data: CreateTemplateRequest) => {
  return request.post('/admin/template/create', data)
}

export const updateTemplate = (id: number, data: UpdateTemplateRequest) => {
  return request.put(`/admin/template/${id}`, data)
}

export const deleteTemplate = (id: number) => {
  return request.delete(`/admin/template/${id}`)
}
