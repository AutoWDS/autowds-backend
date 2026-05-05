import { ArrowDownOutlined } from "@ant-design/icons";
import { Card, Col, Row, Space, Statistic } from "antd";
import { graphic } from "echarts";

import ReactECharts from "echarts-for-react";
import { UsageStatistics } from "api/statistics";
import formatter from "./countUp";

interface UsageStatsProps {
  statistics: UsageStatistics;
}

const UsageStats = ({ statistics }: UsageStatsProps) => {
  const getOption = () => {
    const dates = statistics.timeSeries.map((item) => item.date);
    const data = statistics.timeSeries.map((item) => item.value);

    return {
      tooltip: {
        trigger: "axis",
      },
      xAxis: {
        type: "category",
        boundaryGap: false,
        data: dates,
      },
      yAxis: {
        type: "value",
        boundaryGap: [0, "100%"],
      },
      series: [
        {
          name: "任务创建数",
          type: "line",
          symbol: "none",
          sampling: "lttb",
          itemStyle: {
            color: "rgb(255, 70, 131)",
          },
          areaStyle: {
            color: new graphic.LinearGradient(0, 0, 0, 1, [
              {
                offset: 0,
                color: "rgb(255, 158, 68)",
              },
              {
                offset: 1,
                color: "rgb(255, 70, 131)",
              },
            ]),
          },
          data: data,
        },
      ],
    };
  };

  return (
    <Space direction="vertical" size={16} style={{ width: "100%" }}>
      <Row gutter={16}>
        <Col span={12}>
          <Card bordered={false}>
            <Statistic
              title="数据表数量"
              value={statistics.tableCount}
              valueStyle={{ color: "#cf1322" }}
              prefix={<ArrowDownOutlined />}
              formatter={formatter}
              suffix="个"
            />
          </Card>
        </Col>
        <Col span={12}>
          <Card bordered={false}>
            <Statistic
              title="数据表数量"
              value={statistics.tableCount}
              formatter={formatter}
              valueStyle={{ color: "#cf1322" }}
              prefix={<ArrowDownOutlined />}
              suffix="个"
            />
          </Card>
        </Col>
      </Row>
      <Card bordered={false} bodyStyle={{ display: "flex" }}>
        <Statistic
          title="数据占用空间"
          value={statistics.storageSize}
          precision={2}
          valueStyle={{ color: "#cf1322" }}
          prefix={<ArrowDownOutlined />}
          style={{ width: 120 }}
          suffix="M"
        />
        <ReactECharts
          option={getOption()}
          style={{
            width: "calc(100% - 120px)",
            height: "110px",
            margin: "-24px 0",
          }}
        />
      </Card>
    </Space>
  );
};

export default UsageStats;
