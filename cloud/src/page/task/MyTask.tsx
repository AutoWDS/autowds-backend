import { DeleteOutlined } from "@ant-design/icons";
import {
  Button,
  DatePicker,
  Input,
  Popconfirm,
  Space,
  Table,
  type TableColumnsType,
  Tooltip,
} from "antd";
import { deleteTask, queryTask } from "api/task";
import i18n from "i18n";
import dayjs from "dayjs";
import { useCallback, useEffect, useMemo, useState } from "react";
import { Link } from "react-router-dom";
import {
  DateTimeParam,
  NumberParam,
  StringParam,
  useQueryParams,
} from "use-query-params";
import { Page } from "types/Page";
import { Task } from "types/Task";

const getColumns = (
  onDelete: (id: string) => void,
  onRunCloudTask: (record: Task) => void
) => [
  {
    title: i18n("popup_task_column_name"),
    render: ({ name, id }: Task) => (
      <Link to={`/task/myself/${id}`}>{name}</Link>
    ),
  },
  {
    dataIndex: "created",
    title: i18n("popup_task_column_created"),
    width: 100,
    align: "center",
    ellipsis: { showTitle: false },
    render: (created: string) => {
      return <Tooltip title={created}>{created}</Tooltip>;
    },
  },
  {
    title: i18n("popup_task_column_status"),
    width: 100,
    align: "center",
    render: (record: Task) => {
      return i18n("popup_task_actions_run");
    },
  },
  {
    title: i18n("popup_task_column_cron"),
    width: 140,
    align: "center",
    render: (record: Task) => {
      const { id, data } = record;
      const cron = data?.schedule?.cron;
      return (
        <Tooltip title={i18n("popup_task_set_cron_tooltip")} className="cron">
          <Link to={`/task/myself/${id}/settings`}>
            {cron || i18n("popup_task_set_cron_cloud")}
          </Link>
        </Tooltip>
      );
    },
  },
  {
    title: i18n("popup_task_column_actions"),
    width: 90,
    align: "center",
    render: (record: Task) => {
      return (
        <>
          <Popconfirm
            title={i18n("popup_task_actions_deleteTip")}
            onConfirm={() => onDelete(record.id)}
          >
            <Button type="text" icon={<DeleteOutlined />} />
          </Popconfirm>
        </>
      );
    },
  },
];

const MyTask = ({ lastImportTime }: { lastImportTime?: string }) => {
  const [query, setQuery] = useQueryParams({
    name: StringParam,
    page: NumberParam,
    size: NumberParam,
    startTime: DateTimeParam,
    endTime: DateTimeParam,
  });
  const [loading, setLoading] = useState(true);
  const [page, setPage] = useState({} as Page<Task>);
  const name = query.name || "";
  const pageNum = query.page || 1;
  const pageSize = query.size || 10;
  const startTime = query.startTime ? query.startTime.toISOString() : "";
  const endTime = query.endTime ? query.endTime.toISOString() : "";

  const fetch = useCallback(
    async () => {
      setLoading(true);
      try {
        const page = await queryTask(name, pageNum, pageSize, startTime, endTime);
        setPage(page);
      } catch (e) {
        console.log(e);
      }
      setLoading(false);
    },
    [name, pageNum, pageSize, startTime, endTime]
  );

  useEffect(() => {
    fetch();
  }, [lastImportTime, fetch]);

  const columns = useMemo(() => {
    const handleDelete = async (recordId: string) => {
      await deleteTask(recordId);
      fetch();
    };
    const handleRunCloudTask = async ({ id }: Task) => {
      console.log("cloud task", id);
    };
    return getColumns(handleDelete, handleRunCloudTask);
  }, [fetch]);

  return (
    <>
      <Space style={{ marginBottom: 16 }}>
        <Input
          value={name}
          placeholder="输入任务名"
          addonBefore="任务名"
          onChange={(e) => setQuery({ name: e.target.value })}
        />
        <DatePicker.RangePicker
          showTime
          changeOnBlur
          style={{ minWidth: 350 }}
          value={[
            query.startTime ? dayjs(query.startTime) : null,
            query.endTime ? dayjs(query.endTime) : null,
          ]}
          onChange={(dates) => {
            setQuery({
              startTime: dates?.[0]?.toDate(),
              endTime: dates?.[1]?.toDate(),
            });
          }}
        />
      </Space>
      <Table
        className="ant-table-fit-parent"
        size="small"
        rowKey="id"
        columns={columns as TableColumnsType<Task>}
        loading={loading}
        dataSource={page.content}
        scroll={{ y: "100%" }}
        style={{ height: "calc(100% - 48px)" }}
        pagination={{
          current: pageNum,
          total: page.totalElements,
          pageSize: page.size,
          onChange: (p) => setQuery({ page: p }),
        }}
      />
    </>
  );
};

export default MyTask;
