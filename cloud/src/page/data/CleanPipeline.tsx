import {
  BranchesOutlined,
  DownloadOutlined,
  PlayCircleOutlined,
  SaveOutlined,
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
  MiniMap,
  Node,
  NodeChange,
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
    type: "default",
    position: node.position || { x: 80 + index * 220, y: 160 },
    data: {
      label: node.label || nodeTitle[node.type],
      nodeType: node.type,
    },
    style: {
      borderColor:
        node.type === "source" || node.type === "sink" ? "#1677ff" : "#52c41a",
      borderWidth: 2,
      borderRadius: 8,
      minWidth: 150,
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
      type: "default",
      position,
      data: {
        label: nodeTitle[type],
        nodeType: type,
      },
      style: {
        borderColor: "#52c41a",
        borderWidth: 2,
        borderRadius: 8,
        minWidth: 150,
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
                    nodes={nodes}
                    edges={edges}
                    onNodesChange={onNodesChange}
                    onEdgesChange={onEdgesChange}
                    onConnect={onConnect}
                    onDrop={onDrop}
                    onDragOver={onDragOver}
                    onNodeClick={(_, node) => {
                      setSelectedId(node.id);
                      setParamModalOpen(true);
                    }}
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
          title="清洗预览"
          style={{
            position: "fixed",
            left: 280,
            right: 24,
            bottom: 16,
            zIndex: 10,
            maxHeight: 320,
            boxShadow: "0 8px 24px rgba(0, 0, 0, 0.12)",
          }}
          bodyStyle={{ maxHeight: 260, overflow: "auto" }}
        >
          {preview.valid ? (
            <Table
              size="small"
              rowKey={(_, index) => String(index)}
              dataSource={preview.output as JsonObject[]}
              columns={tableColumns}
              pagination={{ pageSize: 10 }}
            />
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
