import type { FormRule } from "antd";
import type {
  FormInstance as RcFormInstance,
  RuleObject,
  RuleRender,
} from "rc-field-form/lib/interface";
import i18n from "i18n";

export const emailRule: FormRule[] = [
  {
    required: true,
    message: i18n("popup_user_validate_email_notEmpty"),
  },
  {
    pattern: /^[\w-.]+@([\w-]+\.)+[\w-]{2,4}$/,
    message: i18n("popup_user_validate_email_notMatch"),
  },
];

export const passwdRule: FormRule[] = [
  {
    required: true,
    message: i18n("popup_user_validate_passwd_notEmpty"),
  },
];

export const confirmPasswdRule: FormRule[] = [
  {
    required: true,
    message: i18n("popup_user_validate_passwd_notEmpty"),
  },
  ((form: RcFormInstance) =>
    ({
      validator(rule: RuleObject, value: string) {
        if (!value || form.getFieldValue("passwd") === value) {
          return Promise.resolve();
        }
        return Promise.reject(i18n("popup_user_validate_passwd_notMatch"));
      },
    } as RuleObject)) as RuleRender,
];

export const validateCodeRule: FormRule[] = [
  {
    required: true,
    message: i18n("popup_user_validate_code_notEmpty"),
  },
];
