import {
  CloudOutlined,
  CloudSyncOutlined,
  GlobalOutlined,
} from "@ant-design/icons";
import { Card } from "antd";
import {
  Route,
  Routes,
  useLocation,
  useNavigate,
} from "react-router-dom";
import { substringAfter } from "utils/string";
import DB from "./data/DB";
import Store from "./data/Store";

const tabs = [
  {
    key: "/store",
    label: (
      <>
        <CloudOutlined />
        云端存储
      </>
    ),
  },
  {
    key: "/db",
    label: (
      <>
        <GlobalOutlined />
        数据连接
      </>
    ),
  },
  {
    key: "/sync",
    label: (
      <>
        <CloudSyncOutlined />
        数据同步
      </>
    ),
  },
];

const Data = () => {
  const { pathname } = useLocation();
  const navigate = useNavigate();
  const parentPath = "/data";
  const activeKey = substringAfter(pathname, parentPath);

  return (
    <Card
      style={{ height: "100%", display: "flex", flexDirection: "column" }}
      headStyle={{ zIndex: 1 }}
      bodyStyle={{ height: "calc(100% - 56px)", padding: 16, overflow: "auto" }}
      tabList={tabs}
      activeTabKey={activeKey}
      onTabChange={(subPath) => navigate(parentPath + subPath)}
    >
      <Routes>
        <Route path="store" Component={Store} />
        <Route path="db" Component={DB} />
      </Routes>
    </Card>
  );
};

export default Data;
