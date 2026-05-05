import { DeleteOutlined, ExportOutlined } from "@ant-design/icons";
import {
  Button,
  DatePicker,
  Input,
  Popconfirm,
  Space,
  Table,
  type TableColumnsType,
  type TableProps,
  Tooltip,
  theme,
} from "antd";
import dayjs from "dayjs";
import { useCallback, useEffect, useMemo, useState } from "react";
import { formatTime } from "utils/format";

import { TableFilter, deleteDataStore, queryDataStore } from "api/data";
import { selectDB } from "components/modal/select-db";
import i18n from "i18n";
import { Link, useNavigate } from "react-router-dom";
import { Page } from "types/Page";
import { DataStoreMeta, TableMetaType, TableMetaTypeDesc } from "types/Table";
import {
  BooleanParam,
  NumberParam,
  StringParam,
  useQueryParams,
} from "use-query-params";

const getColumns = (
  onDelete: (id: string) => void,
  onExport: (id: string) => void
): TableColumnsType<DataStoreMeta> => {
  return [
    {
      title: "名称",
      ellipsis: { showTitle: false },
      render: ({ id, name }: DataStoreMeta) => (
        <Tooltip placement="topLeft" title={name}>
          <Link to={`/data/${id}/detail`}>{name}</Link>
        </Tooltip>
      ),
    },
    {
      dataIndex: "created",
      title: "创建时间",
      sorter: true,
      align: "center",
      width: 100,
      render: (time: number) => (
        <Tooltip title={dayjs(time).format("YYYY-MM-DD HH:mm:ss")}>
          {formatTime(time)}
        </Tooltip>
      ),
    },
    {
      dataIndex: "type",
      align: "center",
      width: 100,
      title: i18n("popup_data_column_type"),
      render: (type: TableMetaType) => i18n(TableMetaTypeDesc[type]),
    },
    {
      title: i18n("popup_data_column_operate"),
      width: 90,
      align: "center",
      render: (record: DataStoreMeta) => (
        <>
          <Tooltip title={i18n("popup_data_actions_export")}>
            <Button
              type="text"
              icon={<ExportOutlined />}
              onClick={(e) => {
                onExport(record.id);
                e.stopPropagation();
              }}
            />
          </Tooltip>
          <Popconfirm
            title={i18n("popup_data_actions_deleteTip")}
            onConfirm={() => onDelete(record.id)}
          >
            <Button
              type="text"
              icon={<DeleteOutlined />}
              onClick={(e) => e.stopPropagation()}
            />
          </Popconfirm>
        </>
      ),
    },
  ];
};

const DB = () => {
  const {
    token: { colorBgContainer },
  } = theme.useToken();
  const navigate = useNavigate();
  const [query, setQuery] = useQueryParams({
    name: StringParam,
    sort: StringParam,
    page: NumberParam,
    size: NumberParam,
    desc: BooleanParam,
  });
  const [data, setData] = useState<Page<DataStoreMeta>>();
  const [loading, setLoading] = useState(true);

  const fetchData = useCallback(async () => {
    setLoading(true);
    const data = await queryDataStore(query as TableFilter);
    setData(data);
    setLoading(false);
  }, [query]);

  useEffect(() => {
    fetchData();
  }, [fetchData]);

  const handleNewDB = async () => {
    const type = await selectDB();
    navigate(`/data/db/new/${type}`);
  };

  const columns = useMemo(() => {
    const handleDelete = async (storeId: string) => {
      await deleteDataStore(storeId);
      await fetchData();
    };
    const handleExport = async () => {
      await selectDB();
    };
    return getColumns(handleDelete, handleExport);
  }, [fetchData]);

  const handleChange: NonNullable<TableProps<DataStoreMeta>["onChange"]> = (
    _pagination,
    _filters,
    sorter
  ) => {
    const currentSorter = Array.isArray(sorter) ? sorter[0] : sorter;
    const { field, order } = currentSorter || {};
    const desc = order === "descend";
    setQuery({ sort: field as string, desc });
  };

  return (
    <div style={{ display: "flex", flexDirection: "column", height: "100%" }}>
      <div
        style={{
          display: "flex",
          marginBottom: 16,
        }}
      >
        <Space>
          <Input
            addonBefore="名称"
            onChange={(e) => setQuery({ name: e.target.value })}
          />
          <DatePicker.RangePicker
            showTime
            changeOnBlur
            style={{ minWidth: 350 }}
          />
        </Space>
        <div style={{ flex: 1 }} />
        <Button type="primary" onClick={handleNewDB}>
          新建
        </Button>
      </div>
      <Table
        rowKey="id"
        size="middle"
        className="ant-table-fit-parent data-table"
        dataSource={data?.content}
        columns={columns}
        loading={loading}
        scroll={{ y: "100%" }}
        showSorterTooltip={false}
        onChange={handleChange}
        style={{ flex: 1, borderRadius: 8, background: colorBgContainer }}
        pagination={{
          pageSize: data?.size,
          current: (data?.number || 0) + 1,
          total: data?.totalElements,
          onChange: (page, size) => setQuery({ page, size }),
        }}
      />
    </div>
  );
};

export default DB;
