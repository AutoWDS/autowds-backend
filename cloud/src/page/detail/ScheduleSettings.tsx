import { Button, Form, Select } from "antd";
import { saveScheduleConfig, getScheduleConfig } from "api/task";
import { optionsForEnumDesc } from "components/select/options";
import { useParams } from "react-router-dom";
import { SchedulerTypeDesc } from "types/Task";
import CronSelector from "./settings/CronSelector";
import ProxySelector from "./settings/ProxySelector";
import { useEffect } from "react";

const ScheduleSettings = () => {
  const { taskId } = useParams();
  const [form] = Form.useForm();

  useEffect(() => {
    (async () => {
      if (taskId) {
        const conf = await getScheduleConfig(taskId as string);
        form.setFieldsValue(conf);
      }
    })();
  }, [taskId, form]);

  const handleFinish = async (values: any) => {
    console.log(values);
    await saveScheduleConfig(taskId as string, values);
  };

  return (
    <Form
      form={form}
      labelCol={{ span: 3 }}
      wrapperCol={{ span: 16 }}
      onFinish={handleFinish}
    >
      <Form.Item
        label="调度时间"
        name="cron"
        hasFeedback
        rules={[{ required: true, message: "请选择调度时间" }]}
      >
        <CronSelector />
      </Form.Item>
      <Form.Item
        label="执行方式"
        name="type"
        tooltip={{
          title:
            "爬虫执行程序，快速HTTP调度使用简单HTTP请求方式性能更高但对于单页应用等网页兼容性不好；浏览器调度使用Chrome浏览器执行因此能在任意网站爬取数据",
        }}
        rules={[{ required: true, message: "请选择执行方式" }]}
      >
        <Select options={optionsForEnumDesc(SchedulerTypeDesc)} />
      </Form.Item>
      <Form.Item
        label="IP代理池"
        name="proxyId"
        tooltip={{
          title: "使用IP代理池能有效防止网站针对IP频繁访问的反爬虫策略",
        }}
        rules={[{ required: true, message: "请选择IP代理池" }]}
      >
        <ProxySelector />
      </Form.Item>
      <Form.Item wrapperCol={{ offset: 3, span: 16 }}>
        <Button type="primary" htmlType="submit">
          提交
        </Button>
      </Form.Item>
    </Form>
  );
};

export default ScheduleSettings;
