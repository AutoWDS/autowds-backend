import { ArrowRightOutlined } from "@ant-design/icons";
import { Button, Col, Input, Row, Select, Space, type GetProp } from "antd";
import { queryStoreSchema } from "api/data";
import { useEffect, useMemo, useState } from "react";
import { Field, Graph, getAllField } from "types/NodeTypes";
import { DataStoreMeta } from "types/Table";
import { FieldMapping } from "types/Task";

type SelectOption = NonNullable<GetProp<typeof Select, "options">>[number];

interface FieldMappingProps {
  value?: FieldMapping;
  rule?: Graph;
  onChange?: (value: any) => void;
  store: DataStoreMeta;
}

const FieldSetting = ({
  graphField,
  options,
  value,
  onChange,
}: {
  graphField?: Field;
  options?: SelectOption[];
  value?: string;
  onChange?: (value?: string) => void;
}) => {
  return (
    <Row gutter={8}>
      <Col flex={1}>
        <Input disabled value={graphField?.name || "<未命名字段>"} />
      </Col>
      <Col>
        <Button type="text" icon={<ArrowRightOutlined />} />
      </Col>
      <Col flex={1}>
        <Select value={value} options={options} onChange={onChange} />
      </Col>
    </Row>
  );
};

const FieldMappingSettings = ({
  value = {},
  store,
  rule,
  onChange,
}: FieldMappingProps) => {
  const [options, setOptions] = useState<SelectOption[]>();
  useEffect(() => {
    (async () => {
      const fields = await queryStoreSchema(store.id);
      setOptions(fields.map((f) => ({ label: f.name, value: f.name })));
    })();
  }, [store.id]);

  const graphFields = useMemo(() => {
    return getAllField(rule as Graph);
  }, [rule]);

  const handleChange = (fieldId: string, rdbFieldName?: string) => {
    onChange && onChange({ ...value, [fieldId]: rdbFieldName });
  };

  return (
    <Space direction="vertical" style={{ width: "100%" }}>
      {graphFields.map((f, i) => (
        <FieldSetting
          key={i}
          graphField={f}
          options={options}
          value={value[f.id]}
          onChange={(v?: string) => handleChange(f.id, v)}
        />
      ))}
    </Space>
  );
};

export default FieldMappingSettings;
