import { Button, Form, Input, Space } from "antd";
import { useNavigate } from "react-router-dom";
import {
  registerValidationCode,
  resetPasswd,
  setAuthUser,
  type AuthUser,
  type UserDTO,
} from "api/user";

import i18n from "i18n";

import { ReactComponent as Logo } from "assets/svg/logo.svg";

import { ValidateCodeInput } from "./ValidateCodeInput";
import {
  confirmPasswdRule,
  emailRule,
  passwdRule,
  validateCodeRule,
} from "./validateRule";

import "./index.css";

export const ResetPasswd = () => {
  const [form] = Form.useForm();
  const navigate = useNavigate();
  const handleReturn = () => {
    navigate(-1);
  };
  const handleFinish = async (user: UserDTO) => {
    try {
      const data = await resetPasswd(user);
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
        <Form.Item name="email" rules={emailRule}>
          <Input placeholder={i18n("popup_user_email")} />
        </Form.Item>
        <Form.Item name="passwd" rules={passwdRule}>
          <Input.Password placeholder={i18n("popup_user_passwd_new")} />
        </Form.Item>
        <Form.Item name="confirm" rules={confirmPasswdRule}>
          <Input.Password placeholder={i18n("popup_user_confirm_passwd_new")} />
        </Form.Item>
        <Form.Item name="validate_code" rules={validateCodeRule}>
          <ValidateCodeInput sendEmailApi={registerValidationCode} />
        </Form.Item>
        <Button block type="primary" htmlType="submit">
          {i18n("popup_user_reset_passwd")}
        </Button>
        <Button block type="text" size="small" onClick={handleReturn}>
          {i18n("popup_user_return")}
        </Button>
      </Space>
    </Form>
  );
};
