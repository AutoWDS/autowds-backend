import { Space, Typography } from "antd";
import { ReactComponent as Building } from "assets/svg/building.svg";

const Proxy = () => {
  return (
    <Space direction="vertical" style={{ width: "100%", textAlign: "center" }}>
      <Building />
      <Typography.Text>正在奋力建设中</Typography.Text>
    </Space>
  );
};

export default Proxy;
