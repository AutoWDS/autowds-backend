import { useEffect, useState } from 'react'
import { Button, Form, Input, Modal, Space, Switch, Table, Tag, Upload, message } from 'antd'
import { InboxOutlined } from '@ant-design/icons'
import type { UploadProps } from 'antd'
import { getMarketingLeads, importMarketingLeads, type MarketingLead } from '@/api/marketing'

const LeadList = () => {
  const [leads, setLeads] = useState<MarketingLead[]>([])
  const [total, setTotal] = useState(0)
  const [page, setPage] = useState(1)
  const [pageSize, setPageSize] = useState(20)
  const [keyword, setKeyword] = useState('')
  const [loading, setLoading] = useState(false)
  const [importVisible, setImportVisible] = useState(false)
  const [csvContent, setCsvContent] = useState('')
  const [importForm] = Form.useForm()

  useEffect(() => {
    loadLeads()
  }, [page, pageSize, keyword])

  const loadLeads = async () => {
    setLoading(true)
    try {
      const data = await getMarketingLeads({ page, size: pageSize, keyword: keyword || undefined })
      setLeads(data.content || [])
      setTotal(Number(data.total_elements) || 0)
    } finally {
      setLoading(false)
    }
  }

  const uploadProps: UploadProps = {
    accept: '.csv,text/csv',
    maxCount: 1,
    beforeUpload: (file) => {
      const reader = new FileReader()
      reader.onload = () => {
        setCsvContent(String(reader.result || ''))
      }
      reader.readAsText(file)
      return false
    },
  }

  const handleImport = async () => {
    const values = await importForm.validateFields()
    if (!csvContent.trim()) {
      message.warning('请先选择 CSV 文件')
      return
    }
    const res = await importMarketingLeads({
      csv_content: csvContent,
      source: values.source,
    })
    message.success(`导入成功：新增 ${res.created}，重复 ${res.duplicated}，无效 ${res.invalid}`)
    setImportVisible(false)
    setCsvContent('')
    importForm.resetFields()
    loadLeads()
  }

  const columns = [
    { title: 'ID', dataIndex: 'id', width: 80 },
    { title: '邮箱', dataIndex: 'email' },
    { title: '姓名', dataIndex: 'name', width: 140 },
    { title: '来源', dataIndex: 'source', width: 140 },
    {
      title: '退订',
      dataIndex: 'unsubscribed',
      width: 100,
      render: (value: boolean) => (
        <Tag color={value ? 'default' : 'green'}>{value ? '已退订' : '可触达'}</Tag>
      ),
    },
    { title: '状态', dataIndex: 'status', width: 120 },
    { title: '创建时间', dataIndex: 'created_at', width: 180 },
  ]

  return (
    <div>
      <Space style={{ marginBottom: 16 }}>
        <Input.Search
          placeholder="搜索邮箱或姓名"
          allowClear
          onSearch={(value) => {
            setKeyword(value.trim())
            setPage(1)
          }}
          style={{ width: 280 }}
        />
        <Button type="primary" onClick={() => setImportVisible(true)}>
          导入 CSV
        </Button>
      </Space>
      <Table
        rowKey="id"
        loading={loading}
        dataSource={leads}
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
        title="导入营销线索"
        open={importVisible}
        onOk={handleImport}
        onCancel={() => setImportVisible(false)}
        okText="导入"
      >
        <Form form={importForm} layout="vertical">
          <Form.Item label="来源" name="source">
            <Input placeholder="例如 ProductHunt、展会、合作方" />
          </Form.Item>
          <Upload.Dragger {...uploadProps}>
            <p className="ant-upload-drag-icon">
              <InboxOutlined />
            </p>
            <p>点击或拖拽 CSV 文件到此处</p>
            <p style={{ color: '#999' }}>建议包含 email、name 等列</p>
          </Upload.Dragger>
          <Form.Item label="已读取内容" style={{ marginTop: 16 }}>
            <Switch checked={Boolean(csvContent)} checkedChildren="已读取" unCheckedChildren="未读取" />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  )
}

export default LeadList
