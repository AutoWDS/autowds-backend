import {
  CodeOutlined,
  DatabaseOutlined,
  RollbackOutlined,
} from "@ant-design/icons";
import { Button, Card } from "antd";
import {
  Navigate,
  Route,
  Routes,
  useLocation,
  useNavigate,
  useParams,
} from "react-router-dom";
import { substringAfter } from "utils/string";
import Data from "./instance/Data";
import Log from "./instance/Log";

const tabs = [
  {
    key: "/data",
    label: (
      <>
        <DatabaseOutlined />
        数据
      </>
    ),
  },
  {
    key: "/log",
    label: (
      <>
        <CodeOutlined />
        日志
      </>
    ),
  },
];

const TaskInstance = () => {
  const { taskId, instanceId } = useParams();
  const { pathname } = useLocation();
  const navigate = useNavigate();
  const parentPath = `/task/myself/${taskId}/instances/${instanceId}`;
  const activeKey = substringAfter(pathname, parentPath);

  return (
    <Card
      style={{ height: "100%", display: "flex", flexDirection: "column" }}
      headStyle={{ zIndex: 1 }}
      bodyStyle={{ height: "calc(100% - 56px)", padding: 16, overflow: "auto" }}
      tabList={tabs}
      activeTabKey={activeKey}
      onTabChange={(subPath) => navigate(parentPath + subPath)}
      tabBarExtraContent={
        <Button
          type="link"
          icon={<RollbackOutlined />}
          onClick={() => navigate(-1)}
        >
          返回
        </Button>
      }
    >
      <Routes>
        <Route index element={<Navigate to={`${parentPath}/data`} replace />} />
        <Route path="data" Component={Data} />
        <Route path="log" Component={Log} />
      </Routes>
    </Card>
  );
};

export default TaskInstance;
