import { BarsOutlined, CodeOutlined } from "@ant-design/icons";
import { Button, Space, Table, Tag, Tooltip } from "antd";
import { queryInstance } from "api/instance";
import { optionsForEnumDesc } from "components/select/options";
import { useCallback, useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";
import { Page } from "types/Page";
import { InstanceStatusDesc, TaskInstance } from "types/Task";

const StatusDescOptions = optionsForEnumDesc(InstanceStatusDesc);

const StatusColor: Record<string, string> = {
  RUNNING: "processing",
  SUCCESS: "success",
  FAILED: "error",
};

const getColumn = (taskId: string) => {
  return [
    {
      title: "实例ID",
      dataIndex: "id",
      width: 80,
    },
    {
      title: "调度时间",
      dataIndex: "created",
      render: (created: string, record: TaskInstance) => (
        <Link to={`/task/myself/${taskId}/instances/${record.id}`}>
          {created}
        </Link>
      ),
    },
    {
      title: "状态",
      dataIndex: "status",
      width: 100,
      render: (status: string) => (
        <Tag color={StatusColor[status] || "default"}>
          {InstanceStatusDesc[status as keyof typeof InstanceStatusDesc] || status}
        </Tag>
      ),
    },
    {
      title: "采集数据",
      dataIndex: "dataCount",
      width: 100,
    },
    {
      title: "日志",
      dataIndex: "logKey",
      width: 80,
      render: (logKey: string | undefined, record: TaskInstance) =>
        logKey ? (
          <Link to={`/task/myself/${taskId}/instances/${record.id}/log`}>
            <Button type="link" size="small" icon={<CodeOutlined />}>
              查看
            </Button>
          </Link>
        ) : (
          "—"
        ),
    },
    {
      title: "错误信息",
      dataIndex: "errorMessage",
      ellipsis: {
        showTitle: false,
      },
      render: (errorMessage: string | undefined) =>
        errorMessage ? (
          <Tooltip placement="topLeft" title={errorMessage}>
            <span>{errorMessage}</span>
          </Tooltip>
        ) : (
          "—"
        ),
    },
    {
      title: "操作",
      width: 120,
      render: (record: TaskInstance) => (
        <Space>
          <Link to={`/task/myself/${taskId}/instances/${record.id}/data`}>
            <Button type="link" size="small" icon={<BarsOutlined />}>
              数据
            </Button>
          </Link>
        </Space>
      ),
    },
  ];
};

const InstanceRecords = () => {
  const { taskId } = useParams();
  const [loading, setLoading] = useState(true);
  const [data, setData] = useState<Page<TaskInstance>>();
  const [pagination, setPagination] = useState({ page: 1, size: 20 });

  const fetchData = useCallback(async () => {
    if (!taskId) return;
    setLoading(true);
    const res = await queryInstance({
      taskId,
      page: pagination.page,
      size: pagination.size,
    });
    setLoading(false);
    setData(res);
  }, [taskId, pagination]);

  useEffect(() => {
    fetchData();
  }, [fetchData]);

  return (
    <div style={{ display: "flex", flexDirection: "column", width: "100%" }}>
      <Table
        rowKey="id"
        size="small"
        loading={loading}
        dataSource={data?.content}
        columns={getColumn(taskId || "")}
        pagination={{
          current: data?.number || 1,
          pageSize: data?.size || 20,
          total: data?.totalElements || 0,
          showSizeChanger: true,
          showTotal: (total) => `共 ${total} 条`,
          onChange: (page, pageSize) => {
            setPagination({ page, size: pageSize || 20 });
          },
        }}
      />
    </div>
  );
};

export default InstanceRecords;
