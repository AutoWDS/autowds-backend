import { useEffect, useState, useRef } from 'react'
import { useSearchParams } from 'react-router-dom'
import ReactQuill from 'react-quill'
import 'react-quill/dist/quill.snow.css'
import { Table, Button, Space, Modal, Form, Input, message, InputNumber, Tag, Tooltip, Select, Dropdown, Switch } from 'antd'
import { PlusOutlined, EditOutlined, DeleteOutlined, DollarOutlined, CopyOutlined, CrownOutlined, MoreOutlined, SendOutlined } from '@ant-design/icons'
import type { MenuProps } from 'antd'
import { getUserList, createUser, updateUser, deleteUser, adjustUserCredits, updateUserEdition, sendMarketingEmail } from '@/api/user'
import type { User, AdjustCreditsData, UpdateUserEditionData } from '@/api/user'

// 版本等级选项
const editionOptions = [
  { label: 'L0 - 免费版', value: 'L0', color: 'default' },
  { label: 'L1 - 个人版', value: 'L1', color: 'blue' },
  { label: 'L2 - 团队版', value: 'L2', color: 'green' },
  { label: 'L3 - 企业版', value: 'L3', color: 'gold' },
]

const getEditionConfig = (edition: string) => {
  return editionOptions.find(opt => opt.value === edition) || editionOptions[0]
}

