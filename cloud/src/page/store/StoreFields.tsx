import { DeleteOutlined, PlusOutlined } from "@ant-design/icons";
import { Button, Form, type FormRule, Input, Space, Tooltip } from "antd";
import type { FormInstance as RcFormInstance, RuleObject } from "rc-field-form/lib/interface";
import _ from "lodash";
import { RDBField } from "types/Table";

interface StoreFieldsProps {
  value?: RDBField[];
  onChange?: (value: RDBField[]) => void;
}

interface RDBFieldProps extends RDBField {
  onNameChange: (name: string) => void;
  onDefaultValueChange: (name: string) => void;
  onDelete: () => void;
}

export const fieldsRule: FormRule[] = [
  (form: RcFormInstance) => ({
    validator(rule: RuleObject, fields: RDBField[]) {
      if (form.getFieldValue("type") === "RDB") {
        if (!fields || !fields.length) {
          return Promise.reject("请添加表字段");
        } else if (fields.some((f) => !f.name)) {
          return Promise.reject("字段名不能为空");
        }
        const nameCount = _.chain(fields)
          .map("name")
          .reduce((r: any, n: string) => {
            r[n] = (r[n] || 0) + 1;
            return r;
          }, {})
          .value();

        if (_.some(nameCount, (c: number) => c > 1)) {
          return Promise.reject("字段名不能同名");
        }
      }
      return Promise.resolve();
    },
  }),
];

const StoreField = ({
  name,
  defaultValue,
  onNameChange,
  onDefaultValueChange,
  onDelete,
}: RDBFieldProps) => {
  const { status, errors } = Form.Item.useStatus();
  console.log(status, errors);
  return (
    <div style={{ display: "flex", width: "100%", gap: 8 }}>
      <Tooltip title={name ? undefined : "字段名不能为空"}>
        <Input
          style={{ flex: 1 }}
          placeholder="字段名"
          maxLength={30}
          value={name}
          status={status === "error" && !name ? "error" : ""}
          onChange={(e) => onNameChange(e.target.value)}
        />
      </Tooltip>
      <Input
        style={{ flex: 1 }}
        placeholder="默认值"
        value={defaultValue}
        status=""
        onChange={(e) => onDefaultValueChange(e.target.value)}
      />
      <Button type="dashed" icon={<DeleteOutlined />} onClick={onDelete} />
    </div>
  );
};

const StoreFields = ({ value = [], onChange }: StoreFieldsProps) => {
  const fields = value;
  const handleAdd = () => {
    onChange && onChange([...fields, { name: "", defaultValue: "" }]);
  };
  const handleDelete = (index: number) => {
    onChange && onChange(fields.filter((v, i) => i !== index));
  };
  const handleChange = (index: number, key: string, value: string) => {
    onChange &&
      onChange(
        fields.map((f, i) => (i === index ? { ...f, [key]: value } : f))
      );
  };
  return (
    <Space direction="vertical" style={{ width: "100%" }}>
      {_.map(fields, (field, i) => (
        <StoreField
          key={i}
          {...field}
          onNameChange={(v) => handleChange(i, "name", v)}
          onDefaultValueChange={(v) => handleChange(i, "defaultValue", v)}
          onDelete={() => handleDelete(i)}
        />
      ))}
      <Button type="dashed" block icon={<PlusOutlined />} onClick={handleAdd} />
    </Space>
  );
};

export default StoreFields;
