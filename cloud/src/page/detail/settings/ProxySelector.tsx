import { Select, type GetProp, type SelectProps } from "antd";
import { useEffect, useState } from "react";

type SelectOption = NonNullable<GetProp<typeof Select, "options">>[number];

const ProxySelector = (props: SelectProps) => {
  const [proxys, setProxys] = useState<SelectOption[]>();
  useEffect(() => {
    setProxys([{ label: "系统默认", value: 0 }]);
  }, []);
  return <Select {...props} options={proxys} />;
};

export default ProxySelector;
