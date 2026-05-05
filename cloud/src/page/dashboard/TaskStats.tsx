import { Card, Statistic } from "antd";
import ReactECharts from "echarts-for-react";
import { TaskStatistics } from "api/statistics";
import formatter from "./countUp";

interface TaskStatsProps {
  statistics: TaskStatistics;
}

const TaskStats = ({ statistics }: TaskStatsProps) => {
  const option = {
    tooltip: {
      trigger: "item",
    },
    legend: {
      bottom: "10",
      left: "center",
    },
    series: [
      {
        name: "任务数量",
        type: "pie",
        left: "100",
        bottom: "10%",
        radius: ["40%", "70%"],
        avoidLabelOverlap: false,
        itemStyle: {
          borderRadius: 10,
          borderColor: "#fff",
          borderWidth: 2,
        },
        label: {
          show: false,
          position: "center",
        },
        emphasis: {
          label: {
            show: true,
            fontSize: 40,
            fontWeight: "bold",
          },
        },
        labelLine: {
          show: false,
        },
        data: [
          { value: statistics.undeployed, name: "未部署" },
          { value: statistics.scheduled, name: "调度中" },
          { value: statistics.completed, name: "调度结束" },
        ],
      },
    ],
  };

  return (
    <Card bordered={false}>
      <Statistic
        title="任务数量"
        value={statistics.total}
        formatter={formatter}
        valueStyle={{ color: "#3f8600" }}
        suffix="个"
        style={{ position: "absolute" }}
      />
      <ReactECharts
        option={option}
        style={{ width: "calc(100% + 48px)", height: "240px", margin: -24 }}
      />
    </Card>
  );
};

export default TaskStats;
