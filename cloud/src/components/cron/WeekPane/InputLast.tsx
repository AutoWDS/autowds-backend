import React from 'react';
import WeekSelect from './WeekSelect';

interface InputLastProps {
  disabled: boolean;
  value: string;
  onChange: (value: string) => void;
}

function InputLast(props: InputLastProps) {
  const { disabled, value, onChange } = props;
  let lastWeekOfMonth = 'SUN';
  if (!disabled) {
    [lastWeekOfMonth] = value.split('L');
  }
  const onChangeLastWeekOfMonth = (v: string) => onChange(`${v}L`);

  return (
    <React.Fragment>
      本月的最后一个&nbsp;
      <WeekSelect
        disabled={disabled}
        value={lastWeekOfMonth}
        size="small"
        onChange={onChangeLastWeekOfMonth}
        style={{ width: 100 }}
      />
      &nbsp;执行一次
    </React.Fragment>
  );
}

export default InputLast;
