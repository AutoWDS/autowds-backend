import { Radio, RadioChangeEvent, Space } from 'antd';
import _ from 'lodash';
import React from 'react';
import InputFromInterval from '../Common/InputFromInterval';
import InputFromTo from '../Common/InputFromTo';
import InputSpecified from '../Common/InputSpecified';

interface MonthPaneProps {
  value: string;
  onChange: (value: string) => void;
}

function MonthPane(props: MonthPaneProps) {
  const { value, onChange } = props;
  let currentRadio = 0;
  if (value === '*') {
    currentRadio = 0;
  } else if (value.indexOf('-') > -1) {
    currentRadio = 1;
  } else if (value.indexOf('/') > -1) {
    currentRadio = 2;
  } else {
    currentRadio = 3;
  }

  const onChangeRadio = (e: RadioChangeEvent) => {
    const valueType = e.target.value;
    const defaultValues = ['*', '1-12', '1/1', '1'];
    onChange(defaultValues[valueType]);
  };

  return (
    <Radio.Group
      style={{ width: '100%' }}
      value={currentRadio}
      onChange={onChangeRadio}
    >
      <Space direction="vertical">
        <Radio value={0}>每一月</Radio>
        <Radio value={1}>
          <InputFromTo
            disabled={currentRadio !== 1}
            value={value}
            unit="月"
            min={1}
            max={12}
            defaultFrom={1}
            defaultTo={12}
            onChange={onChange}
          />
        </Radio>
        <Radio value={2}>
          <InputFromInterval
            disabled={currentRadio !== 2}
            value={value}
            unit="月"
            min={1}
            max={12}
            defaultFrom={1}
            defaultInterval={1}
            onChange={onChange}
          />
        </Radio>
        <Radio value={3}>
          <InputSpecified
            disabled={currentRadio !== 3}
            value={value}
            unit="月"
            options={_.chain(1)
              .range(13)
              .map((i) => ({ label: i, value: i }))
              .value()}
            onChange={onChange}
          />
        </Radio>
      </Space>
    </Radio.Group>
  );
}

export default MonthPane;
