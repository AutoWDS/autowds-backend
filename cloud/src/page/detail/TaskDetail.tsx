import {
  BarsOutlined,
  RollbackOutlined,
  SettingOutlined,
  StockOutlined,
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
import DataQuality from "./DataQuality";
import InstanceRecords from "./InstanceRecords";
import ScheduleSettings from "./ScheduleSettings";

const tabs = [
  {
    key: "/instances",
    label: (
      <>
        <BarsOutlined />
        调度记录
      </>
    ),
  },
  {
    key: "/settings",
    label: (
      <>
        <SettingOutlined />
        调度设置
      </>
    ),
  },
  {
    key: "/quality",
    label: (
      <>
        <StockOutlined />
        数据质量
      </>
    ),
  },
];

const TaskDetail = () => {
  const { taskId } = useParams();
  const { pathname } = useLocation();
  const navigate = useNavigate();
  const parentPath = `/task/myself/${taskId}`;
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
        <Route
          index
          element={<Navigate to={`${parentPath}/instances`} replace />}
        />
        <Route path="instances" Component={InstanceRecords} />
        <Route path="settings" Component={ScheduleSettings} />
        <Route path="quality" Component={DataQuality} />
      </Routes>
    </Card>
  );
};

export default TaskDetail;
