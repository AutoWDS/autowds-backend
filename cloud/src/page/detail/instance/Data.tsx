import { queryInstanceCaptureData } from "api/instance";
import { getTask } from "api/task";
import { Table } from "antd";
import type { ColumnsType } from "antd/es/table";
import type { ReactNode } from "react";
import { useCallback, useEffect, useMemo, useState } from "react";
import { useParams } from "react-router-dom";
import type { Graph } from "types/NodeTypes";
import { NodeType } from "types/NodeTypes";
import type { DetailNodeConfig, Field, ListNodeConfig } from "types/NodeTypes";
import type { Page } from "types/Page";
import type { Task } from "types/Task";
import type { TaskInstanceCaptureItem } from "types/Task";

/** 与实例侧 `field_from_json` 一致：行内 key 优先 `name`，否则 `id`。 */
function fieldPayloadKey(f: Field): string {
  const n = f.name?.trim();
  if (n) return n;
  return f.id;
}

type PayloadColumnSpec = {
  key: string;
  nodeId: string;
  dataKey: string;
  title: string;
};

function collectPayloadColumns(graph: Graph | null | undefined): PayloadColumnSpec[] {
  if (!graph?.nodes?.length) return [];
  const out: PayloadColumnSpec[] = [];
  for (const node of graph.nodes) {
    if (node.action !== NodeType.list && node.action !== NodeType.detail) continue;
    const cfg = node.config as ListNodeConfig | DetailNodeConfig;
    const fields = cfg?.fields;
    if (!Array.isArray(fields) || fields.length === 0) continue;
    for (const f of fields) {
      const dataKey = fieldPayloadKey(f);
      if (!dataKey) continue;
      const label = f.name?.trim() || f.id;
      out.push({
        key: `${node.id}:${dataKey}`,
        nodeId: node.id,
        dataKey,
        title: `${node.id} · ${label}`,
      });
    }
  }
  return out;
}

function formatCellValue(v: unknown): string {
  if (v === undefined || v === null) return "—";
  if (typeof v === "string" || typeof v === "number" || typeof v === "boolean") return String(v);
  try {
    return JSON.stringify(v);
  } catch {
    return "—";
  }
}

function cellFromPayload(
  payload: Record<string, unknown>,
  nodeId: string,
  dataKey: string
): string {
  const nodeObj = payload[nodeId];
  if (!nodeObj || typeof nodeObj !== "object" || Array.isArray(nodeObj)) return "—";
  const row = nodeObj as Record<string, unknown>;
  return formatCellValue(row[dataKey]);
}

const PAYLOAD_CELL_MAX_WIDTH = 120;

/** http(s) 为可点击链接；省略与悬停全文由列上 `ellipsis` + 固定列宽处理。 */
function renderPayloadCell(text: string): ReactNode {
  if (text === "—") return text;
  const trimmed = text.trim();
  const isWebUrl = trimmed.startsWith("http://") || trimmed.startsWith("https://");
  const href = isWebUrl ? trimmed : "";
  if (isWebUrl) {
    return (
      <a href={href} target="_blank" rel="noreferrer" onClick={(e) => e.stopPropagation()}>
        {text}
      </a>
    );
  }
  return text;
}

const Data = () => {
  const { taskId, instanceId } = useParams();
  const [graph, setGraph] = useState<Graph | null>(null);
  const [taskLoading, setTaskLoading] = useState(true);
  const [tableLoading, setTableLoading] = useState(true);
  const [data, setData] = useState<Page<TaskInstanceCaptureItem>>();
  const [pagination, setPagination] = useState({ page: 1, size: 20 });

  useEffect(() => {
    if (!taskId) return;
    let cancelled = false;
    (async () => {
      setTaskLoading(true);
      try {
        const task = (await getTask(taskId)) as Task & { rule?: Graph };
        if (!cancelled && task?.rule && typeof task.rule === "object" && "nodes" in task.rule) {
          setGraph(task.rule as Graph);
        } else if (!cancelled) {
          setGraph(null);
        }
      } catch {
        if (!cancelled) setGraph(null);
      } finally {
        if (!cancelled) setTaskLoading(false);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [taskId]);

  const fetchData = useCallback(async () => {
    if (!taskId || !instanceId) return;
    setTableLoading(true);
    try {
      const res = await queryInstanceCaptureData({
        taskId,
        instanceId,
        page: pagination.page,
        size: pagination.size,
      });
      setData(res);
    } finally {
      setTableLoading(false);
    }
  }, [taskId, instanceId, pagination]);

  useEffect(() => {
    fetchData();
  }, [fetchData]);

  const payloadColumns = useMemo(() => collectPayloadColumns(graph), [graph]);

  const columns = useMemo((): ColumnsType<TaskInstanceCaptureItem> => {
    const base: ColumnsType<TaskInstanceCaptureItem> = [
      {
        title: "ID",
        dataIndex: "id",
        width: 88,
      },
      {
        title: "采集时间",
        dataIndex: "createdAt",
        width: 200,
      },
    ];
    const dynamic: ColumnsType<TaskInstanceCaptureItem> = payloadColumns.map((spec) => ({
      key: spec.key,
      title: spec.title,
      width: PAYLOAD_CELL_MAX_WIDTH,
      ellipsis: { showTitle: true },
      onCell: () => ({
        style: { maxWidth: PAYLOAD_CELL_MAX_WIDTH, verticalAlign: "top" as const },
      }),
      render: (_: unknown, record: TaskInstanceCaptureItem) =>
        renderPayloadCell(cellFromPayload(record.payload, spec.nodeId, spec.dataKey)),
    }));
    return [...base, ...dynamic];
  }, [payloadColumns]);

  const loading = taskLoading || tableLoading;

  return (
    <div style={{ display: "flex", flexDirection: "column", width: "100%", height: "100%" }}>
      <Table<TaskInstanceCaptureItem>
        rowKey="id"
        size="small"
        loading={loading}
        dataSource={data?.content}
        columns={columns}
        scroll={{ x: "max-content" }}
        pagination={{
          current: (data?.page ?? data?.number ?? 0) + 1,
          pageSize: data?.size ?? pagination.size,
          total: data?.total_elements ?? data?.totalElements ?? 0,
          showSizeChanger: true,
          showTotal: (total) => `共 ${total} 条`,
          onChange: (page, pageSize) => {
            setPagination({ page, size: pageSize ?? 20 });
          },
        }}
      />
    </div>
  );
};

export default Data;
