import { useEffect, useState } from 'react'
import { Table, Button, Space, Modal, Form, Input, message, Popconfirm, Select, Upload, Tooltip } from 'antd'
import { PlusOutlined, EditOutlined, DeleteOutlined, UploadOutlined } from '@ant-design/icons'
import { getTemplateList, createTemplate, updateTemplate, deleteTemplate, TemplateTopic, ProductEdition } from '@/api/template'
import type { Template } from '@/api/template'

const { Option } = Select

const TemplateList = () => {
  const [templates, setTemplates] = useState<Template[]>([])
  const [total, setTotal] = useState(0)
  const [page, setPage] = useState(1)
  const [pageSize, setPageSize] = useState(20)
  const [loading, setLoading] = useState(false)
  const [modalVisible, setModalVisible] = useState(false)
  const [editingTemplate, setEditingTemplate] = useState<Template | null>(null)
  const [ruleValue, setRuleValue] = useState('')
  const [paramsValue, setParamsValue] = useState('')
  const [form] = Form.useForm()

  // 主题选项
  const topicOptions = [
    { value: TemplateTopic.Bidding, label: '招投标' },
    { value: TemplateTopic.ECommerce, label: '电商' },
    { value: TemplateTopic.LocalLife, label: '本地生活' },
    { value: TemplateTopic.Media, label: '媒体' },
    { value: TemplateTopic.ResearchEducation, label: '科研教育' },
    { value: TemplateTopic.SearchEngine, label: '搜索引擎' },
    { value: TemplateTopic.SocialNetwork, label: '社交网络' },
    { value: TemplateTopic.Other, label: '其他' },
  ]

  // 版本选项
  const editionOptions = [
    { value: ProductEdition.L0, label: 'L0' },
    { value: ProductEdition.L1, label: 'L1' },
    { value: ProductEdition.L2, label: 'L2' },
    { value: ProductEdition.L3, label: 'L3' },
  ]

  // 语言选项
  const langOptions = [
    { value: 'zh', label: '中文' },
    { value: 'en', label: 'English' },
  ]

  // 处理文件导入
  const handleFileImport = (file: File) => {
    const reader = new FileReader()
    reader.onload = (e) => {
      try {
        const content = e.target?.result as string
        // 验证是否为有效的JSON
        JSON.parse(content)
        // 格式化JSON并设置到表单
        const formattedJson = JSON.stringify(JSON.parse(content), null, 2)
        form.setFieldValue('rule', formattedJson)
        setRuleValue(formattedJson)
        message.success('文件导入成功')
      } catch (error) {
        message.error('文件格式错误，请确保是有效的JSON文件')
      }
    }
    reader.readAsText(file)
    return false // 阻止默认上传行为
  }

  useEffect(() => {
    loadTemplates()
  }, [page, pageSize])

  const loadTemplates = async () => {
    setLoading(true)
    try {
      const data = await getTemplateList({ page, size: pageSize })
      setTemplates(data.content || [])
      setTotal(Number(data.total_elements) || 0)
    } catch (error) {
      console.error('加载模板列表失败:', error)
      message.error('加载模板列表失败，请重试')
    } finally {
      setLoading(false)
    }
  }

  const handleTableChange = (p: number, ps: number) => {
    if (ps !== pageSize) {
      setPage(1)
    } else {
      setPage(p)
    }
    setPageSize(ps)
  }

  const handleAdd = () => {
    setEditingTemplate(null)
    form.resetFields()
    setRuleValue('')
    setParamsValue('')
    // 设置默认值
    form.setFieldsValue({
      topic: TemplateTopic.Other,
      edition: ProductEdition.L0,
      lang: 'zh'
    })
    setModalVisible(true)
  }

  const handleEdit = (record: Template) => {
    setEditingTemplate(record)
    
    // 序列化rule和params
    const ruleStr = record.rule !== null && record.rule !== undefined ? JSON.stringify(record.rule, null, 2) : ''
    const paramsStr = record.params !== null && record.params !== undefined ? JSON.stringify(record.params, null, 2) : ''
    
    // 设置基本字段
    form.setFieldsValue({
      name: record.name,
      description: record.description,
      topic: record.topic,
      edition: record.edition,
      lang: record.lang,
      img: record.img
    })
    
    // 设置JSON字段
    form.setFieldValue('rule', ruleStr)
    form.setFieldValue('params', paramsStr)
    setRuleValue(ruleStr)
    setParamsValue(paramsStr)
    
    setModalVisible(true)
  }

  const handleDelete = async (id: number) => {
    try {
      await deleteTemplate(id)
      message.success('删除成功')
      loadTemplates()
    } catch (error) {
      console.error('删除失败:', error)
      message.error('删除失败，请重试')
    }
  }

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields()
      
      // 处理 JSON 字段
      const processedValues = {
        ...values,
        rule: values.rule ? JSON.parse(values.rule) : undefined,
        params: values.params ? JSON.parse(values.params) : undefined
      }
      
      if (editingTemplate) {
        await updateTemplate(editingTemplate.id, processedValues)
        message.success('更新成功')
      } else {
        await createTemplate(processedValues)
        message.success('创建成功')
      }
      setModalVisible(false)
      loadTemplates()
    } catch (error) {
      if (error instanceof SyntaxError) {
        message.error('JSON格式错误，请检查规则和参数字段')
      } else {
        console.error('提交失败:', error)
        message.error('提交失败')
      }
    }
  }

  const columns = [
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
      width: 60,
    },
    {
      title: '模板名称',
      dataIndex: 'name',
      key: 'name',
      width: 150,
      ellipsis: true,
      render: (text: string) => (
        <Tooltip title={text} placement="topLeft">
          <span>{text}</span>
        </Tooltip>
      ),
    },
    {
      title: '描述',
      dataIndex: 'description',
      key: 'description',
      width: 200,
      ellipsis: true,
      render: (text: string) => (
        <Tooltip title={text} placement="topLeft">
          <span>{text}</span>
        </Tooltip>
      ),
    },
    {
      title: '主题',
      dataIndex: 'topic',
      key: 'topic',
      width: 100,
      render: (topic: TemplateTopic) => {
        const topicOption = topicOptions.find(opt => opt.value === topic)
        return topicOption ? topicOption.label : topic
      },
    },
    {
      title: '版本',
      dataIndex: 'edition',
      key: 'edition',
      width: 60,
      render: (edition: ProductEdition) => edition,
    },
    {
      title: '语言',
      dataIndex: 'lang',
      key: 'lang',
      width: 60,
      render: (lang: string) => {
        const langOption = langOptions.find(opt => opt.value === lang)
        return langOption ? langOption.label : lang
      },
    },
    {
      title: '创建时间',
      dataIndex: 'created_at',
      key: 'created_at',
      width: 150,
    },
    {
      title: '操作',
      key: 'action',
      width: 120,
      fixed: 'right' as const,
      render: (_: any, record: Template) => (
        <Space>
          <Button type="link" icon={<EditOutlined />} onClick={() => handleEdit(record)} size="small">
            编辑
          </Button>
          <Popconfirm title="确定删除吗？" onConfirm={() => handleDelete(record.id)}>
            <Button type="link" danger icon={<DeleteOutlined />} size="small">
              删除
            </Button>
          </Popconfirm>
        </Space>
      ),
    },
  ]

  return (
    <div>
      <div style={{ marginBottom: 16, display: 'flex', gap: '16px', alignItems: 'center' }}>
        <Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>
          新增模板
        </Button>
      </div>
      <Table
        columns={columns}
        dataSource={templates}
        rowKey="id"
        loading={loading}
        scroll={{ x: 900 }}
        pagination={{
          current: page,
          pageSize,
          total,
          showSizeChanger: true,
          showQuickJumper: true,
          showTotal: (t, range) => `第 ${range[0]}-${range[1]} 条，共 ${t} 条`,
          onChange: handleTableChange,
        }}
      />
      <Modal
        title={editingTemplate ? '编辑模板' : '新增模板'}
        open={modalVisible}
        onOk={handleSubmit}
        onCancel={() => setModalVisible(false)}
        width={900}
        style={{ top: 20 }}
      >
        <Form form={form} layout="vertical">
          <div style={{ display: 'flex', gap: '16px' }}>
            <div style={{ flex: 1 }}>
              <Form.Item 
                name="name" 
                label="模板名称" 
                rules={[
                  { required: true, message: '请输入模板名称' },
                  { min: 1, max: 100, message: '模板名称长度必须在1-100字符之间' }
                ]}
              >
                <Input />
              </Form.Item>
              <Form.Item 
                name="topic" 
                label="主题" 
                rules={[{ required: true, message: '请选择主题' }]}
              >
                <Select placeholder="请选择主题">
                  {topicOptions.map(option => (
                    <Option key={option.value} value={option.value}>
                      {option.label}
                    </Option>
                  ))}
                </Select>
              </Form.Item>
            </div>
            <div style={{ flex: 1 }}>
              <Form.Item 
                name="lang" 
                label="语言" 
                rules={[
                  { required: true, message: '请选择语言' },
                  { min: 2, max: 10, message: '语言代码长度必须在2-10字符之间' }
                ]}
              >
                <Select placeholder="请选择语言">
                  {langOptions.map(option => (
                    <Option key={option.value} value={option.value}>
                      {option.label}
                    </Option>
                  ))}
                </Select>
              </Form.Item>
              <Form.Item 
                name="edition" 
                label="版本" 
                rules={[{ required: true, message: '请选择版本' }]}
              >
                <Select placeholder="请选择版本">
                  {editionOptions.map(option => (
                    <Option key={option.value} value={option.value}>
                      {option.label}
                    </Option>
                  ))}
                </Select>
              </Form.Item>
            </div>
          </div>
          
          <Form.Item 
            name="img" 
            label="图片URL"
            rules={[
              { type: 'url', message: '请输入有效的URL' }
            ]}
          >
            <Input placeholder="模板图片URL" />
          </Form.Item>
          
          <Form.Item 
            name="description" 
            label="描述"
            rules={[
              { max: 500, message: '描述长度不能超过500字符' }
            ]}
          >
            <Input.TextArea rows={2} />
          </Form.Item>
          
          <div style={{ display: 'flex', gap: '16px' }}>
            <Form.Item 
              name="rule" 
              label="规则"
              style={{ flex: 1 }}
              rules={[
                {
                  validator: (_, value) => {
                    if (!value) return Promise.resolve()
                    try {
                      JSON.parse(value)
                      return Promise.resolve()
                    } catch {
                      return Promise.reject(new Error('请输入有效的JSON格式'))
                    }
                  }
                }
              ]}
            >
              <div style={{ position: 'relative' }}>
                <Input.TextArea 
                  rows={4} 
                  placeholder="JSON格式规则"
                  value={ruleValue}
                  onChange={(e) => {
                    setRuleValue(e.target.value)
                    form.setFieldValue('rule', e.target.value)
                  }}
                />
                <Upload
                  beforeUpload={handleFileImport}
                  showUploadList={false}
                  accept=".json"
                  style={{
                    position: 'absolute',
                    bottom: 8,
                    right: 8,
                    zIndex: 10
                  }}
                >
                  <Button 
                    icon={<UploadOutlined />} 
                    size="small"
                    type="text"
                    style={{
                      backgroundColor: 'rgba(255, 255, 255, 0.9)',
                      border: '1px solid #d9d9d9',
                      borderRadius: '4px',
                      boxShadow: '0 2px 4px rgba(0, 0, 0, 0.1)'
                    }}
                  >
                    导入
                  </Button>
                </Upload>
              </div>
            </Form.Item>
            <Form.Item 
              name="params" 
              label="参数"
              style={{ flex: 1 }}
              rules={[
                {
                  validator: (_, value) => {
                    if (!value) return Promise.resolve()
                    try {
                      JSON.parse(value)
                      return Promise.resolve()
                    } catch {
                      return Promise.reject(new Error('请输入有效的JSON格式'))
                    }
                  }
                }
              ]}
            >
              <Input.TextArea 
                rows={4} 
                placeholder="JSON格式参数"
                value={paramsValue}
                onChange={(e) => {
                  setParamsValue(e.target.value)
                  form.setFieldValue('params', e.target.value)
                }}
              />
            </Form.Item>
          </div>
        </Form>
      </Modal>
    </div>
  )
}

export default TemplateList
