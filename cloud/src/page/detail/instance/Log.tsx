import { getAuthUser } from "api/user";
import { BASE_URL } from "utils/ajax";
import { useEffect, useRef, useState } from "react";
import { useParams } from "react-router-dom";

interface LogEntry {
  timestamp: number;
  taskId: string;
  instanceId: string;
  nodeId: string;
  level: string;
  content: string;
}

const LevelColor: Record<string, string> = {
  debug: "#8c8c8c",
  info: "#1890ff",
  warn: "#faad14",
  error: "#f5222d",
};

const formatTime = (ts: number) => {
  const d = new Date(ts);
  return d.toLocaleTimeString("zh-CN", {
    hour12: false,
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  });
};

const Log = () => {
  const { taskId, instanceId } = useParams();
  const [entries, setEntries] = useState<LogEntry[]>([]);
  const [connected, setConnected] = useState(false);
  const bottomRef = useRef<HTMLDivElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    let es: EventSource | null = null;

    (async () => {
      const user = await getAuthUser();
      const token = user?.token;
      if (!token || !taskId || !instanceId) return;

      const url = `${BASE_URL}/task/${taskId}/instance/${instanceId}/logs?token=${encodeURIComponent(token)}`;
      es = new EventSource(url);

      es.onopen = () => setConnected(true);

      es.onmessage = (event) => {
        try {
          const entry: LogEntry = JSON.parse(event.data);
          setEntries((prev) => [...prev, entry]);
        } catch {
          // 忽略非 JSON 消息
        }
      };

      /** 归档回放：后端发完 NDJSON 后发 `event: logs_complete`，须主动 close，否则 EventSource 会在连接关闭后自动重连。 */
      es.addEventListener("logs_complete", () => {
        setConnected(false);
        es?.close();
      });

      es.onerror = () => {
        setConnected(false);
        // 默认会重连（含代理 idle 断线、归档流结束等）；主动关闭，避免一直打请求。
        es?.close();
      };
    })();

    return () => {
      if (es) {
        es.close();
      }
    };
  }, [taskId, instanceId]);

  // 自动滚动到底部
  useEffect(() => {
    if (bottomRef.current && containerRef.current) {
      containerRef.current.scrollTop = containerRef.current.scrollHeight;
    }
  }, [entries]);

  return (
    <div style={{ display: "flex", flexDirection: "column", height: "100%" }}>
      <div
        style={{
          padding: "8px 12px",
          borderBottom: "1px solid #f0f0f0",
          fontSize: 12,
          color: connected ? "#52c41a" : "#ff4d4f",
        }}
      >
        {connected ? "● 实时连接中" : "● 连接已断开"} · 共 {entries.length} 条日志
      </div>
      <div
        ref={containerRef}
        style={{
          flex: 1,
          overflow: "auto",
          padding: 12,
          fontFamily: '"SF Mono", Monaco, "Cascadia Code", monospace',
          fontSize: 13,
          lineHeight: "22px",
          background: "#fafafa",
        }}
      >
        {entries.length === 0 && (
          <div style={{ color: "#bfbfbf", textAlign: "center", marginTop: 40 }}>
            暂无日志，等待任务执行...
          </div>
        )}
        {entries.map((e, i) => (
          <div key={i} style={{ whiteSpace: "pre-wrap", wordBreak: "break-all" }}>
            <span style={{ color: "#8c8c8c" }}>[{formatTime(e.timestamp)}]</span>{" "}
            <span style={{ color: LevelColor[e.level] || "#333", fontWeight: 600 }}>
              [{e.level.toUpperCase()}]
            </span>{" "}
            <span style={{ color: "#595959" }}>[{e.nodeId}]</span>{" "}
            <span style={{ color: "#262626" }}>{e.content}</span>
          </div>
        ))}
        <div ref={bottomRef} />
      </div>
    </div>
  );
};

export default Log;
