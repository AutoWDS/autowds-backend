import { InputNumber } from 'antd';
import React from 'react';

interface InputFromIntervalProps {
  disabled: boolean;
  unit: string;
  min: number;
  max: number;
  defaultFrom: number;
  defaultInterval: number;
  value: string;
  onChange: (v: string) => void;
}

function InputFromInterval(props: InputFromIntervalProps) {
  const {
    disabled,
    unit,
    value,
    defaultFrom,
    defaultInterval,
    onChange,
    ...rest
  } = props;
  let from = defaultFrom;
  let interval = defaultInterval;
  if (!disabled) {
    [from, interval] = value.split('/').map((v) => parseInt(v, 10));
  }
  const onChangeFrom = (v: number | null) =>
    onChange(`${v || defaultFrom}/${interval}`);
  const onChangeInterval = (v: number | null) =>
    onChange(`${from}/${v || defaultInterval}`);

  return (
    <>
      从&nbsp;
      <InputNumber
        disabled={disabled}
        value={from}
        size="small"
        onChange={onChangeFrom}
        style={{ width: 100 }}
        {...rest}
      />
      &nbsp;{unit}开始， 每&nbsp;
      <InputNumber
        disabled={disabled}
        value={interval}
        size="small"
        onChange={onChangeInterval}
        style={{ width: 100 }}
        {...rest}
      />
      &nbsp;{unit}执行一次
    </>
  );
}

export default InputFromInterval;
