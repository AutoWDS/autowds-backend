import { useState } from "react";
import { Outlet, useNavigate, useLocation } from "react-router-dom";
import { Layout as AntLayout, Menu, Avatar, Dropdown } from "antd";
import {
  DashboardOutlined,
  UserOutlined,
  FileTextOutlined,
  LogoutOutlined,
  SettingOutlined,
  AppstoreOutlined,
  MailOutlined,
} from "@ant-design/icons";
import { useAuthStore } from "@/store/auth";
import type { MenuProps } from "antd";

const { Header, Sider, Content } = AntLayout;

const Layout = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const [collapsed, setCollapsed] = useState(false);
  const { userInfo, clearToken } = useAuthStore();

  const menuItems = [
    {
      key: "/",
      icon: <DashboardOutlined />,
      label: "仪表盘",
    },
    {
      key: "/users",
      icon: <UserOutlined />,
      label: "用户管理",
    },
    {
      key: "/tasks",
      icon: <AppstoreOutlined />,
      label: "任务管理",
    },
    {
      key: "/templates",
      icon: <FileTextOutlined />,
      label: "模板管理",
    },
    {
      key: "/marketing",
      icon: <MailOutlined />,
      label: "营销获客",
    },
  ];

  const handleMenuClick = ({ key }: { key: string }) => {
    navigate(key);
  };

  const handleLogout = () => {
    clearToken();
    navigate("/login");
  };

  const userMenuItems: MenuProps["items"] = [
    {
      key: "settings",
      icon: <SettingOutlined />,
      label: "个人设置",
    },
    {
      type: "divider",
    },
    {
      key: "logout",
      icon: <LogoutOutlined />,
      label: "退出登录",
      onClick: handleLogout,
    },
  ];

  return (
    <AntLayout style={{ minHeight: "100vh" }}>
      <Sider collapsible collapsed={collapsed} onCollapse={setCollapsed}>
        <div
          style={{
            height: 64,
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            color: "#fff",
            fontSize: 18,
            fontWeight: "bold",
          }}
        >
          {collapsed ? "Admin" : "后台管理系统"}
        </div>
        <Menu
          theme="dark"
          selectedKeys={[`/${location.pathname.split('/')[1] || ''}`]}
          mode="inline"
          items={menuItems}
          onClick={handleMenuClick}
        />
      </Sider>
      <AntLayout style={{ height: "100vh" }}>
        <Header
          style={{
            padding: "0 24px",
            background: "#fff",
            display: "flex",
            justifyContent: "flex-end",
            alignItems: "center",
          }}
        >
          <Dropdown menu={{ items: userMenuItems }} placement="bottomRight">
            <div
              style={{
                cursor: "pointer",
                display: "flex",
                alignItems: "center",
                gap: 8,
              }}
            >
              <Avatar icon={<UserOutlined />} />
              <span>{userInfo?.name || "管理员"}</span>
            </div>
          </Dropdown>
        </Header>
        <Content style={{ padding: 24, flex: 1, overflow: "scroll" }}>
          <div style={{ background: "white", padding: "16px" }}>
            <Outlet />
          </div>
        </Content>
      </AntLayout>
    </AntLayout>
  );
};

export default Layout;
