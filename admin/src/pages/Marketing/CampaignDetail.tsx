import { useEffect, useState } from 'react'
import { Card, Col, Row, Statistic, Table, Tag } from 'antd'
import { useParams } from 'react-router-dom'
import { getCampaignEvents, getCampaignFunnel, type CampaignFunnel, type MarketingEvent } from '@/api/marketing'

const metricText: Record<string, string> = {
  delivered: '到达',
  opened: '打开',
  email_link_clicked: '邮件点击',
  landing_opened: 'Landing 打开',
  plugin_installed: '插件安装',
  registered: '注册',
  purchased: '购买',
}

const CampaignDetail = () => {
  const { id } = useParams()
  const campaignId = Number(id)
  const [funnel, setFunnel] = useState<CampaignFunnel>()
  const [events, setEvents] = useState<MarketingEvent[]>([])

  useEffect(() => {
    if (campaignId) {
      loadDetail()
    }
  }, [campaignId])

  const loadDetail = async () => {
    const [funnelData, eventData] = await Promise.all([
      getCampaignFunnel(campaignId),
      getCampaignEvents(campaignId),
    ])
    setFunnel(funnelData)
    setEvents(eventData)
  }

  const columns = [
    { title: 'ID', dataIndex: 'id', width: 80 },
    {
      title: '事件',
      dataIndex: 'event_type',
      width: 160,
      render: (value: string) => <Tag>{metricText[value] || value}</Tag>,
    },
    { title: '投递 ID', dataIndex: 'delivery_id', width: 120 },
    { title: '线索 ID', dataIndex: 'lead_id', width: 120 },
    { title: 'URL', dataIndex: 'url', ellipsis: true },
    { title: '时间', dataIndex: 'created_at', width: 180 },
  ]

  return (
    <div>
      <h2>活动漏斗</h2>
      <Row gutter={16} style={{ marginBottom: 24 }}>
        <Col span={4}>
          <Card>
            <Statistic title="发送人数" value={funnel?.sent || 0} />
          </Card>
        </Col>
        {(funnel?.metrics || []).map((metric) => (
          <Col span={4} key={metric.event_type}>
            <Card>
              <Statistic
                title={metricText[metric.event_type] || metric.event_type}
                value={metric.count}
                suffix={`${(metric.rate * 100).toFixed(1)}%`}
              />
            </Card>
          </Col>
        ))}
      </Row>
      <h2>事件明细</h2>
      <Table rowKey="id" dataSource={events} columns={columns} pagination={{ pageSize: 20 }} />
    </div>
  )
}

export default CampaignDetail
