import { Form, Input, Button, Card, message } from 'antd'
import { UserOutlined, LockOutlined } from '@ant-design/icons'
import { useNavigate } from 'react-router-dom'
import { login } from '@/api/auth'
import { useAuthStore } from '@/store/auth'
import './index.css'

const Login = () => {
  const navigate = useNavigate()
  const { setToken, setUserInfo } = useAuthStore()
  const [form] = Form.useForm()

  const handleSubmit = async (values: any) => {
    try {
      const res = await login({
        email: values.username,
        passwd: values.password,
      })
      
      // 检查是否为管理员
      if (!res.is_admin) {
        message.error('您没有管理员权限，无法访问后台')
        return
      }
      
      setToken(res.token)
      setUserInfo({
        id: res.id,
        name: res.name,
        email: res.email,
        is_admin: res.is_admin,
        edition: res.edition,
      })
      message.success('登录成功')
      navigate('/')
    } catch (error) {
      console.error('登录失败:', error)
    }
  }

  return (
    <div className="login-container">
      <Card className="login-card" title="后台管理系统">
        <Form form={form} onFinish={handleSubmit} size="large">
          <Form.Item
            name="username"
            rules={[{ required: true, message: '请输入用户名' }]}
          >
            <Input prefix={<UserOutlined />} placeholder="用户名" />
          </Form.Item>
          <Form.Item
            name="password"
            rules={[{ required: true, message: '请输入密码' }]}
          >
            <Input.Password prefix={<LockOutlined />} placeholder="密码" />
          </Form.Item>
          <Form.Item>
            <Button type="primary" htmlType="submit" block>
              登录
            </Button>
          </Form.Item>
        </Form>
      </Card>
    </div>
  )
}

export default Login
