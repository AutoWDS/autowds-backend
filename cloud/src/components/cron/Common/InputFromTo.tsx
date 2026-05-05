import React from 'react';
import { InputNumber } from 'antd';

interface InputFromToProps {
  disabled: boolean;
  unit: string;
  min: number;
  max: number;
  defaultFrom: number;
  defaultTo: number;
  value: string;
  onChange: (v: string) => void;
}

function InputFromTo(props: InputFromToProps) {
  const { disabled, defaultFrom, defaultTo, unit, value, onChange, ...rest } =
    props;
  let from = defaultFrom;
  let to = defaultTo;
  if (!disabled) {
    [from, to] = value.split('-').map((v) => parseInt(v, 10));
  }
  const onChangeFrom = (v: number | null) =>
    onChange(`${v || defaultFrom}-${to}`);
  const onChangeTo = (v: number | null) =>
    onChange(`${from}-${v || defaultTo}`);

  return (
    <React.Fragment>
      从&nbsp;
      <InputNumber
        disabled={disabled}
        value={from}
        size="small"
        onChange={onChangeFrom}
        style={{ width: 100 }}
        {...rest}
      />
      &nbsp;-&nbsp;
      <InputNumber
        disabled={disabled}
        value={to}
        size="small"
        onChange={onChangeTo}
        style={{ width: 100 }}
        {...rest}
      />
      &nbsp;{unit}，每{unit}执行一次
    </React.Fragment>
  );
}

export default InputFromTo;
