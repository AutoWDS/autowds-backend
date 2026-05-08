import {
  BranchesOutlined,
  CloseOutlined,
  DownloadOutlined,
  EyeOutlined,
  PlayCircleOutlined,
  SaveOutlined,
  SettingOutlined,
} from "@ant-design/icons";
import {
  Alert,
  Button,
  Card,
  Col,
  Empty,
  Input,
  List,
  message,
  Modal,
  Row,
  Segmented,
  Select,
  Space,
  Table,
  Tag,
  Typography,
} from "antd";
import {
  addEdge,
  applyEdgeChanges,
  applyNodeChanges,
  Background,
  Connection,
  Controls,
  Edge,
  EdgeChange,
  Handle,
  MiniMap,
  Node,
  NodeChange,
  NodeProps,
  Position,
  ReactFlow,
  ReactFlowProvider,
  useReactFlow,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import {
  CleanExportFormat,
  CleanNodeType,
  CleanPipeline as CleanPipelineDTO,
  CleanPreviewResp,
  JsonObject,
  JsonValue,
  exportCleanPipeline,
  previewCleanPipeline,
  queryCleanPipelines,
  saveCleanPipeline,
  validateCleanPipeline,
} from "api/dataClean";
import { queryStoreData } from "api/data";
import { useCallback, useEffect, useMemo, useState } from "react";
import { useParams } from "react-router-dom";

const { TextArea } = Input;

type TransformNodeType = Exclude<CleanNodeType, "source" | "sink">;
type PreviewMode = "rows" | "columns" | "both";

interface ColumnValueProfile {
  value: JsonValue;
  label: string;
  count: number;
}

interface ColumnProfile {
  field: string;
  values: ColumnValueProfile[];
}

const nodeOptions: { value: TransformNodeType; label: string }[] = [
  { value: "selectRename", label: "选择/重命名" },
  { value: "trim", label: "去除空白" },
  { value: "replace", label: "文本替换" },
  { value: "typeCast", label: "类型转换" },
  { value: "filter", label: "过滤" },
  { value: "dedupe", label: "去重" },
  { value: "derivedField", label: "派生字段" },
];

const defaultPipeline: CleanPipelineDTO = {
  name: "默认清洗流程",
  nodes: [
    {
      id: "source",
      type: "source",
      label: "Source",
      position: { x: 80, y: 160 },
      params: {},
    },
    {
      id: "sink",
      type: "sink",
      label: "Sink",
      position: { x: 620, y: 160 },
      params: {},
    },
  ],
  edges: [{ id: "edge-source-sink", source: "source", target: "sink" }],
};

const defaultParams: Record<TransformNodeType, JsonObject> = {
  selectRename: { fields: [{ from: "name", to: "name" }] as JsonValue },
  trim: { fields: ["name"] },
  replace: { field: "name", from: "old", to: "new" },
  typeCast: { field: "price", target: "number" },
  filter: { field: "name", op: "isNotEmpty" },
  dedupe: { fields: ["name"] },
  derivedField: { field: "summary", template: ["$", "{name}"].join("") },
};

const nodeTitle: Record<CleanNodeType, string> = {
  source: "Source",
  selectRename: "选择/重命名",
  trim: "去除空白",
  replace: "文本替换",
  typeCast: "类型转换",
  filter: "过滤",
  dedupe: "去重",
  derivedField: "派生字段",
  sink: "Sink",
};

interface CleanNodeData extends Record<string, unknown> {
  label: string;
  nodeType: CleanNodeType;
  params?: JsonObject;
  onPreview?: (nodeId: string) => void;
  onSettings?: (nodeId: string) => void;
}

type FlowNode = Node<CleanNodeData>;
type FlowEdge = Edge;

function isJsonValue(value: JsonValue): value is JsonValue {
  return (
    value === null ||
    ["string", "number", "boolean", "object"].includes(typeof value)
  );
}

function parseJsonObject(value: string): JsonObject | null {
  try {
    const parsed = JSON.parse(value) as JsonValue;
    if (
      parsed &&
      typeof parsed === "object" &&
      !Array.isArray(parsed) &&
      isJsonValue(parsed)
    ) {
      return parsed as JsonObject;
    }
  } catch (e) {
    return null;
  }
  return null;
}

function toFlowNodes(pipeline: CleanPipelineDTO): FlowNode[] {
  return pipeline.nodes.map((node, index) => ({
    id: node.id,
    type: "cleanNode",
    position: node.position || { x: 80 + index * 220, y: 160 },
    data: {
      label: node.label || nodeTitle[node.type],
      nodeType: node.type,
    },
  }));
}

function toFlowEdges(pipeline: CleanPipelineDTO): FlowEdge[] {
  return pipeline.edges.map((edge) => ({
    id: edge.id,
    source: edge.source,
    target: edge.target,
    animated: true,
  }));
}

function paramsFromPipeline(pipeline: CleanPipelineDTO) {
  return pipeline.nodes.reduce<Record<string, JsonObject>>((acc, node) => {
    acc[node.id] = node.params;
    return acc;
  }, {});
}

function uniqueEdgeId(source: string, target: string) {
  return `edge-${source}-${target}-${Date.now()}`;
}

function isTransformNodeType(type: CleanNodeType): type is TransformNodeType {
  return type !== "source" && type !== "sink";
}

function openNodeSettings(
  nodeId: string,
  setSelectedId: (nodeId: string) => void,
  setParamModalOpen: (open: boolean) => void,
) {
  setSelectedId(nodeId);
  setParamModalOpen(true);
}

function buildPreviewPipeline(
  pipeline: CleanPipelineDTO,
  targetId: string,
): CleanPipelineDTO {
  const target = pipeline.nodes.find((node) => node.id === targetId);
  if (!target || target.type === "sink") return pipeline;

  const included = new Set<string>([targetId]);
  const stack = [targetId];
  while (stack.length) {
    const current = stack.pop() as string;
    pipeline.edges
      .filter((edge) => edge.target === current)
      .forEach((edge) => {
        if (!included.has(edge.source)) {
          included.add(edge.source);
          stack.push(edge.source);
        }
      });
  }

  const nodes = pipeline.nodes.filter((node) => included.has(node.id));
  const edges = pipeline.edges.filter(
    (edge) => included.has(edge.source) && included.has(edge.target),
  );
  const previewSinkId = "__preview_sink";
  return {
    ...pipeline,
    nodes: nodes.concat({
      id: previewSinkId,
      type: "sink",
      label: "Preview",
      position: {
        x: (target.position?.x || 0) + 260,
        y: target.position?.y || 0,
      },
      params: {},
    }),
    edges: edges.concat({
      id: `edge-${targetId}-${previewSinkId}`,
      source: targetId,
      target: previewSinkId,
    }),
  };
}

function valueBrief(value: JsonValue | undefined): string {
  if (value === undefined || value === null) return "";
  if (Array.isArray(value))
    return value.map(valueBrief).filter(Boolean).join(", ");
  if (typeof value === "object") return JSON.stringify(value);
  return String(value);
}

function isJsonObject(value: JsonValue): value is JsonObject {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function valueProfileKey(value: JsonValue | undefined): string {
  if (value === undefined) return "__undefined__";
  return JSON.stringify(value);
}

function buildColumnProfiles(
  records: JsonValue[],
  fields: string[],
): ColumnProfile[] {
  return fields.map((field) => {
    const counter = new Map<string, ColumnValueProfile>();
    records.forEach((record) => {
      if (!isJsonObject(record)) return;
      const value = record[field];
      const key = valueProfileKey(value);
      const current = counter.get(key);
      if (current) {
        current.count += 1;
      } else {
        counter.set(key, {
          value: value ?? null,
          label: value === undefined ? "(空)" : valueBrief(value) || "(空)",
          count: 1,
        });
      }
    });
    return {
      field,
      values: Array.from(counter.values()).sort((a, b) => b.count - a.count),
    };
  });
}

function cleanNodeSummary(type: CleanNodeType, params: JsonObject = {}) {
  if (type === "source") return ["当前数据集"];
  if (type === "sink") return ["输出结果"];

  if (type === "selectRename") {
    const fields = Array.isArray(params.fields) ? params.fields : [];
    return fields.slice(0, 3).map((field) => {
      if (field && typeof field === "object" && !Array.isArray(field)) {
        const from = valueBrief(field.from);
        const to = valueBrief(field.to);
        return `${from} → ${to}`;
      }
      return valueBrief(field);
    });
  }

  if (type === "trim" || type === "dedupe") {
    return [`字段：${valueBrief(params.fields) || "未配置"}`];
  }

  if (type === "replace") {
    return [
      `${valueBrief(params.field) || "字段"}：${valueBrief(params.from)} → ${valueBrief(params.to)}`,
    ];
  }

  if (type === "typeCast") {
    return [
      `${valueBrief(params.field) || "字段"} → ${valueBrief(params.target) || "类型"}`,
    ];
  }

  if (type === "filter") {
    return [
      `${valueBrief(params.field) || "字段"} ${valueBrief(params.op) || "条件"} ${valueBrief(params.value)}`,
    ];
  }

  if (type === "derivedField") {
    return [
      `${valueBrief(params.field) || "字段"} = ${valueBrief(params.template)}`,
    ];
  }

  return ["未配置"];
}

function CleanFlowNode({ id, data, selected }: NodeProps<FlowNode>) {
  const isEndpoint = data.nodeType === "source" || data.nodeType === "sink";
  const summaries = cleanNodeSummary(data.nodeType, data.params);

  return (
    <div
      style={{
        width: 220,
        border: `1px solid ${selected ? "#1677ff" : "#d9d9d9"}`,
        borderRadius: 8,
        background: "#fff",
        boxShadow: selected
          ? "0 0 0 2px rgba(22, 119, 255, 0.12)"
          : "0 2px 8px rgba(0, 0, 0, 0.08)",
        overflow: "hidden",
      }}
    >
      {data.nodeType === "source" ? null : (
        <Handle type="target" position={Position.Left} />
      )}
      {data.nodeType === "sink" ? null : (
        <Handle type="source" position={Position.Right} />
      )}
      <div
        style={{
          padding: "8px 10px",
          borderBottom: "1px solid #f0f0f0",
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          background: "#fafafa",
        }}
      >
        <Typography.Text strong ellipsis style={{ maxWidth: 170 }}>
          {data.label}
        </Typography.Text>
        <Tag
          color={isEndpoint ? "blue" : "green"}
          style={{ marginInlineEnd: 0 }}
        >
          {data.nodeType}
        </Tag>
      </div>
      <div style={{ padding: 10, minHeight: 72 }}>
        {summaries.length ? (
          summaries.map((summary, index) => (
            <Typography.Text
              key={`${summary}-${index}`}
              type="secondary"
              ellipsis
              style={{ display: "block", fontSize: 12, lineHeight: "22px" }}
            >
              {summary || "未配置"}
            </Typography.Text>
          ))
        ) : (
          <Typography.Text type="secondary" style={{ fontSize: 12 }}>
            未配置
          </Typography.Text>
        )}
      </div>
      <div
        style={{
          padding: 8,
          borderTop: "1px solid #f0f0f0",
          display: "flex",
          justifyContent: "center",
          gap: 8,
          background: "#fafafa",
        }}
      >
        <Button
          size="small"
          icon={<EyeOutlined />}
          onClick={(event) => {
            event.stopPropagation();
            data.onPreview?.(id);
          }}
        >
          预览
        </Button>
        <Button
          size="small"
          icon={<SettingOutlined />}
          onClick={(event) => {
            event.stopPropagation();
            data.onSettings?.(id);
          }}
        >
          设置
        </Button>
      </div>
    </div>
  );
}

const nodeTypes = {
  cleanNode: CleanFlowNode,
};

const CleanPipelineWorkbench = () => {
  const { storeId } = useParams();
  const { screenToFlowPosition } = useReactFlow<FlowNode, FlowEdge>();
  const [pipelineName, setPipelineName] = useState(
    defaultPipeline.name || "默认清洗流程",
  );
  const [nodes, setNodes] = useState<FlowNode[]>(() =>
    toFlowNodes(defaultPipeline),
  );
  const [edges, setEdges] = useState<FlowEdge[]>(() =>
    toFlowEdges(defaultPipeline),
  );
  const [nodeParams, setNodeParams] = useState<Record<string, JsonObject>>(() =>
    paramsFromPipeline(defaultPipeline),
  );
  const [selectedId, setSelectedId] = useState("source");
  const [paramText, setParamText] = useState("{}");
  const [samples, setSamples] = useState<JsonValue[]>([]);
  const [preview, setPreview] = useState<CleanPreviewResp>();
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [format, setFormat] = useState<CleanExportFormat>("json");
  const [paramModalOpen, setParamModalOpen] = useState(false);
  const [previewMode, setPreviewMode] = useState<PreviewMode>("both");
  const [valueDrafts, setValueDrafts] = useState<Record<string, string>>({});

  const selectedNode = nodes.find((node) => node.id === selectedId);
  const selectedNodeType = selectedNode?.data.nodeType;
  const pipeline = useMemo<CleanPipelineDTO>(
    () => ({
      name: pipelineName,
      nodes: nodes.map((node) => ({
        id: node.id,
        type: node.data.nodeType,
        label: node.data.label,
        position: node.position,
        params: nodeParams[node.id] || {},
      })),
      edges: edges.map((edge) => ({
        id: edge.id,
        source: edge.source,
        target: edge.target,
      })),
    }),
    [edges, nodeParams, nodes, pipelineName],
  );

  const loadInitialData = useCallback(async () => {
    if (!storeId) return;
    try {
      const [pipelines, page] = await Promise.all([
        queryCleanPipelines(storeId),
        queryStoreData(storeId, { desc: true }),
      ]);
      if (pipelines[0]) {
        setPipelineName(
          pipelines[0].name || pipelines[0].pipeline.name || "默认清洗流程",
        );
        setNodes(toFlowNodes(pipelines[0].pipeline));
        setEdges(toFlowEdges(pipelines[0].pipeline));
        setNodeParams(paramsFromPipeline(pipelines[0].pipeline));
        setSelectedId(pipelines[0].pipeline.nodes[0]?.id || "source");
      }
      setSamples(page.content.map((item) => item.data as JsonValue));
    } catch (e) {
      message.warning("清洗流程或样本数据加载失败，请确认后端接口可用");
    }
  }, [storeId]);

  useEffect(() => {
    loadInitialData();
  }, [loadInitialData]);

  useEffect(() => {
    setParamText(JSON.stringify(nodeParams[selectedId] || {}, null, 2));
  }, [nodeParams, selectedId]);

  const onNodesChange = useCallback(
    (changes: NodeChange<FlowNode>[]) =>
      setNodes((oldNodes) => applyNodeChanges(changes, oldNodes)),
    [],
  );

  const onEdgesChange = useCallback(
    (changes: EdgeChange<FlowEdge>[]) =>
      setEdges((oldEdges) => applyEdgeChanges(changes, oldEdges)),
    [],
  );

  const onConnect = useCallback(
    (connection: Connection) =>
      setEdges((oldEdges) =>
        addEdge(
          {
            ...connection,
            id: uniqueEdgeId(
              connection.source || "node",
              connection.target || "node",
            ),
            animated: true,
          },
          oldEdges,
        ),
      ),
    [],
  );

  const addNodeAt = (
    type: TransformNodeType,
    position: { x: number; y: number },
  ) => {
    const id = `${type}-${Date.now()}`;
    const node: FlowNode = {
      id,
      type: "cleanNode",
      position,
      data: {
        label: nodeTitle[type],
        nodeType: type,
      },
    };
    setNodes((oldNodes) => oldNodes.concat(node));
    setNodeParams((oldParams) => ({ ...oldParams, [id]: defaultParams[type] }));
    setSelectedId(id);
  };

  const onDragStart = (
    event: React.DragEvent<HTMLDivElement>,
    nodeType: TransformNodeType,
  ) => {
    event.dataTransfer.setData("application/autowds-clean-node", nodeType);
    event.dataTransfer.effectAllowed = "move";
  };

  const onDrop = (event: React.DragEvent<HTMLDivElement>) => {
    event.preventDefault();
    const type = event.dataTransfer.getData(
      "application/autowds-clean-node",
    ) as CleanNodeType;
    if (!type || !isTransformNodeType(type)) return;
    addNodeAt(
      type,
      screenToFlowPosition({ x: event.clientX, y: event.clientY }),
    );
  };

  const onDragOver = (event: React.DragEvent<HTMLDivElement>) => {
    event.preventDefault();
    event.dataTransfer.dropEffect = "move";
  };

  const applyParams = () => {
    const params = parseJsonObject(paramText);
    if (!params) {
      message.error("参数必须是 JSON 对象");
      return;
    }
    setNodeParams((oldParams) => ({ ...oldParams, [selectedId]: params }));
    setParamModalOpen(false);
    message.success("节点参数已应用");
  };

  const removeSelected = () => {
    if (
      !selectedNode ||
      selectedNodeType === "source" ||
      selectedNodeType === "sink"
    )
      return;
    setNodes((oldNodes) => oldNodes.filter((node) => node.id !== selectedId));
    setEdges((oldEdges) =>
      oldEdges.filter(
        (edge) => edge.source !== selectedId && edge.target !== selectedId,
      ),
    );
    setNodeParams((oldParams) => {
      const next = { ...oldParams };
      delete next[selectedId];
      return next;
    });
    setSelectedId("source");
    setParamModalOpen(false);
  };

  const validate = async () => {
    if (!storeId) return;
    const resp = await validateCleanPipeline(storeId, pipeline);
    if (resp.valid) {
      message.success("清洗流程校验通过");
    } else {
      message.error(resp.issues[0]?.message || "清洗流程校验失败");
    }
  };

  const runPreview = async () => {
    if (!storeId) return;
    setLoading(true);
    try {
      const resp = await previewCleanPipeline(storeId, pipeline, samples, 100);
      setPreview(resp);
      if (!resp.valid) {
        message.error(resp.issues[0]?.message || "清洗流程校验失败");
      }
    } finally {
      setLoading(false);
    }
  };

  const runNodePreview = useCallback(
    async (nodeId: string) => {
      if (!storeId) return;
      setSelectedId(nodeId);
      setLoading(true);
      try {
        const resp = await previewCleanPipeline(
          storeId,
          buildPreviewPipeline(pipeline, nodeId),
          samples,
          100,
        );
        setPreview(resp);
        if (!resp.valid) {
          message.error(resp.issues[0]?.message || "节点预览失败");
        }
      } finally {
        setLoading(false);
      }
    },
    [pipeline, samples, storeId],
  );

  const openSettings = useCallback((nodeId: string) => {
    openNodeSettings(nodeId, setSelectedId, setParamModalOpen);
  }, []);

  const renderNodes = useMemo<FlowNode[]>(
    () =>
      nodes.map((node) => ({
        ...node,
        data: {
          ...node.data,
          params: nodeParams[node.id] || {},
          onPreview: runNodePreview,
          onSettings: openSettings,
        },
      })),
    [nodeParams, nodes, openSettings, runNodePreview],
  );

  const save = async () => {
    if (!storeId) return;
    setSaving(true);
    try {
      await saveCleanPipeline(
        storeId,
        pipelineName || "未命名清洗流程",
        pipeline,
      );
      message.success("清洗流程已保存");
    } finally {
      setSaving(false);
    }
  };

  const exportData = async () => {
    if (!storeId) return;
    const resp = await exportCleanPipeline(storeId, pipeline, samples, format);
    const blob = new Blob([resp.content], { type: resp.mimeType });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = resp.filename;
    link.click();
    URL.revokeObjectURL(url);
  };

  const tableColumns = (preview?.schema || []).map((field) => ({
    title: field,
    dataIndex: field,
    ellipsis: true,
    render: (value: JsonValue) =>
      typeof value === "object" ? JSON.stringify(value) : String(value ?? ""),
  }));
  const columnProfiles = useMemo(
    () =>
      preview?.valid ? buildColumnProfiles(preview.output, preview.schema) : [],
    [preview],
  );

  const applyValueRevision = (field: string, from: JsonValue, to: string) => {
    const trimmedTo = to.trim();
    if (!trimmedTo) {
      message.warning("请输入修订后的值");
      return;
    }
    const id = `replace-${Date.now()}`;
    const sink = nodes.find((node) => node.data.nodeType === "sink");
    const sinkId = sink?.id || "sink";
    const incoming = edges.filter((edge) => edge.target === sinkId);
    const sources = incoming.length
      ? incoming.map((edge) => edge.source)
      : ["source"];
    const sourceNode = nodes.find((node) => node.id === sources[0]);
    const position = {
      x: (sourceNode?.position.x || 320) + 260,
      y: sourceNode?.position.y || 160,
    };

    setNodes((oldNodes) =>
      oldNodes.concat({
        id,
        type: "cleanNode",
        position,
        data: {
          label: `修订 ${field}`,
          nodeType: "replace",
        },
      }),
    );
    setNodeParams((oldParams) => ({
      ...oldParams,
      [id]: {
        field,
        from: valueBrief(from),
        to: trimmedTo,
      },
    }));
    setEdges((oldEdges) =>
      oldEdges
        .filter((edge) => edge.target !== sinkId)
        .concat(
          sources.map((source) => ({
            id: uniqueEdgeId(source, id),
            source,
            target: id,
            animated: true,
          })),
          {
            id: uniqueEdgeId(id, sinkId),
            source: id,
            target: sinkId,
            animated: true,
          },
        ),
    );
    setSelectedId(id);
    setValueDrafts((oldDrafts) => {
      const next = { ...oldDrafts };
      delete next[`${field}:${valueProfileKey(from)}`];
      return next;
    });
    message.success("已添加值修订节点");
  };

  const renderRowsPreview = () => (
    <Table
      size="small"
      rowKey={(_, index) => String(index)}
      dataSource={(preview?.output || []) as JsonObject[]}
      columns={tableColumns}
      pagination={false}
      scroll={{ x: "max-content", y: 220 }}
    />
  );

  const renderColumnsPreview = () => (
    <div style={{ display: "flex", flexWrap: "nowrap", overflowX: "auto" }}>
      {columnProfiles.map((profile) => (
        <div key={profile.field} style={{ flex: "0 0 220px" }}>
          <Card
            size="small"
            title={
              <Space direction="vertical" size={0}>
                <Typography.Text strong>{profile.field}</Typography.Text>
                <Typography.Text type="secondary" style={{ fontSize: 12 }}>
                  {profile.values.length} 个唯一值
                </Typography.Text>
              </Space>
            }
            style={{ borderRadius: 0 }}
            bodyStyle={{ maxHeight: 220, overflow: "auto", padding: 8 }}
          >
            <Space direction="vertical" style={{ width: "100%" }} size={8}>
              {profile.values.map((item) => {
                const draftKey = `${profile.field}:${valueProfileKey(item.value)}`;
                return (
                  <div
                    key={draftKey}
                    style={{
                      borderBottom: "1px solid #f0f0f0",
                      paddingBottom: 8,
                    }}
                  >
                    <Space
                      style={{ width: "100%", justifyContent: "space-between" }}
                    >
                      <Typography.Text ellipsis style={{ maxWidth: 130 }}>
                        {item.label}
                      </Typography.Text>
                      <Tag>{item.count}</Tag>
                    </Space>
                    <Space.Compact style={{ width: "100%", marginTop: 6 }}>
                      <Input
                        size="small"
                        placeholder="修订为"
                        value={valueDrafts[draftKey]}
                        onChange={(event) =>
                          setValueDrafts((oldDrafts) => ({
                            ...oldDrafts,
                            [draftKey]: event.target.value,
                          }))
                        }
                      />
                      <Button
                        size="small"
                        onClick={() =>
                          applyValueRevision(
                            profile.field,
                            item.value,
                            valueDrafts[draftKey] || "",
                          )
                        }
                      >
                        修订
                      </Button>
                    </Space.Compact>
                  </div>
                );
              })}
            </Space>
          </Card>
        </div>
      ))}
    </div>
  );

  const renderValidPreview = () => {
    if (previewMode === "rows") return renderRowsPreview();
    if (previewMode === "columns") return renderColumnsPreview();
    return (
      <Space direction="vertical" style={{ width: "100%" }} size={12}>
        {renderColumnsPreview()}
        {renderRowsPreview()}
      </Space>
    );
  };

  return (
    <>
      <Row gutter={16} style={{ height: "100%" }}>
        <Col span={24} style={{ height: "100%" }}>
          <Card
            title={
              <Space>
                <BranchesOutlined />
                <Input
                  placeholder="请输入清洗流程名称"
                  value={pipelineName}
                  onChange={(e) => setPipelineName(e.target.value)}
                  style={{ width: 260 }}
                />
              </Space>
            }
            extra={
              <Space>
                <Button onClick={validate}>校验</Button>
                <Button
                  icon={<PlayCircleOutlined />}
                  loading={loading}
                  onClick={runPreview}
                >
                  预览
                </Button>
                <Button icon={<SaveOutlined />} loading={saving} onClick={save}>
                  保存
                </Button>
                <Select
                  value={format}
                  onChange={setFormat}
                  style={{ width: 110 }}
                >
                  <Select.Option value="json">JSON</Select.Option>
                  <Select.Option value="ndjson">NDJSON</Select.Option>
                  <Select.Option value="csv">CSV</Select.Option>
                </Select>
                <Button icon={<DownloadOutlined />} onClick={exportData}>
                  导出
                </Button>
              </Space>
            }
            style={{ height: "100%" }}
            bodyStyle={{
              height: "calc(100% - 57px)",
              display: "flex",
              flexDirection: "column",
              gap: 16,
            }}
          >
            <Row gutter={12} style={{ height: "100%" }}>
              <Col span={5} style={{ height: "100%" }}>
                <Card
                  size="small"
                  title={
                    <Space direction="vertical" size={0}>
                      <span>算子面板</span>
                      <Typography.Text
                        type="secondary"
                        style={{ fontSize: 12 }}
                      >
                        拖入算子，移动节点并连线
                      </Typography.Text>
                    </Space>
                  }
                  style={{
                    height: "100%",
                    display: "flex",
                    flexDirection: "column",
                  }}
                  bodyStyle={{ overflow: "auto" }}
                >
                  <Space direction="vertical" style={{ width: "100%" }}>
                    {nodeOptions.map((option) => (
                      <Card
                        key={option.value}
                        size="small"
                        hoverable
                        draggable
                        onDragStart={(event) =>
                          onDragStart(event, option.value)
                        }
                        onDoubleClick={() =>
                          addNodeAt(option.value, { x: 240, y: 120 })
                        }
                      >
                        <Tag color="green">{option.label}</Tag>
                        <Typography.Paragraph
                          type="secondary"
                          style={{ margin: "8px 0 0" }}
                        >
                          拖到画布添加
                        </Typography.Paragraph>
                      </Card>
                    ))}
                  </Space>
                </Card>
              </Col>
              <Col span={19} style={{ height: "100%" }}>
                <div
                  style={{
                    height: "100%",
                    border: "1px solid #f0f0f0",
                    borderRadius: 8,
                  }}
                >
                  <ReactFlow
                    nodes={renderNodes}
                    edges={edges}
                    nodeTypes={nodeTypes}
                    proOptions={{ hideAttribution: true }}
                    onNodesChange={onNodesChange}
                    onEdgesChange={onEdgesChange}
                    onConnect={onConnect}
                    onDrop={onDrop}
                    onDragOver={onDragOver}
                    onNodeClick={(_, node) => setSelectedId(node.id)}
                    fitView
                  >
                    <Background />
                    <Controls />
                    <MiniMap pannable zoomable />
                  </ReactFlow>
                </div>
              </Col>
            </Row>
          </Card>
        </Col>
      </Row>
      {preview ? (
        <Card
          title={
            <Space>
              <span>清洗预览</span>
              <Segmented
                size="small"
                value={previewMode}
                onChange={(value) => setPreviewMode(value as PreviewMode)}
                options={[
                  { label: "按行", value: "rows" },
                  { label: "按列", value: "columns" },
                  { label: "行列", value: "both" },
                ]}
              />
            </Space>
          }
          extra={
            <Button
              type="text"
              size="small"
              icon={<CloseOutlined />}
              onClick={() => setPreview(undefined)}
            />
          }
          style={{
            position: "absolute",
            left: 0,
            right: 0,
            bottom: 0,
            zIndex: 10,
            maxHeight: 420,
            boxShadow: "0 8px 24px rgba(0, 0, 0, 0.12)",
          }}
          bodyStyle={{ maxHeight: "40vh", overflow: "auto", padding: 0 }}
        >
          {preview.valid ? (
            renderValidPreview()
          ) : (
            <List
              dataSource={preview.issues}
              renderItem={(item) => <List.Item>{item.message}</List.Item>}
            />
          )}
        </Card>
      ) : null}
      <Modal
        title="节点参数"
        open={paramModalOpen}
        onCancel={() => setParamModalOpen(false)}
        width={720}
        destroyOnClose
        footer={
          selectedNode ? (
            <Space>
              <Button onClick={() => setParamModalOpen(false)}>取消</Button>
              <Button
                danger
                disabled={
                  selectedNodeType === "source" || selectedNodeType === "sink"
                }
                onClick={removeSelected}
              >
                删除节点
              </Button>
              <Button type="primary" onClick={applyParams}>
                应用参数
              </Button>
            </Space>
          ) : null
        }
      >
        {selectedNode ? (
          <Space direction="vertical" style={{ width: "100%" }}>
            <Alert
              type="info"
              showIcon
              message={selectedNodeType ? nodeTitle[selectedNodeType] : "节点"}
              description="参数使用 JSON 对象，字段名需与后端算子 DTO 对齐。"
            />
            <TextArea
              value={paramText}
              onChange={(e) => setParamText(e.target.value)}
              autoSize={{ minRows: 14, maxRows: 24 }}
              style={{ fontFamily: "monospace" }}
            />
          </Space>
        ) : (
          <Empty />
        )}
      </Modal>
    </>
  );
};

const CleanPipeline = () => (
  <ReactFlowProvider>
    <CleanPipelineWorkbench />
  </ReactFlowProvider>
);

export default CleanPipeline;
