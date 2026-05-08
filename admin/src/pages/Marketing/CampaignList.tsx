import { useEffect, useState } from 'react'
import { Button, Form, Input, Modal, Select, Space, Table, Tag, message } from 'antd'
import { EyeOutlined, PlayCircleOutlined, PlusOutlined } from '@ant-design/icons'
import { useNavigate } from 'react-router-dom'
import {
  createMarketingCampaign,
  getMarketingCampaigns,
  getMarketingLeads,
  sendMarketingCampaign,
  type MarketingCampaign,
} from '@/api/marketing'

const statusText: Record<string, { text: string; color: string }> = {
  draft: { text: '草稿', color: 'default' },
  sending: { text: '发送中', color: 'processing' },
  completed: { text: '已完成', color: 'success' },
  failed: { text: '失败', color: 'error' },
}

const CampaignList = () => {
  const [campaigns, setCampaigns] = useState<MarketingCampaign[]>([])
  const [total, setTotal] = useState(0)
  const [page, setPage] = useState(1)
  const [pageSize, setPageSize] = useState(20)
  const [loading, setLoading] = useState(false)
  const [modalVisible, setModalVisible] = useState(false)
  const [leadOptions, setLeadOptions] = useState<{ label: string; value: number }[]>([])
  const [form] = Form.useForm()
  const navigate = useNavigate()

  useEffect(() => {
    loadCampaigns()
  }, [page, pageSize])

  const loadCampaigns = async () => {
    setLoading(true)
    try {
      const data = await getMarketingCampaigns({ page, size: pageSize })
      setCampaigns(data.content || [])
      setTotal(Number(data.total_elements) || 0)
    } finally {
      setLoading(false)
    }
  }

  const loadLeadOptions = async (keyword = '') => {
    const data = await getMarketingLeads({ page: 1, size: 500, keyword: keyword || undefined })
    setLeadOptions(
      (data.content || [])
        .filter((lead) => !lead.unsubscribed)
        .map((lead) => ({ label: `[${lead.id}] ${lead.email}`, value: lead.id }))
    )
  }

  const openCreateModal = () => {
    form.resetFields()
    setModalVisible(true)
    loadLeadOptions()
  }

  const handleCreate = async () => {
    const values = await form.validateFields()
    await createMarketingCampaign(values)
    message.success('活动创建成功')
    setModalVisible(false)
    loadCampaigns()
  }

  const handleSend = async (campaign: MarketingCampaign) => {
    const res = await sendMarketingCampaign(campaign.id)
    message.success(`已创建腾讯云 SES 发送任务：${res.task_id}`)
    loadCampaigns()
  }

  const columns = [
    { title: 'ID', dataIndex: 'id', width: 80 },
    { title: '活动名称', dataIndex: 'name' },
    { title: '邮件主题', dataIndex: 'subject' },
    { title: '收件人数', dataIndex: 'delivery_count', width: 100 },
    {
      title: '状态',
      dataIndex: 'status',
      width: 100,
      render: (value: string) => {
        const config = statusText[value] || { text: value, color: 'default' }
        return <Tag color={config.color}>{config.text}</Tag>
      },
    },
    { title: 'SES 任务', dataIndex: 'provider_task_id', width: 140 },
    { title: '创建时间', dataIndex: 'created_at', width: 180 },
    {
      title: '操作',
      width: 180,
      render: (_: unknown, record: MarketingCampaign) => (
        <Space>
          <Button type="link" icon={<EyeOutlined />} onClick={() => navigate(`/marketing/${record.id}`)}>
            详情
          </Button>
          {!record.provider_task_id && (
            <Button type="link" icon={<PlayCircleOutlined />} onClick={() => handleSend(record)}>
              发送
            </Button>
          )}
        </Space>
      ),
    },
  ]

  return (
    <div>
      <Space style={{ marginBottom: 16 }}>
        <Button type="primary" icon={<PlusOutlined />} onClick={openCreateModal}>
          创建活动
        </Button>
      </Space>
      <Table
        rowKey="id"
        loading={loading}
        dataSource={campaigns}
        columns={columns}
        pagination={{
          current: page,
          pageSize,
          total,
          onChange: (p, ps) => {
            setPage(p)
            setPageSize(ps)
          },
        }}
      />
      <Modal
        title="创建营销活动"
        open={modalVisible}
        onOk={handleCreate}
        onCancel={() => setModalVisible(false)}
        width={720}
      >
        <Form form={form} layout="vertical">
          <Form.Item name="name" label="活动名称" rules={[{ required: true, message: '请输入活动名称' }]}>
            <Input />
          </Form.Item>
          <Form.Item name="subject" label="邮件主题" rules={[{ required: true, message: '请输入邮件主题' }]}>
            <Input />
          </Form.Item>
          <Form.Item name="landing_url" label="Landing URL" rules={[{ required: true, message: '请输入 Landing URL' }]}>
            <Input placeholder="https://www.autowds.com/landing" />
          </Form.Item>
          <Form.Item
            name="provider_template_id"
            label="腾讯云 SES 模板 ID"
            rules={[{ required: true, message: '请输入腾讯云 SES 模板 ID' }]}
          >
            <Input />
          </Form.Item>
          <Form.Item name="lead_ids" label="收件人线索" rules={[{ required: true, message: '请选择收件人' }]}>
            <Select
              mode="multiple"
              showSearch
              filterOption={false}
              options={leadOptions}
              onSearch={loadLeadOptions}
              placeholder="搜索并选择可触达线索"
            />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  )
}

export default CampaignList
