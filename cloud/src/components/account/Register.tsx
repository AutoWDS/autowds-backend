import { Button, Form, Input, Space } from "antd";
import { useNavigate } from "react-router-dom";
import {
  type AuthUser,
  type UserDTO,
  register,
  registerValidationCode,
  setAuthUser,
} from "api/user";

import { ReactComponent as Logo } from "assets/svg/logo.svg";

import { ValidateCodeInput } from "./ValidateCodeInput";

import "./index.css";

import i18n from "i18n";

import {
  confirmPasswdRule,
  emailRule,
  passwdRule,
  validateCodeRule,
} from "./validateRule";

export const Register = () => {
  const [form] = Form.useForm();
  const navigate = useNavigate();
  const handleLogin = () => {
    navigate("/user/login");
  };
  const handleFinish = async (user: UserDTO) => {
    try {
      const data = await register(user);
      await setAuthUser(data as AuthUser);
      navigate("/");
    } catch (e) {}
  };
  return (
    <Form
      className="form"
      form={form}
      validateTrigger="onBlur"
      onFinish={handleFinish}
    >
      <Space direction="vertical">
        <div className="form-logo-wrap">
          <Logo aria-hidden />
        </div>
        <Form.Item
          name="name"
          rules={[{ required: true, message: i18n("popup_user_validate_name_notEmpty") }]}>
          <Input placeholder={i18n("popup_user_name")} />
        </Form.Item>
        <Form.Item name="email" rules={emailRule}>
          <Input placeholder={i18n("popup_user_email")} />
        </Form.Item>
        <Form.Item name="passwd" rules={passwdRule}>
          <Input.Password placeholder={i18n("popup_user_passwd")} />
        </Form.Item>
        <Form.Item name="confirm" rules={confirmPasswdRule}>
          <Input.Password placeholder={i18n("popup_user_confirm_passwd")} />
        </Form.Item>
        <Form.Item name="validate_code" rules={validateCodeRule}>
          <ValidateCodeInput sendEmailApi={registerValidationCode} />
        </Form.Item>
        <Button block type="primary" htmlType="submit">
          {i18n("popup_user_register")}
        </Button>
        <Button block type="text" size="small" onClick={handleLogin}>
          {i18n("popup_user_return")}
        </Button>
      </Space>
    </Form>
  );
};
