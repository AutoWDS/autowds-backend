import type { SelectProps } from "antd";
import { queryDataStore } from "api/data";
import DebounceSelect from "components/select/DebounceSelect";
import { keyBy } from "lodash";
import { useState } from "react";
import { DataStoreMeta } from "types/Table";

interface StoreSelectorProps extends Partial<SelectProps> {
  value?: DataStoreMeta;
  onChange?: (value?: DataStoreMeta) => void;
}

const StoreSelector = ({ value, onChange, ...props }: StoreSelectorProps) => {
  const [idMap, setIdMap] = useState<{ [id: string]: DataStoreMeta }>();
  const getFetchOptions = async (name: string) => {
    const page = await queryDataStore({ name });
    setIdMap(keyBy(page.content, "id"));
    return page.content.map(({ name, id }) => ({
      label: name,
      value: id,
    }));
  };
  const handleOnChange = (id: string) => {
    onChange && onChange(idMap ? idMap[id] : undefined);
  };
  return (
    <DebounceSelect
      {...props}
      fetchOptions={getFetchOptions}
      value={value?.id}
      onChange={handleOnChange}
    />
  );
};

export default StoreSelector;
