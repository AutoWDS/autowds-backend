import { useEffect, useState } from 'react'
import { Card, Row, Col, Statistic } from 'antd'
import { UserOutlined, AppstoreOutlined, FileTextOutlined } from '@ant-design/icons'
import { useNavigate } from 'react-router-dom'
import { getStatistics, type StatisticsOverview } from '@/api/statistics'

const Dashboard = () => {
  const [stats, setStats] = useState<StatisticsOverview | null>(null)
  const navigate = useNavigate()

  useEffect(() => {
    loadStatistics()
  }, [])

  const loadStatistics = async () => {
    try {
      const data = await getStatistics()
      setStats(data)
    } catch (error) {
      console.error('加载统计数据失败:', error)
    }
  }

  return (
    <div>
      <h2 style={{ marginBottom: 24 }}>数据概览</h2>
      <Row gutter={16}>
        <Col span={8}>
          <Card 
            hoverable 
            onClick={() => navigate('/users')}
            style={{ cursor: 'pointer' }}
          >
            <Statistic
              title="用户总数"
              value={stats?.user_count || 0}
              prefix={<UserOutlined />}
            />
          </Card>
        </Col>
        <Col span={8}>
          <Card 
            hoverable 
            onClick={() => navigate('/tasks')}
            style={{ cursor: 'pointer' }}
          >
            <Statistic
              title="任务总数"
              value={stats?.task_count || 0}
              prefix={<AppstoreOutlined />}
            />
          </Card>
        </Col>
        <Col span={8}>
          <Card 
            hoverable 
            onClick={() => navigate('/templates')}
            style={{ cursor: 'pointer' }}
          >
            <Statistic
              title="模板总数"
              value={stats?.template_count || 0}
              prefix={<FileTextOutlined />}
            />
          </Card>
        </Col>
      </Row>
    </div>
  )
}

export default Dashboard
