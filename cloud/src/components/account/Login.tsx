import { Button, Form, Input, Space } from "antd";
import { useNavigate } from "react-router-dom";
import { login, setAuthUser } from "api/user";

import "./index.css";

import i18n from "i18n";

import { ReactComponent as Logo } from "assets/svg/logo.svg";

import { emailRule, passwdRule } from "./validateRule";

interface LoginState {
  email: string;
  passwd: string;
}

export const Login = () => {
  const navigate = useNavigate();
  const handleFinish = async ({ email, passwd }: LoginState) => {
    try {
      const data = await login(email, passwd);
      console.log(data);
      await setAuthUser(data);
      navigate("/");
    } catch (e) {}
  };
  return (
    <Form className="form" onFinish={handleFinish}>
      <Space direction="vertical">
        <div className="form-logo-wrap">
          <Logo aria-hidden />
        </div>
        <Form.Item name="email" rules={emailRule}>
          <Input placeholder={i18n("popup_user_email")} />
        </Form.Item>
        <Form.Item name="passwd" rules={passwdRule}>
          <Input.Password placeholder={i18n("popup_user_passwd")} />
        </Form.Item>
        <Form.Item>
          <Button block type="primary" htmlType="submit">
            {i18n("popup_user_login")}
          </Button>
        </Form.Item>
        <div style={{ display: "flex" }}>
          <Button
            type="text"
            size="small"
            onClick={() => navigate("/user/register")}
          >
            {i18n("popup_user_register")}
          </Button>
          <div style={{ flex: 1 }}></div>
          <Button
            type="text"
            size="small"
            onClick={() => navigate("/user/reset-passwd")}
          >
            {i18n("popup_user_forget")}
          </Button>
        </div>
      </Space>
    </Form>
  );
};
