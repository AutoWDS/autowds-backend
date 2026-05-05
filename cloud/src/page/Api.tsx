import { Space, Typography } from "antd";
import { ReactComponent as Building } from "assets/svg/building.svg";
import { useEffect } from "react";
import { BASE_URL } from "utils/ajax";

const Api = () => {
  useEffect(() => {
    const sse = new EventSource(BASE_URL + "/instance/0/log");
    sse.onmessage = (e) => {
      console.log(e);
    };
    sse.onerror = (e) => {
      sse.close();
      console.log(e);
    };
    return () => sse.close();
  });
  return (
    <Space direction="vertical" style={{ width: "100%", textAlign: "center" }}>
      <Building />
      <Typography.Text>正在奋力建设中</Typography.Text>
    </Space>
  );
};

export default Api;
