import { Form, Input, type InputProps } from "antd";
import _ from "lodash";
import { useMemo, useState } from "react";

import "./index.css";

import i18n from "i18n";

type ExpiredStatus = boolean | number;

interface ValidateCodeInputProps extends InputProps {
  sendEmailApi: (email: string) => Promise<any>;
}

export const ValidateCodeInput = ({
  sendEmailApi,
  ...rest
}: ValidateCodeInputProps) => {
  const [expired, setExpired] = useState<ExpiredStatus>(true);
  const form = Form.useFormInstance();
  const getStopwatch = useMemo(
    () => (expiration: number) => {
      const stopwatch = () => {
        const expired = (expiration - Date.now()) / 1000;
        if (expired > 0) {
          setExpired(expired);
          setTimeout(stopwatch, 1000);
        } else {
          setExpired(true);
        }
      };
      return stopwatch;
    },
    []
  );
  const handleSendValidationCode = async () => {
    try {
      if (_.isNumber(expired) && expired > 0) {
        return;
      }
      await form.validateFields(["email"]);
      setExpired(false);
      await sendEmailApi(form.getFieldValue("email"));
      const stopwatch = getStopwatch(Date.now() + 60 * 1000);
      stopwatch();
    } catch {}
  };
  return (
    <Input.Search
      placeholder={i18n("popup_user_validateCode")}
      onSearch={handleSendValidationCode}
      loading={expired === false}
      enterButton={
        _.isNumber(expired) && expired > 0
          ? i18n("popup_user_resend_validateCode", { expired: String(Math.round(expired)) })
          : i18n("popup_user_send_validateCode")
      }
      {...rest}
    />
  );
};
