import { Radio, RadioChangeEvent, Space } from 'antd';
import _ from 'lodash';
import React from 'react';
import InputFromInterval from '../Common/InputFromInterval';
import InputFromTo from '../Common/InputFromTo';
import InputSpecified from '../Common/InputSpecified';

interface YearPaneProps {
  value: string;
  onChange: (value: string) => void;
}

function YearPane(props: YearPaneProps) {
  const { value, onChange } = props;
  let currentRadio = 0;
  if (value === '*') {
    currentRadio = 0;
  } else if (value === '?') {
    currentRadio = 1;
  } else if (value.indexOf('-') > -1) {
    currentRadio = 2;
  } else if (value.indexOf('/') > -1) {
    currentRadio = 3;
  } else {
    currentRadio = 4;
  }

  const onChangeRadio = (e: RadioChangeEvent) => {
    const valueType = e.target.value;
    const currentYear = new Date().getUTCFullYear();
    const defaultValues = [
      '*',
      '?',
      `${currentYear}-${currentYear + 10}`,
      `${currentYear}/1`,
      `${currentYear}`,
    ];
    onChange(defaultValues[valueType]);
  };

  const currentYear = new Date().getUTCFullYear();
  return (
    <Radio.Group
      style={{ width: '100%' }}
      value={currentRadio}
      onChange={onChangeRadio}
    >
      <Space direction="vertical">
        <Radio value={0}>每年</Radio>
        <Radio value={1}>不指定</Radio>
        <Radio value={2}>
          <InputFromTo
            disabled={currentRadio !== 2}
            value={value}
            unit="年"
            min={0}
            max={9999}
            defaultFrom={currentYear}
            defaultTo={currentYear + 10}
            onChange={onChange}
          />
        </Radio>
        <Radio value={3}>
          <InputFromInterval
            disabled={currentRadio !== 3}
            value={value}
            unit="年"
            min={0}
            max={9999}
            defaultFrom={currentYear}
            defaultInterval={1}
            onChange={onChange}
          />
        </Radio>
        <Radio value={4}>
          <InputSpecified
            disabled={currentRadio !== 4}
            value={value}
            unit="年"
            options={_.chain(currentYear)
              .range(9999)
              .map((i) => ({ label: i, value: i }))
              .value()}
            onChange={onChange}
          />
        </Radio>
      </Space>
    </Radio.Group>
  );
}

export default YearPane;
