import {
  AppstoreOutlined,
  BarsOutlined,
  HeartOutlined,
  ImportOutlined,
  MenuOutlined,
} from "@ant-design/icons";
import { Button, Card, Segmented, type GetProp } from "antd";
import { addTask } from "api/task";
import { fileOpen } from "browser-fs-access";
import i18n from "i18n";
import _ from "lodash";
import { useState } from "react";
import {
  Navigate,
  Route,
  Routes,
  useLocation,
  useNavigate,
} from "react-router-dom";
import CommonTemplate from "./task/CommonTemplate";
import MyFavorite from "./task/MyFavorite";
import MyTask from "./task/MyTask";

const tabs = [
  {
    key: "/task/myself",
    label: (
      <>
        <BarsOutlined />
        我的任务
      </>
    ),
  },
  {
    key: "/task/template",
    label: (
      <>
        <AppstoreOutlined />
        模板市场
      </>
    ),
  },
  {
    key: "/task/favorite",
    label: (
      <>
        <HeartOutlined />
        我的收藏
      </>
    ),
  },
];

const TemplateMode = [
  {
    value: 1,
    icon: <AppstoreOutlined />,
  },
  {
    value: 0,
    icon: <MenuOutlined />,
  },
];

type SegmentedValue = GetProp<typeof Segmented, "value">;

const ScraperTask = () => {
  const { pathname } = useLocation();
  const navigate = useNavigate();
  const [mode, setMode] = useState<SegmentedValue>(1);
  const [lastImportTime, setLastImportTime] = useState<string>();
  const activeKey = pathname || "/task/myself";

  const handleImport = async () => {
    try {
      const blob = await fileOpen({
        description: i18n("popup_task_actions_exportFileDesc"),
        mimeTypes: ["application/json"],
        extensions: [".json"],
      });
      const json = await blob.text();
      const { created } = await addTask({
        name: _.trimEnd(blob.name, ".json"),
        rule: JSON.parse(json),
      });
      setLastImportTime(created);
    } catch (e) {
      console.log(e);
    }
  };

  return (
    <Card
      style={{ height: "100%", display: "flex", flexDirection: "column" }}
      headStyle={{ zIndex: 1 }}
      bodyStyle={{ height: "calc(100% - 56px)", padding: "16px 16px 0 16px" }}
      tabList={tabs}
      activeTabKey={activeKey}
      onTabChange={navigate}
      tabBarExtraContent={
        activeKey === "/task/myself" ? (
          <Button type="link" icon={<ImportOutlined />} onClick={handleImport}>
            导入
          </Button>
        ) : (
          <Segmented options={TemplateMode} value={mode} onChange={setMode} />
        )
      }
    >
      <Routes>
        <Route index element={<Navigate to="/task/myself" replace />} />
        <Route
          path="myself"
          element={<MyTask lastImportTime={lastImportTime} />}
        />
        <Route
          path="template"
          element={<CommonTemplate card={Boolean(mode)} />}
        />
        <Route path="favorite" element={<MyFavorite card={Boolean(mode)} />} />
      </Routes>
    </Card>
  );
};

export default ScraperTask;
