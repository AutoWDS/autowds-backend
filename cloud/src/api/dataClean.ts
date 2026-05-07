import ajax from "utils/ajax";

export type JsonPrimitive = string | number | boolean | null;
export type JsonValue = JsonPrimitive | JsonObject | JsonValue[];
export interface JsonObject {
  [key: string]: JsonValue;
}

export type CleanNodeType =
  | "source"
  | "selectRename"
  | "trim"
  | "replace"
  | "typeCast"
  | "filter"
  | "dedupe"
  | "derivedField"
  | "sink";

export interface CleanNode {
  id: string;
  type: CleanNodeType;
  label?: string;
  position?: {
    x: number;
    y: number;
  };
  params: JsonObject;
}

export interface CleanEdge {
  id: string;
  source: string;
  target: string;
}

export interface CleanPipeline {
  name?: string;
  nodes: CleanNode[];
  edges: CleanEdge[];
}

export interface CleanPipelineSummary {
  id: number;
  storeId: string;
  name: string;
  pipeline: CleanPipeline;
  createdAt: string;
  modifiedAt: string;
}

export interface CleanValidationIssue {
  nodeId?: string;
  edgeId?: string;
  message: string;
}

export interface CleanValidationResp {
  valid: boolean;
  issues: CleanValidationIssue[];
}

export interface CleanPreviewResp extends CleanValidationResp {
  input: JsonValue[];
  output: JsonValue[];
  schema: string[];
  inputCount: number;
  outputCount: number;
}

export type CleanExportFormat = "json" | "ndjson" | "csv";

export interface CleanExportResp {
  filename: string;
  mimeType: string;
  content: string;
  rowCount: number;
}

const clean = (storeId: string) => ajax("/store").path(storeId).path("clean");

export async function queryCleanPipelines(storeId: string) {
  return clean(storeId).path("pipelines").get() as Promise<CleanPipelineSummary[]>;
}

export async function saveCleanPipeline(
  storeId: string,
  name: string,
  pipeline: CleanPipeline
) {
  return clean(storeId).path("pipelines").payload({ name, pipeline }).post() as Promise<CleanPipelineSummary>;
}

export async function validateCleanPipeline(storeId: string, pipeline: CleanPipeline) {
  return clean(storeId).path("validate").payload({ pipeline }).post() as Promise<CleanValidationResp>;
}

export async function previewCleanPipeline(
  storeId: string,
  pipeline: CleanPipeline,
  records: JsonValue[],
  limit = 100
) {
  return clean(storeId)
    .path("preview")
    .payload({ pipeline, records, limit })
    .post() as Promise<CleanPreviewResp>;
}

export async function exportCleanPipeline(
  storeId: string,
  pipeline: CleanPipeline,
  records: JsonValue[],
  format: CleanExportFormat
) {
  return clean(storeId)
    .path("export")
    .payload({ pipeline, records, format })
    .post() as Promise<CleanExportResp>;
}