const UserList = () => {
  const [users, setUsers] = useState<User[]>([])
  const [total, setTotal] = useState(0)
  const [page, setPage] = useState(1)
  const [pageSize, setPageSize] = useState(20)
  const [searchInput, setSearchInput] = useState('')
  const [keyword, setKeyword] = useState('')
  const [loading, setLoading] = useState(false)
  const [modalVisible, setModalVisible] = useState(false)
  const [creditsModalVisible, setCreditsModalVisible] = useState(false)
  const [editionModalVisible, setEditionModalVisible] = useState(false)
  const [editingUser, setEditingUser] = useState<User | null>(null)
  const [adjustingUser, setAdjustingUser] = useState<User | null>(null)
  const [editionUser, setEditionUser] = useState<User | null>(null)
  const [form] = Form.useForm()
  const [creditsForm] = Form.useForm()
  const [editionForm] = Form.useForm()
  const [marketingModalVisible, setMarketingModalVisible] = useState(false)
  const [marketingForm] = Form.useForm()
  const [marketingUserOptions, setMarketingUserOptions] = useState<
    { label: string; value: number; disabled?: boolean }[]
  >([])
  const [marketingOptionsLoading, setMarketingOptionsLoading] = useState(false)
  const [marketingSending, setMarketingSending] = useState(false)
  const marketingSearchTimer = useRef<ReturnType<typeof setTimeout> | null>(null)
  const [searchParams] = useSearchParams()
  const queryUserId = searchParams.get('userId')

  useEffect(() => {
    loadUsers()
  }, [page, pageSize, keyword])

  // URL 带 userId 时，直接按 ID 查询该用户
  useEffect(() => {
    if (queryUserId) {
      setSearchInput(queryUserId)
      setKeyword('')
      setPage(1)
    }
  }, [queryUserId])

  const loadUsers = async () => {
    setLoading(true)
    try {
      const params: Parameters<typeof getUserList>[0] = {
        page,
        size: pageSize,
      }
      const k = keyword.trim()
      if (k) {
        params.keyword = k
      }
      // URL 带 userId 时，直接按 ID 精确查询
      if (queryUserId) {
        params.user_id = Number(queryUserId)
      }
      const data = await getUserList(params)
      setUsers(data.content || [])
      setTotal(Number(data.total_elements) || 0)
    } catch (error) {
      console.error('加载用户列表失败:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleSearch = (value: string) => {
    setKeyword(value.trim())
    setPage(1)
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
    setEditingUser(null)
    form.resetFields()
    setModalVisible(true)
  }

  const loadMarketingUserOptions = async (keyword: string) => {
    setMarketingOptionsLoading(true)
    try {
      const k = keyword.trim()
      const data = await getUserList({
        page: 1,
        size: 500,
        ...(k ? { keyword: k } : {}),
      })
      setMarketingUserOptions(
        (data.content || []).map((u) => ({
          label: `[${u.id}] ${u.username} — ${u.email}${u.email_subscribed ? '' : '（已退订）'}`,
          value: u.id,
          disabled: !u.email_subscribed,
        }))
      )
    } catch (error) {
      console.error('加载收件人列表失败:', error)
    } finally {
      setMarketingOptionsLoading(false)
    }
  }

  const openMarketingModal = () => {
    marketingForm.resetFields()
    setMarketingModalVisible(true)
    loadMarketingUserOptions('')
  }

  const onMarketingUserSearch = (kw: string) => {
    if (marketingSearchTimer.current) {
      clearTimeout(marketingSearchTimer.current)
    }
    marketingSearchTimer.current = setTimeout(() => {
      loadMarketingUserOptions(kw)
    }, 300)
  }

  const handleMarketingSubmit = async () => {
    try {
      const values = await marketingForm.validateFields()
      setMarketingSending(true)
      const res = await sendMarketingEmail({
        subject: values.subject,
        html_body: values.html_body,
        user_ids: values.user_ids,
      })
      const parts = [
        `成功 ${res.sent} 封`,
        `跳过已退订 ${res.skipped_unsubscribed}`,
        `ID 不存在 ${res.not_found}`,
      ]
      if (res.failed.length) {
        parts.push(`失败 ${res.failed.length} 封`)
      }
      message.success(parts.join('；'))
      if (res.failed.length) {
        message.warning(res.failed.slice(0, 5).join('；'), 8)
      }
      setMarketingModalVisible(false)
      loadUsers()
    } catch (error) {
      console.error('发送营销邮件失败:', error)
    } finally {
      setMarketingSending(false)
    }
  }

  const handleEdit = (record: User) => {
    setEditingUser(record)
    form.setFieldsValue({
      ...record,
      email_subscribed: record.email_subscribed ?? true,
    })
    setModalVisible(true)
  }

  const handleDelete = async (id: number) => {
    try {
      await deleteUser(id)
      message.success('删除成功')
      loadUsers()
    } catch (error) {
      console.error('删除失败:', error)
    }
  }

  const handleAdjustCredits = (record: User) => {
    setAdjustingUser(record)
    creditsForm.resetFields()
    setCreditsModalVisible(true)
  }

  const handleCreditsSubmit = async () => {
    try {
      const values = await creditsForm.validateFields()
      if (adjustingUser) {
        await adjustUserCredits(adjustingUser.id, values as AdjustCreditsData)
        message.success('积分调整成功')
        setCreditsModalVisible(false)
        loadUsers()
      }
    } catch (error) {
      console.error('积分调整失败:', error)
    }
  }

  const copyInviteCode = (inviteCode: string) => {
    navigator.clipboard.writeText(inviteCode)
    message.success('邀请码已复制到剪贴板')
  }

  const handleUpdateEdition = (record: User) => {
    setEditionUser(record)
    editionForm.setFieldsValue({ edition: record.edition })
    editionForm.resetFields(['description'])
    setEditionModalVisible(true)
  }

  const handleEditionSubmit = async () => {
    try {
      const values = await editionForm.validateFields()
      if (editionUser) {
        await updateUserEdition(editionUser.id, values as UpdateUserEditionData)
        message.success('用户版本等级更新成功')
        setEditionModalVisible(false)
        loadUsers()
      }
    } catch (error) {
      console.error('版本等级更新失败:', error)
    }
  }

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields()
      if (editingUser) {
        await updateUser(editingUser.id, values)
        message.success('更新成功')
      } else {
        await createUser(values)
        message.success('创建成功')
      }
      setModalVisible(false)
      loadUsers()
    } catch (error) {
      console.error('提交失败:', error)
    }
  }

  const columns = [
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
      width: 80,
    },
    {
      title: '用户名',
      dataIndex: 'username',
      key: 'username',
      width: 120,
    },
    {
      title: '邮箱',
      dataIndex: 'email',
      key: 'email',
      width: 200,
    },
    {
      title: '积分',
      dataIndex: 'credits',
      key: 'credits',
      width: 100,
      render: (credits: number) => (
        <Tag color={credits > 0 ? 'green' : 'red'}>
          {credits}
        </Tag>
      ),
    },
    {
      title: '邀请码',
      dataIndex: 'invite_code',
      key: 'invite_code',
      width: 150,
      render: (inviteCode: string) => (
        <Space>
          <code style={{ fontSize: '12px' }}>{inviteCode}</code>
          <Tooltip title="复制邀请码">
            <Button 
              type="text" 
              size="small" 
              icon={<CopyOutlined />}
              onClick={() => copyInviteCode(inviteCode)}
            />
          </Tooltip>
        </Space>
      ),
    },
    {
      title: '版本',
      dataIndex: 'edition',
      key: 'edition',
      width: 120,
      render: (edition: string) => {
        const config = getEditionConfig(edition)
        return (
          <Tag color={config.color}>
            {config.label}
          </Tag>
        )
      },
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      width: 80,
      render: (status: string) => (
        <Tag color={status === 'active' ? 'green' : 'red'}>
          {status === 'active' ? '正常' : '锁定'}
        </Tag>
      ),
    },
    {
      title: '营销邮件',
      key: 'email_subscribed',
      width: 100,
      render: (_: unknown, record: User) => (
        <Tag color={record.email_subscribed !== false ? 'green' : 'default'}>
          {record.email_subscribed !== false ? '订阅' : '已退订'}
        </Tag>
      ),
    },
    {
      title: '创建时间',
      dataIndex: 'created_at',
      key: 'created_at',
      width: 160,
    },
    {
      title: '操作',
      key: 'action',
      width: 120,
      render: (_: any, record: User) => {
        const menuItems: MenuProps['items'] = [
          {
            key: 'edit',
            label: '编辑用户',
            icon: <EditOutlined />,
            onClick: () => handleEdit(record),
          },
          {
            key: 'credits',
            label: '调整积分',
            icon: <DollarOutlined />,
            onClick: () => handleAdjustCredits(record),
          },
          {
            key: 'edition',
            label: '修改版本',
            icon: <CrownOutlined />,
            onClick: () => handleUpdateEdition(record),
          },
          {
            type: 'divider',
          },
          {
            key: 'delete',
            label: '删除用户',
            icon: <DeleteOutlined />,
            danger: true,
            onClick: () => {
              Modal.confirm({
                title: '确认删除',
                content: `确定要删除用户 "${record.username}" 吗？此操作不可恢复。`,
                okText: '确定删除',
                okType: 'danger',
                cancelText: '取消',
                onOk: () => handleDelete(record.id),
              })
            },
          },
        ]

        return (
          <Dropdown
            menu={{ items: menuItems }}
            trigger={['click']}
            placement="bottomRight"
          >
            <Button type="text" icon={<MoreOutlined />} />
          </Dropdown>
        )
      },
    },
  ]

  return (
    <div>
      <div style={{ marginBottom: 16, display: 'flex', gap: 16, alignItems: 'center', flexWrap: 'wrap' }}>
        <Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>
          新增用户
        </Button>
        <Button icon={<SendOutlined />} onClick={openMarketingModal}>
          发送营销邮件
        </Button>
        <Input.Search
          placeholder="搜索用户名或邮箱"
          value={searchInput}
          onChange={(e) => setSearchInput(e.target.value)}
          onSearch={handleSearch}
          allowClear
          style={{ width: 280 }}
          onClear={() => {
            setSearchInput('')
            setKeyword('')
            setPage(1)
          }}
        />
      </div>
      <Table
        columns={columns}
        dataSource={users}
        rowKey="id"
        loading={loading}
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
        title={editingUser ? '编辑用户' : '新增用户'}
        open={modalVisible}
        onOk={handleSubmit}
        onCancel={() => setModalVisible(false)}
      >
        <Form form={form} layout="vertical">
          <Form.Item name="username" label="用户名" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="email" label="邮箱" rules={[{ required: true, type: 'email' }]}>
            <Input />
          </Form.Item>
          {!editingUser && (
            <Form.Item name="password" label="密码" rules={[{ required: true }]}>
              <Input.Password />
            </Form.Item>
          )}
          {editingUser && (
            <Form.Item
              name="email_subscribed"
              label="接收营销邮件"
              valuePropName="checked"
            >
              <Switch checkedChildren="订阅" unCheckedChildren="已退订" />
            </Form.Item>
          )}
        </Form>
      </Modal>

      <Modal
        title="发送营销邮件"
        open={marketingModalVisible}
        onOk={handleMarketingSubmit}
        onCancel={() => setMarketingModalVisible(false)}
        confirmLoading={marketingSending}
        width={920}
        destroyOnClose
      >
        <Form form={marketingForm} layout="vertical">
          <Form.Item
            name="subject"
            label="邮件主题"
            rules={[{ required: true, message: '请输入主题' }]}
          >
            <Input placeholder="邮件标题" maxLength={200} showCount />
          </Form.Item>
          <Form.Item
            name="html_body"
            label="正文（HTML）"
            rules={[{ required: true, message: '请输入正文' }]}
            getValueFromEvent={(content: string) => content}
          >
            <ReactQuill theme="snow" style={{ minHeight: 220 }} placeholder="编辑邮件内容" />
          </Form.Item>
          <Form.Item
            name="user_ids"
            label="收件用户"
            rules={[{ required: true, message: '请选择至少一位用户' }]}
          >
            <Select
              mode="multiple"
              placeholder="搜索并选择用户（已退订用户不可选）"
              options={marketingUserOptions}
              loading={marketingOptionsLoading}
              showSearch
              filterOption={false}
              onSearch={onMarketingUserSearch}
              optionFilterProp="label"
              style={{ width: '100%' }}
            />
          </Form.Item>
          <p style={{ margin: 0, fontSize: 12, color: '#888' }}>
            邮件底部将自动附加退订链接；需在服务端配置 PUBLIC_BASE_URL（对应 email.public_base_url）。
          </p>
        </Form>
      </Modal>

      <Modal
        title={`调整积分 - ${adjustingUser?.username}`}
        open={creditsModalVisible}
        onOk={handleCreditsSubmit}
        onCancel={() => setCreditsModalVisible(false)}
      >
        <Form form={creditsForm} layout="vertical">
          <div style={{ marginBottom: 16, padding: 12, backgroundColor: '#f5f5f5', borderRadius: 4 }}>
            <p><strong>当前积分：</strong>{adjustingUser?.credits}</p>
            <p style={{ margin: 0, fontSize: '12px', color: '#666' }}>
              输入正数增加积分，输入负数扣减积分
            </p>
          </div>
          <Form.Item 
            name="amount" 
            label="调整数量" 
            rules={[
              { required: true, message: '请输入调整数量' },
              { type: 'number', message: '请输入有效数字' }
            ]}
          >
            <InputNumber 
              style={{ width: '100%' }}
              placeholder="正数增加，负数扣减"
              min={-999999}
              max={999999}
            />
          </Form.Item>
          <Form.Item 
            name="description" 
            label="调整说明" 
            rules={[{ required: true, message: '请输入调整说明' }]}
          >
            <Input.TextArea 
              rows={3}
              placeholder="请输入积分调整的原因说明"
            />
          </Form.Item>
        </Form>
      </Modal>

      <Modal
        title={`修改版本等级 - ${editionUser?.username}`}
        open={editionModalVisible}
        onOk={handleEditionSubmit}
        onCancel={() => setEditionModalVisible(false)}
      >
        <Form form={editionForm} layout="vertical">
          <div style={{ marginBottom: 16, padding: 12, backgroundColor: '#f5f5f5', borderRadius: 4 }}>
            <p><strong>当前版本：</strong>{editionUser && getEditionConfig(editionUser.edition).label}</p>
            <p style={{ margin: 0, fontSize: '12px', color: '#666' }}>
              版本等级决定用户可使用的功能范围和权限
            </p>
          </div>
          <Form.Item 
            name="edition" 
            label="新版本等级" 
            rules={[{ required: true, message: '请选择版本等级' }]}
          >
            <Select
              placeholder="请选择版本等级"
              options={editionOptions.map(opt => ({
                label: (
                  <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                    <Tag color={opt.color}>{opt.value}</Tag>
                    <span>{opt.label}</span>
                  </div>
                ),
                value: opt.value
              }))}
            />
          </Form.Item>
          <Form.Item 
            name="description" 
            label="变更说明" 
            rules={[{ required: true, message: '请输入变更说明' }]}
          >
            <Input.TextArea 
              rows={3}
              placeholder="请输入版本等级变更的原因说明"
            />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  )
}

export default UserList
