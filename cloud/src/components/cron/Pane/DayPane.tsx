import { Radio, RadioChangeEvent, Space } from 'antd';
import _ from 'lodash';
import React from 'react';
import InputFromInterval from '../Common/InputFromInterval';
import InputFromTo from '../Common/InputFromTo';
import InputSpecified from '../Common/InputSpecified';

interface DayPaneProps {
  value: string;
  onChange: (value: string) => void;
}

function DayPane(props: DayPaneProps) {
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
    const defaultValues = ['*', '?', '1-28', '1/1', '1'];
    onChange(defaultValues[valueType]);
  };

  return (
    <Radio.Group
      style={{ width: '100%' }}
      value={currentRadio}
      onChange={onChangeRadio}
    >
      <Space direction="vertical">
        <Radio value={0}>每一天</Radio>
        <Radio value={1}>不指定</Radio>
        <Radio value={2}>
          <InputFromTo
            disabled={currentRadio !== 2}
            unit="天"
            min={1}
            max={31}
            defaultFrom={1}
            defaultTo={28}
            value={value}
            onChange={onChange}
          />
        </Radio>
        <Radio value={3}>
          <InputFromInterval
            disabled={currentRadio !== 3}
            unit="天"
            min={1}
            max={31}
            defaultFrom={1}
            defaultInterval={1}
            value={value}
            onChange={onChange}
          />
        </Radio>
        <Radio value={4}>
          <InputSpecified
            disabled={currentRadio !== 4}
            unit="天"
            options={_.chain(31)
              .range()
              .map((i) => ({ label: i, value: i }))
              .value()}
            value={value}
            onChange={onChange}
          />
        </Radio>
      </Space>
    </Radio.Group>
  );
}

export default DayPane;
