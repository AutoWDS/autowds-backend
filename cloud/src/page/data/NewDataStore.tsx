import { Button, Form, Input, Select, theme } from "antd";
import { saveDataStore } from "api/data";
import { optionsForEnumDesc } from "components/select/options";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { TableMetaTypeDesc } from "types/Table";
import StoreFields, { fieldsRule } from "../store/StoreFields";

const TableMetaTypeDescOptions = optionsForEnumDesc(TableMetaTypeDesc);

const NewDataStore = () => {
  const [form] = Form.useForm();
  const navigate = useNavigate();
  const [hasFields, setHasFields] = useState(false);
  const {
    token: { colorBgContainer },
  } = theme.useToken();
  const [loading, setLoading] = useState(false);
  const handleFinish = async (values: any) => {
    console.log(values);
    setLoading(true);
    await saveDataStore(values);
    setLoading(false);
    navigate(-1);
  };
  const handleValueChange = ({ type }: any) => {
    if (type) {
      setHasFields(type === "RDB");
    }
  };
  return (
    <Form
      form={form}
      labelCol={{ span: 4 }}
      wrapperCol={{ span: 16 }}
      onFinish={handleFinish}
      onValuesChange={handleValueChange}
      style={{
        width: "100%",
        height: "100%",
        overflow: "auto",
        background: colorBgContainer,
        padding: 16,
      }}
    >
      <Form.Item
        label="表名"
        name="name"
        hasFeedback
        rules={[{ required: true, message: "请填写表名" }]}
      >
        <Input maxLength={30} />
      </Form.Item>
      <Form.Item
        label="类型"
        name="type"
        tooltip={{
          title:
            "文档型使用JSON存储支持任意数据字段，但是数据无法保证结构化的数据质量；\n关系型需要预先定义字段，采集任务需要字段一一对应",
        }}
        rules={[{ required: true, message: "请选择表类型" }]}
      >
        <Select options={TableMetaTypeDescOptions} />
      </Form.Item>
      {hasFields ? (
        <Form.Item label="字段" name="fields" required rules={fieldsRule}>
          <StoreFields />
        </Form.Item>
      ) : null}
      <Form.Item wrapperCol={{ offset: 4, span: 16 }}>
        <Button loading={loading} type="primary" htmlType="submit">
          提交
        </Button>
      </Form.Item>
    </Form>
  );
};

export default NewDataStore;
