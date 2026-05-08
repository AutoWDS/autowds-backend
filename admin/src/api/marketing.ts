import request from '@/utils/request'

export interface PageResponse<T> {
  content: T[]
  size: number
  page: number
  total_elements: number
  total_pages: number
}

export interface MarketingLead {
  id: number
  email: string
  name?: string
  source?: string
  status: string
  unsubscribed: boolean
  created_at: string
  last_seen_at?: string
}

export interface MarketingCampaign {
  id: number
  name: string
  subject: string
  landing_url: string
  status: string
  provider_receiver_id?: string
  provider_template_id?: string
  provider_task_id?: string
  created_at: string
  delivery_count: number
}

export interface ImportLeadsResp {
  created: number
  duplicated: number
  invalid: number
}

export interface FunnelMetric {
  event_type: string
  count: number
  rate: number
}

export interface CampaignFunnel {
  campaign_id: number
  sent: number
  metrics: FunnelMetric[]
}

export interface MarketingEvent {
  id: number
  campaign_id?: number
  delivery_id?: number
  lead_id?: number
  event_type: string
  url?: string
  user_agent?: string
  created_at: string
  meta?: Record<string, unknown>
}

export const getMarketingLeads = (params: {
  page?: number
  size?: number
  keyword?: string
  source?: string
  unsubscribed?: boolean
}): Promise<PageResponse<MarketingLead>> => {
  return request.get('/admin/marketing/leads', { params })
}

export const importMarketingLeads = (data: {
  csv_content: string
  source?: string
}): Promise<ImportLeadsResp> => {
  return request.post('/admin/marketing/leads/import', data)
}

export const getMarketingCampaigns = (params: {
  page?: number
  size?: number
  keyword?: string
  status?: string
}): Promise<PageResponse<MarketingCampaign>> => {
  return request.get('/admin/marketing/campaigns', { params })
}

export const createMarketingCampaign = (data: {
  name: string
  subject: string
  landing_url: string
  provider_template_id: string
  lead_ids: number[]
}): Promise<MarketingCampaign> => {
  return request.post('/admin/marketing/campaigns', data)
}

export const sendMarketingCampaign = (id: number): Promise<{
  receiver_id: number
  task_id: number
  delivery_count: number
}> => {
  return request.post(`/admin/marketing/campaigns/${id}/send`)
}

export const getCampaignFunnel = (id: number): Promise<CampaignFunnel> => {
  return request.get(`/admin/marketing/campaigns/${id}/funnel`)
}

export const getCampaignEvents = (id: number): Promise<MarketingEvent[]> => {
  return request.get(`/admin/marketing/campaigns/${id}/events`)
}
