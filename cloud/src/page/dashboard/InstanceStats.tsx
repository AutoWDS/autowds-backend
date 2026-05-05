import { ArrowUpOutlined } from "@ant-design/icons";
import { Card, Space, Statistic } from "antd";
import { InstanceStatistics } from "api/statistics";

interface InstanceStatsProps {
  statistics: InstanceStatistics;
}

const InstanceStats = ({ statistics }: InstanceStatsProps) => {
  return (
    <Space direction="vertical" size={16} style={{ width: "100%" }}>
      <Card bordered={false}>
        <Statistic
          title="调度次数"
          value={statistics.totalCount}
          valueStyle={{ color: "#3f8600" }}
          prefix={<ArrowUpOutlined />}
          suffix="次"
        />
      </Card>
      <Card bordered={false}>
        <Statistic
          title="失败次数"
          value={statistics.failedCount}
          valueStyle={{ color: "#cf1322" }}
          prefix={<ArrowUpOutlined />}
          suffix="次"
        />
      </Card>
    </Space>
  );
};

export default InstanceStats;
