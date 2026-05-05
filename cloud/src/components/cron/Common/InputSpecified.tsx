import { Select, type GetProp } from "antd";

type SelectOption = NonNullable<GetProp<typeof Select, "options">>[number];

interface InputSpecifiedProps {
  disabled: boolean;
  unit: string;
  options: SelectOption[];
  value: string;
  onChange: (v: string) => void;
}

function InputSpecified(props: InputSpecifiedProps) {
  const { disabled, unit, options, value, onChange } = props;

  return (
    <span
      onClick={(e) => {
        // https://github.com/ant-design/ant-design/issues/25959
        e.preventDefault();
        e.stopPropagation();
      }}
    >
      指定具体{unit + " "}
      <Select
        mode="multiple"
        tokenSeparators={[","]}
        allowClear
        disabled={disabled}
        value={value}
        options={options}
        onChange={onChange}
        style={{ minWidth: 200 }}
      />
    </span>
  );
}

export default InputSpecified;
