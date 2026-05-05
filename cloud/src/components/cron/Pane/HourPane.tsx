import { Radio, RadioChangeEvent, Space } from 'antd';
import _ from 'lodash';
import React from 'react';
import InputFromInterval from '../Common/InputFromInterval';
import InputFromTo from '../Common/InputFromTo';
import InputSpecified from '../Common/InputSpecified';

interface HourPaneProps {
  value: string;
  onChange: (value: string) => void;
}

function HourPane(props: HourPaneProps) {
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
    const defaultValues = ['*', '0-23', '0/1', '0'];
    onChange(defaultValues[valueType]);
  };

  return (
    <Radio.Group
      style={{ width: '100%' }}
      value={currentRadio}
      onChange={onChangeRadio}
    >
      <Space direction="vertical">
        <Radio value={0}>每小时</Radio>
        <Radio value={1}>
          <InputFromTo
            disabled={currentRadio !== 1}
            value={value}
            unit="小时"
            min={0}
            max={23}
            defaultFrom={0}
            defaultTo={23}
            onChange={onChange}
          />
        </Radio>
        <Radio value={2}>
          <InputFromInterval
            disabled={currentRadio !== 2}
            value={value}
            unit="小时"
            min={0}
            max={23}
            defaultFrom={0}
            defaultInterval={1}
            onChange={onChange}
          />
        </Radio>
        <Radio value={3}>
          <InputSpecified
            disabled={currentRadio !== 3}
            value={value}
            unit="小时"
            options={_.chain(23)
              .range()
              .map((i) => ({ label: i, value: i }))
              .value()}
            onChange={onChange}
          />
        </Radio>
      </Space>
    </Radio.Group>
  );
}

export default HourPane;
