import { CronExpressionParser as cronParser } from 'cron-parser';
import { Button, Modal, Space, Tabs } from 'antd';
import dayjs from 'dayjs';
import React, { useEffect, useState } from 'react';
import './Cron.css';
import DayPane from './Pane/DayPane';
import HourPane from './Pane/HourPane';
import MinutePane from './Pane/MinutePane';
import MonthPane from './Pane/MonthPane';
import SecondPane from './Pane/SecondPane';
// import YearPane from './Pane/YearPane';
import WeekPane from './WeekPane';
import {
  dayRegex,
  hourRegex,
  minuteRegex,
  monthRegex,
  secondRegex,
  weekRegex,
  // yearRegex,
} from './cron-regex';

const { TabPane } = Tabs;

interface IProps {
  /**
   * Cron表达式，用来解析到UI
   */
  value?: string;
  /**
   * 点击生成按钮时调用该回调
   */
  onOk?: (value: string) => void;
}

const Cron = (props: IProps) => {
  const { value, onOk } = props;
  const [currentTab, setCurrentTab] = useState('1');
  const [second, setSecond] = useState('*');
  const [minute, setMinute] = useState('*');
  const [hour, setHour] = useState('*');
  const [day, setDay] = useState('*');
  const [month, setMonth] = useState('*');
  const [week, setWeek] = useState('?');
  // const [year, setYear] = useState('*');
  const [generatedTimes, setGeneratedTimes] = useState<Date[]>();

  const onParse = () => {
    if (value) {
      try {
        let [
          secondVal,
          minuteValue,
          hourVal,
          dayVal,
          monthVal,
          weekVal,
          // yearVal,
        ] = value.split(' ');
        secondVal = secondRegex.test(secondVal) ? secondVal : '*';
        minuteValue = minuteRegex.test(minuteValue) ? minuteValue : '*';
        hourVal = hourRegex.test(hourVal) ? hourVal : '*';
        dayVal = dayRegex.test(dayVal) ? dayVal : '*';
        monthVal = monthRegex.test(monthVal) ? monthVal : '*';
        weekVal = weekRegex.test(weekVal) ? weekVal : '?';
        weekVal = dayVal !== '?' ? '?' : weekVal;
        // yearVal = yearRegex.test(yearVal) ? yearVal : '*';
        setSecond(secondVal);
        setMinute(minuteValue);
        setHour(hourVal);
        setDay(dayVal);
        setMonth(monthVal);
        setWeek(weekVal);
        // setYear(yearVal);
      } catch (error) {
        setSecond('*');
        setMinute('*');
        setHour('*');
        setDay('*');
        setMonth('*');
        setWeek('?');
        // setYear('*');
      }
    }
  };

  const handleClear = () => {
    if (onOk) onOk('');
  };

  const handleReset = () => {
    setSecond('*');
    setMinute('*');
    setHour('*');
    setDay('*');
    setMonth('*');
    setWeek('?');
    // setYear('*');
    setGeneratedTimes(undefined);
  };

  const handleGenerate = () => {
    const expression = [second, minute, hour, day, month, week/*, year*/].join(' ');
    const interval = cronParser.parse(expression);
    const nexts = [];
    nexts.push(interval.next().toDate());
    nexts.push(interval.next().toDate());
    nexts.push(interval.next().toDate());
    setGeneratedTimes(nexts);
  };

  const handleOk = () => {
    if (onOk) {
      onOk([second, minute, hour, day, month, week/*, year*/].join(' '));
    }
  };

  const onChangeDay = (v: string) => {
    setDay(v);
    if (v !== '?') {
      setWeek('?');
    }
  };

  const onChangeWeek = (v: string) => {
    setWeek(v);
    if (v !== '?') {
      setDay('?');
    }
  };

  useEffect(onParse, [value]);

  return (
    <div className="cron-panel">
      <Tabs
        tabBarGutter={0}
        animated
        destroyInactiveTabPane
        activeKey={currentTab}
        onChange={setCurrentTab}
      >
        <TabPane tab="秒" key="1">
          <SecondPane value={second} onChange={setSecond} />
        </TabPane>
        <TabPane tab="分" key="2">
          <MinutePane value={minute} onChange={setMinute} />
        </TabPane>
        <TabPane tab="时" key="3">
          <HourPane value={hour} onChange={setHour} />
        </TabPane>
        <TabPane tab="日" key="4">
          <DayPane value={day} onChange={onChangeDay} />
        </TabPane>
        <TabPane tab="月" key="5">
          <MonthPane value={month} onChange={setMonth} />
        </TabPane>
        <TabPane tab="周" key="6">
          <WeekPane value={week} onChange={onChangeWeek} />
        </TabPane>
        {/* <TabPane tab="年" key="7">
          <YearPane value={year} onChange={setYear} />
        </TabPane> */}
      </Tabs>
      <Space className="cron-buttons">
        <Button onClick={handleClear}>清空</Button>
        <Button onClick={handleReset}>重置</Button>
        <Button type="primary" onClick={handleGenerate}>
          生成
        </Button>
        <Button type="primary" onClick={handleOk}>
          确定
        </Button>
      </Space>
      <div style={{ textAlign: 'center' }}>
        {generatedTimes?.map((t, i) => (
          <p key={i}>{dayjs(t).format('YYYY-MM-DD HH:mm:ss')}</p>
        ))}
      </div>
    </div>
  );
};

export const selectCron = async (cron?: string): Promise<string> => {
  return new Promise((resolve, reject) => {
    const handleOk = (value: string) => {
      resolve(value);
      modal.destroy();
    };
    const modal = Modal.confirm({
      title: null,
      icon: null,
      closable: true,
      width: 600,
      onCancel: () => console.log('aaa'),
      footer: null,
      content: <Cron value={cron} onOk={handleOk} />,
      className: 'cron-modal',
    });
  });
};

export default Cron;
