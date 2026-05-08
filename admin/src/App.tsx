import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom'
import Login from './pages/Login'
import Layout from './components/Layout'
import Dashboard from './pages/Dashboard'
import UserList from './pages/User/UserList'
import TaskList from './pages/Task/TaskList'
import TemplateList from './pages/Template/TemplateList'
import Marketing from './pages/Marketing'
import { useAuthStore } from './store/auth'

function App() {
  const token = useAuthStore((state) => state.token)
  const isAdmin = useAuthStore((state) => state.isAdmin())

  return (
    <BrowserRouter basename="/admin">
      <Routes>
        <Route path="/login" element={<Login />} />
        <Route
          path="/"
          element={
            token && isAdmin ? <Layout /> : <Navigate to="/login" replace />
          }
        >
          <Route index element={<Dashboard />} />
          <Route path="users" element={<UserList />} />
          <Route path="tasks" element={<TaskList />} />
          <Route path="templates" element={<TemplateList />} />
          <Route path="marketing/*" element={<Marketing />} />
        </Route>
      </Routes>
    </BrowserRouter>
  )
}

export default App
