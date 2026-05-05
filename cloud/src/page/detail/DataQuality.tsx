import { Button, Card, Form, Space, Spin, Typography, message } from "antd";
import { getTask, updateTask } from "api/task";
import { useCallback, useEffect, useMemo, useState } from "react";
import { useParams } from "react-router-dom";
import { Field, Graph, Node, NodeType } from "types/NodeTypes";
import { DataQualityConfig, ScraperTaskData } from "types/Task";

type FieldPathOption = {
  /** 写入 `dedupeJsonPaths`，与实例侧 `payload` 上点路径一致：`{nodeId}.{fieldKey}` */
  path: string;
  label: string;
};

function fieldKey(f: Field): string {
  const n = f.name?.trim();
  if (n) return n;
  return f.id?.trim() || "field";
}

function nodeLabel(n: Node): string {
  return `${n.id}（${n.action}）`;
}

function collectDedupeFieldOptions(graph: Graph | null | undefined): FieldPathOption[] {
  if (!graph?.nodes?.length) return [];
  const out: FieldPathOption[] = [];
  for (const node of graph.nodes) {
    if (node.action !== NodeType.list && node.action !== NodeType.detail) continue;
    const cfg = node.config as { fields?: Field[] };
    const fields = cfg?.fields;
    if (!fields?.length) continue;
    for (const f of fields) {
      const key = fieldKey(f);
      const path = `${node.id}.${key}`;
      out.push({
        path,
        label: `${nodeLabel(node)} — ${f.name || f.id}（${path}）`,
      });
    }
  }
  return out;
}

function mergeTaskData(
  prev: ScraperTaskData | null | undefined,
  quality: DataQualityConfig
): ScraperTaskData {
  return {
    ...(prev ?? {}),
    dataQuality: quality,
  };
}

const DataQuality = () => {
  const { taskId } = useParams();
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [graph, setGraph] = useState<Graph | null>(null);
  const [taskData, setTaskData] = useState<ScraperTaskData | null>(null);
  const [rule, setRule] = useState<unknown>(null);

  const load = useCallback(async () => {
    if (!taskId) return;
    setLoading(true);
    try {
      const t = await getTask(taskId);
      setGraph((t.rule as Graph) ?? null);
      setTaskData(t.data ?? null);
      setRule(t.rule);
    } catch (e) {
      console.error(e);
      message.error("加载任务失败");
    } finally {
      setLoading(false);
    }
  }, [taskId]);

  useEffect(() => {
    void load();
  }, [load]);

  const pathOptions = useMemo(() => collectDedupeFieldOptions(graph), [graph]);

  const [selectedPaths, setSelectedPaths] = useState<string[]>([]);

  useEffect(() => {
    const dq = taskData?.dataQuality;
    setSelectedPaths(dq?.dedupeJsonPaths?.length ? dq.dedupeJsonPaths : []);
  }, [taskData]);

  const togglePath = (path: string, checked: boolean) => {
    setSelectedPaths((prev) => {
      if (checked) return prev.includes(path) ? prev : [...prev, path];
      return prev.filter((p) => p !== path);
    });
  };

  const handleSave = async () => {
    if (!taskId || rule == null) {
      message.warning("任务数据未就绪");
      return;
    }
    setSaving(true);
    try {
      const nextQuality: DataQualityConfig = {
        dedupeJsonPaths: selectedPaths,
      };
      const nextData = mergeTaskData(taskData ?? undefined, nextQuality);
      await updateTask(taskId, { data: nextData, rule });
      message.success("已保存");
      await load();
    } catch (e) {
      console.error(e);
      message.error("保存失败");
    } finally {
      setSaving(false);
    }
  };

  if (loading) {
    return (
      <div style={{ padding: 48, textAlign: "center" }}>
        <Spin />
      </div>
    );
  }

  return (
    <Space direction="vertical" size="large" style={{ width: "100%", maxWidth: 720 }}>
      {pathOptions.length === 0 ? (
        <Card>
          <Typography.Text type="secondary">
            暂无可选字段，请先在任务规则里为列表或详情配置字段。
          </Typography.Text>
        </Card>
      ) : (
        <Card title="去重依据（可多选）">
          <Form layout="vertical">
            <Form.Item>
              <Space direction="vertical" style={{ width: "100%" }}>
                {pathOptions.map((opt) => (
                  <label
                    key={opt.path}
                    style={{
                      display: "flex",
                      alignItems: "flex-start",
                      gap: 8,
                      cursor: "pointer",
                    }}
                  >
                    <input
                      type="checkbox"
                      checked={selectedPaths.includes(opt.path)}
                      onChange={(e) => togglePath(opt.path, e.target.checked)}
                    />
                    <span>{opt.label}</span>
                  </label>
                ))}
              </Space>
            </Form.Item>
            <Form.Item>
              <Button type="primary" loading={saving} onClick={() => void handleSave()}>
                保存
              </Button>
            </Form.Item>
          </Form>
        </Card>
      )}
    </Space>
  );
};

export default DataQuality;
