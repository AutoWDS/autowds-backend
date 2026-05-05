import { InputNumber } from 'antd';
import React from 'react';
import WeekSelect from './WeekSelect';

interface InputTargetProps {
  disabled: boolean;
  value: string;
  onChange: (value: string) => void;
}

function InputTarget(props: InputTargetProps) {
  const { disabled, value, onChange } = props;
  let weekOfMonth = 1;
  let dayOfWeek = 'MON';
  if (!disabled) {
    const [weekOfMonthStr, dayOfWeekStr] = value.split('#');
    weekOfMonth = parseInt(weekOfMonthStr, 10);
    dayOfWeek = dayOfWeekStr;
  }
  const onChangeWeekOfMonth = (v: number | null) =>
    onChange(`${v || 1}#${dayOfWeek}`);
  const onChangeDayOfWeek = (v: string) =>
    onChange(`${weekOfMonth}#${v || 'MON'}`);

  return (
    <React.Fragment>
      本月第&nbsp;
      <InputNumber
        disabled={disabled}
        min={1}
        max={5}
        value={weekOfMonth}
        size="small"
        onChange={onChangeWeekOfMonth}
        style={{ width: 100 }}
      />
      &nbsp;周的&nbsp;
      <WeekSelect
        disabled={disabled}
        value={dayOfWeek}
        size="small"
        onChange={onChangeDayOfWeek}
        style={{ width: 100 }}
      />
      &nbsp;执行一次
    </React.Fragment>
  );
}

export default InputTarget;
