import type { StatisticProps } from "antd";
import CountUp from "react-countup";

type StatisticFormatter = Exclude<
  StatisticProps["formatter"],
  false | "number" | "countdown" | undefined
>;

const formatter: StatisticFormatter = (value) => (
  <CountUp end={value as number} separator="," />
);

export default formatter;
