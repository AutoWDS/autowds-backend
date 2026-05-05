import { Col, Row } from "antd";
import { useEffect, useState } from "react";
import { getStatistics, StatisticsSummary } from "api/statistics";
import InstanceStats from "./dashboard/InstanceStats";
import TaskStats from "./dashboard/TaskStats";
import UsageStats from "./dashboard/UsageStats";

const Home = () => {
  const [statistics, setStatistics] = useState<StatisticsSummary | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    getStatistics()
      .then((data) => {
        setStatistics(data);
        setLoading(false);
      })
      .catch((err) => {
        console.error("获取统计信息失败:", err);
        setLoading(false);
      });
  }, []);

  if (loading || !statistics) {
    return <div>加载中...</div>;
  }

  return (
    <>
      <Row gutter={[16, 16]}>
        <Col span={8}>
          <TaskStats statistics={statistics.taskStats} />
        </Col>
        <Col span={4}>
          <InstanceStats statistics={statistics.instanceStats} />
        </Col>
        <Col span={12}>
          <UsageStats statistics={statistics.usageStats} />
        </Col>
      </Row>
    </>
  );
};

export default Home;
