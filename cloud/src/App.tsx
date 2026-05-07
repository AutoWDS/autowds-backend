import Icon, {
  AlertOutlined,
  ApiOutlined,
  BellOutlined,
  BugOutlined,
  CloudServerOutlined,
  CrownOutlined,
  HddOutlined,
  LogoutOutlined,
  MenuFoldOutlined,
  MenuUnfoldOutlined,
  UserOutlined,
} from "@ant-design/icons";
import {
  Avatar,
  Badge,
  Button,
  Dropdown,
  Layout,
  Menu,
  MenuProps,
  Modal,
  Space,
  Spin,
  theme,
} from "antd";
import { AuthUserDTO, currentUser, setAuthUser } from "api/user";
import Alert from "page/Alert";
import Home from "page/Home";
import Pay from "page/Pay";
import CleanPipeline from "page/data/CleanPipeline";
import DataDetail from "page/data/DataDetail";
import NewDataStore from "page/data/NewDataStore";
import TaskInstance from "page/detail/TaskInstance";
import { useEffect, useState } from "react";
import { Link, Navigate, Route, Routes, useLocation, useNavigate } from "react-router-dom";
import "./App.css";
import Navigator from "./Navigator";
import { ReactComponent as Logo } from "./assets/svg/logo.svg";
import { ReactComponent as ApiDocIcon } from "assets/svg/apiDoc.svg";
import Api from "./page/Api";
import Data from "./page/Data";
import Proxy from "./page/Proxy";
import Task from "./page/Task";
import TaskDetail from "./page/detail/TaskDetail";

const { Header, Sider, Content } = Layout;

export const mainMenu = [
  {
    key: "task",
    icon: <BugOutlined />,
    label: <Link to="/task">任务调度</Link>,
  },
  {
    key: "data",
    icon: <HddOutlined />,
    label: <Link to="/data">数据中心</Link>,
  },
  {
    key: "api",
    icon: <ApiOutlined />,
    label: <Link to="/api">接口管理</Link>,
  },
  {
    key: "proxy",
    icon: <CloudServerOutlined />,
    label: <Link to="/proxy">代理设置</Link>,
  },
  {
    key: "alert",
    icon: <AlertOutlined />,
    label: <Link to="/alert">告警设置</Link>,
  },
  {
    key: "pay",
    icon: <CrownOutlined />,
    label: <Link to="/pay">升级会员</Link>,
  },
];


function App() {
  const [collapsed, setCollapsed] = useState(false);
  const { pathname } = useLocation();
  const navigate = useNavigate();
  const {
    token: { colorBgContainer },
  } = theme.useToken();
  const activeKey = pathname.split("/")[1] || "";
  const [user, setUser] = useState<AuthUserDTO>();
  useEffect(() => {
    (async () => {
      const user = await currentUser();
      setUser(user);
    })();
  }, []);

  const handleLogout = () => {
    Modal.confirm({
      title: "退出登录",
      content: "确定要退出登录吗？",
      okText: "确定",
      cancelText: "取消",
      onOk: async () => {
        await setAuthUser(null);
        navigate("/login");
      },
    });
  };

  const userItems: MenuProps["items"] = [
    {
      key: "logout",
      label: "退出登录",
      icon: <LogoutOutlined />,
      onClick: handleLogout,
    },
  ];
  return user ? (
    <Layout style={{ height: "100vh" }}>
      <Sider trigger={null} collapsible collapsed={collapsed}>
        <Link to="/" className={`logo ${collapsed ? "collapsed" : ""}`}>
          <Icon component={Logo} />
          <span className="title">AutoWDS</span>
        </Link>
        <Menu
          theme="dark"
          mode="inline"
          selectedKeys={[activeKey]}
          items={mainMenu}
        />
      </Sider>
      <Layout>
        <Header
          style={{
            display: "flex",
            alignItems: "center",
            padding: "0 20px 0 0",
            background: colorBgContainer,
          }}
        >
          <Button
            type="text"
            icon={collapsed ? <MenuUnfoldOutlined /> : <MenuFoldOutlined />}
            onClick={() => setCollapsed(!collapsed)}
            className="header-btn"
          />
          <div style={{ flex: 1 }} />
          <Space>
            <Button
              type="text"
              size="large"
              href="/openapi/docs/scalar"
              icon={<Icon component={ApiDocIcon} />}
            />
            <Button
              type="text"
              size="large"
              icon={
                <Badge count={0} showZero>
                  <BellOutlined />
                </Badge>
              }
            />
            <Dropdown
              menu={{ items: userItems }}
              placement="bottomRight"
              overlayClassName="user-dropdown-menu"
            >
              <Space
                style={{
                  cursor: "pointer",
                  padding: "4px 8px",
                  borderRadius: "4px",
                  transition: "background-color 0.2s",
                }}
                className="user-dropdown-trigger"
              >
                <Avatar icon={<UserOutlined />} />
                <span>{user.name}</span>
              </Space>
            </Dropdown>
          </Space>
        </Header>
        <Content
          style={{
            display: "flex",
            flexDirection: "column",
            margin: "24px 16px 16px",
          }}
        >
          {activeKey ? <Navigator /> : null}
          <div
            style={{
              flex: 1,
              overflow: "hidden",
            }}
          >
            <Routes>
              <Route path="/" Component={Home} />
              <Route path="/task/myself/:taskId/*" Component={TaskDetail} />
              <Route
                path="/task/myself/:taskId/instances/:instanceId/*"
                Component={TaskInstance}
              />
              <Route path="/task/*" Component={Task} />
              <Route path="/data">
                <Route index element={<Navigate to="/data/store" replace />} />
                <Route path="db/new">
                  <Route path="MySQL" Component={NewDataStore} />
                  <Route path="Postgresql" Component={NewDataStore} />
                  <Route path="Oracle" Component={NewDataStore} />
                  <Route path="SQLServer" Component={NewDataStore} />
                  <Route path="MongoDB" Component={NewDataStore} />
                  <Route path="RDB" Component={NewDataStore} />
                </Route>
                <Route path=":storeId/detail" Component={DataDetail} />
                <Route path=":storeId/clean" Component={CleanPipeline} />
                <Route path="*" Component={Data} />
              </Route>
              <Route path="/api" Component={Api} />
              <Route path="/proxy" Component={Proxy} />
              <Route path="/alert" Component={Alert} />
              <Route path="/pay" Component={Pay} />
            </Routes>
          </div>
        </Content>
      </Layout>
    </Layout>
  ) : (
    <div
      style={{
        width: "100%",
        height: "100%",
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <Spin />
    </div>
  );
}

export default App;
