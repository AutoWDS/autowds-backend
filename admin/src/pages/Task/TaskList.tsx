import { useEffect, useState } from 'react'
import { Table, Button, Space, Modal, Form, Input, Select, message, Popconfirm, Tag, Card, Row, Col, Statistic } from 'antd'
import { PlusOutlined, EditOutlined, DeleteOutlined, PlayCircleOutlined, PauseCircleOutlined } from '@ant-design/icons'
import { useNavigate } from 'react-router-dom'
import { getTaskList, createTask, updateTask, deleteTask, startTask, stopTask } from '@/api/task'
import { getTaskStatistics, type TaskStatistics } from '@/api/statistics'
import type { Task } from '@/api/task'

const TaskList = () => {
  const [tasks, setTasks] = useState<Task[]>([])
  const [total, setTotal] = useState(0)
  const [page, setPage] = useState(1)
  const [pageSize, setPageSize] = useState(20)
  const [loading, setLoading] = useState(false)
  const [modalVisible, setModalVisible] = useState(false)
  const [editingTask, setEditingTask] = useState<Task | null>(null)
  const [stats, setStats] = useState<TaskStatistics | null>(null)
  const [statusFilter, setStatusFilter] = useState<string | undefined>(undefined)
  const [form] = Form.useForm()
  const navigate = useNavigate()

  useEffect(() => {
    loadStatistics()
  }, [])

  useEffect(() => {
    loadTasks()
  }, [statusFilter, page, pageSize])

  const loadStatistics = async () => {
    try {
      const data = await getTaskStatistics()
      setStats(data)
    } catch (error) {
      console.error('加载统计数据失败:', error)
    }
  }

  const loadTasks = async () => {
    setLoading(true)
    try {
      const params: Parameters<typeof getTaskList>[0] = {
        page,
        size: pageSize,
      }
      if (statusFilter) {
        params.status = statusFilter
      }
      const data = await getTaskList(params)
      setTasks(data.content || [])
      setTotal(Number(data.total_elements) || 0)
    } catch (error) {
      console.error('加载任务列表失败:', error)
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
    setEditingTask(null)
    form.resetFields()
    setModalVisible(true)
  }

  const handleEdit = (record: Task) => {
    setEditingTask(record)
    form.setFieldsValue(record)
    setModalVisible(true)
  }

  const handleDelete = async (id: number) => {
    try {
      await deleteTask(id)
      message.success('删除成功')
      loadTasks()
    } catch (error) {
      console.error('删除失败:', error)
    }
  }

  const handleStart = async (id: number) => {
    try {
      await startTask(id)
      message.success('任务已启动')
      loadTasks()
      loadStatistics()
    } catch (error) {
      console.error('启动失败:', error)
    }
  }

  const handleStop = async (id: number) => {
    try {
      await stopTask(id)
      message.success('任务已停止')
      loadTasks()
      loadStatistics()
    } catch (error) {
      console.error('停止失败:', error)
    }
  }

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields()
      if (editingTask) {
        await updateTask(editingTask.id, values)
        message.success('更新成功')
      } else {
        await createTask(values)
        message.success('创建成功')
      }
      setModalVisible(false)
      loadTasks()
      loadStatistics()
    } catch (error) {
      console.error('提交失败:', error)
    }
  }

  const getStatusTag = (status: string) => {
    const statusMap: Record<string, { color: string; text: string }> = {
      pending: { color: 'default', text: '待执行' },
      running: { color: 'processing', text: '运行中' },
      completed: { color: 'success', text: '已完成' },
      failed: { color: 'error', text: '失败' },
    }
    const config = statusMap[status] || { color: 'default', text: status }
    return <Tag color={config.color}>{config.text}</Tag>
  }

  const handleUserClick = (userId: number) => {
    navigate(`/users?userId=${userId}`)
  }

  const handleStatusFilter = (status?: string) => {
    setStatusFilter(status)
    setPage(1)
  }

  const clearFilter = () => {
    setStatusFilter(undefined)
    setPage(1)
  }

  const columns = [
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
      width: 80,
    },
    {
      title: '任务名称',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      render: (status: string) => getStatusTag(status),
    },
    {
      title: '创建人',
      dataIndex: 'user_id',
      key: 'user_id',
      width: 150,
      render: (userId: number, record: Task) => (
        <Button 
          type="link" 
          onClick={() => handleUserClick(userId)}
          style={{ padding: 0 }}
        >
          {record.creator_name || `用户${userId}`}
        </Button>
      ),
    },
    {
      title: '创建时间',
      dataIndex: 'created_at',
      key: 'created_at',
    },
    {
      title: '操作',
      key: 'action',
      render: (_: any, record: Task) => (
        <Space>
          {record.status !== 'running' && (
            <Button
              type="link"
              icon={<PlayCircleOutlined />}
              onClick={() => handleStart(record.id)}
            >
              启动
            </Button>
          )}
          {record.status === 'running' && (
            <Button
              type="link"
              icon={<PauseCircleOutlined />}
              onClick={() => handleStop(record.id)}
            >
              停止
            </Button>
          )}
          <Button type="link" icon={<EditOutlined />} onClick={() => handleEdit(record)}>
            编辑
          </Button>
          <Popconfirm title="确定删除吗？" onConfirm={() => handleDelete(record.id)}>
            <Button type="link" danger icon={<DeleteOutlined />}>
              删除
            </Button>
          </Popconfirm>
        </Space>
      ),
    },
  ]

  return (
    <div>
      <h2 style={{ marginBottom: 16 }}>任务统计</h2>
      <Row gutter={16} style={{ marginBottom: 24 }}>
        <Col span={4}>
          <Card 
            hoverable
            onClick={() => handleStatusFilter('pending')}
            style={{ 
              cursor: 'pointer',
              border: statusFilter === 'pending' ? '2px solid #999' : undefined
            }}
          >
            <Statistic
              title="待执行任务"
              value={stats?.pending || 0}
              valueStyle={{ color: '#999' }}
            />
          </Card>
        </Col>
        <Col span={4}>
          <Card 
            hoverable
            onClick={() => handleStatusFilter('running')}
            style={{ 
              cursor: 'pointer',
              border: statusFilter === 'running' ? '2px solid #1890ff' : undefined
            }}
          >
            <Statistic
              title="运行中任务"
              value={stats?.running || 0}
              valueStyle={{ color: '#1890ff' }}
            />
          </Card>
        </Col>
        <Col span={4}>
          <Card 
            hoverable
            onClick={() => handleStatusFilter('completed')}
            style={{ 
              cursor: 'pointer',
              border: statusFilter === 'completed' ? '2px solid #52c41a' : undefined
            }}
          >
            <Statistic
              title="已完成任务"
              value={stats?.completed || 0}
              valueStyle={{ color: '#52c41a' }}
            />
          </Card>
        </Col>
        <Col span={4}>
          <Card 
            hoverable
            onClick={() => handleStatusFilter('failed')}
            style={{ 
              cursor: 'pointer',
              border: statusFilter === 'failed' ? '2px solid #ff4d4f' : undefined
            }}
          >
            <Statistic
              title="失败任务"
              value={stats?.failed || 0}
              valueStyle={{ color: '#ff4d4f' }}
            />
          </Card>
        </Col>
        <Col span={4}>
          <Card
            hoverable
            onClick={() => handleStatusFilter('deleted')}
            style={{
              cursor: 'pointer',
              border: statusFilter === 'deleted' ? '2px solid #8c8c8c' : undefined
            }}
          >
            <Statistic
              title="已删除任务"
              value={stats?.deleted || 0}
              valueStyle={{ color: '#8c8c8c' }}
            />
          </Card>
        </Col>
      </Row>
      <div style={{ marginBottom: 16, display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <Space>
          <Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>
            新增任务
          </Button>
          {statusFilter && (
            <Button onClick={clearFilter}>
              清除筛选 (当前: {getStatusTag(statusFilter)})
            </Button>
          )}
        </Space>
      </div>
      <Table
        columns={columns}
        dataSource={tasks}
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
        title={editingTask ? '编辑任务' : '新增任务'}
        open={modalVisible}
        onOk={handleSubmit}
        onCancel={() => setModalVisible(false)}
      >
        <Form form={form} layout="vertical">
          <Form.Item name="name" label="任务名称" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="template_id" label="模板ID" rules={[{ required: true }]}>
            <Input type="number" />
          </Form.Item>
          <Form.Item name="status" label="状态">
            <Select>
              <Select.Option value="pending">待执行</Select.Option>
              <Select.Option value="running">运行中</Select.Option>
              <Select.Option value="completed">已完成</Select.Option>
              <Select.Option value="failed">失败</Select.Option>
            </Select>
          </Form.Item>
        </Form>
      </Modal>
    </div>
  )
}

export default TaskList
