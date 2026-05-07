import { BranchesOutlined, ExportOutlined } from "@ant-design/icons";
import {
  Alert,
  Button,
  DatePicker,
  Input,
  Space,
  Table,
  type TableColumnsType,
  type TableProps,
  Tooltip,
  theme,
} from "antd";
import { selectDataExporter } from "components/modal/select-export";
import dayjs from "dayjs";
import prettyBytes from "pretty-bytes";
import { useCallback, useEffect, useMemo, useState } from "react";
import { formatTime } from "utils/format";

import { TableFilter, queryDataStore } from "api/data";
import i18n from "i18n";
import { Link } from "react-router-dom";
import { Page } from "types/Page";
import { DataStoreMeta } from "types/Table";
import {
  BooleanParam,
  NumberParam,
  StringParam,
  useQueryParams,
} from "use-query-params";

const getColumns = (
  onExport: (id: string) => void
): TableColumnsType<DataStoreMeta> => {
  return [
    {
      title: "数据集",
      ellipsis: { showTitle: false },
      render: ({ id, name }: DataStoreMeta) => (
        <Tooltip placement="topLeft" title={name}>
          <Link to={`/data/${id}/detail`}>{name}</Link>
        </Tooltip>
      ),
    },
    {
      dataIndex: "created",
      title: i18n("popup_data_column_time"),
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
      dataIndex: "count",
      align: "center",
      width: 100,
      title: i18n("popup_data_column_count"),
      sorter: true,
    },
    {
      dataIndex: "bytes",
      align: "center",
      width: 100,
      title: i18n("popup_data_column_bytes"),
      render: (bytes: number) => prettyBytes(bytes),
    },
    {
      title: i18n("popup_data_column_operate"),
      width: 100,
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
          <Tooltip title="数据清洗">
            <Link to={`/data/${record.id}/clean`} onClick={(e) => e.stopPropagation()}>
              <Button type="text" icon={<BranchesOutlined />} />
            </Link>
          </Tooltip>
        </>
      ),
    },
  ];
};

const Store = () => {
  const {
    token: { colorBgContainer },
  } = theme.useToken();
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

  const columns = useMemo(() => {
    const handleExport = async () => {
      await selectDataExporter();
    };
    return getColumns(handleExport);
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
      <Alert
        message="数据集按任务聚合展示采集结果，可进入详情查看任务级数据，或打开清洗工作台进行预览和导出。"
        type="info"
        showIcon
      />
      <Space style={{ margin: "16px 0" }}>
        <Input
          addonBefore="数据集"
          onChange={(e) => setQuery({ name: e.target.value })}
        />
        <DatePicker.RangePicker
          showTime
          changeOnBlur
          style={{ minWidth: 350 }}
        />
        <Button type="primary">查询</Button>
      </Space>
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
          onChange: (page, size) => setQuery({ page: page - 1, size }),
        }}
      />
    </div>
  );
};

export default Store;
