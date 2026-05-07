use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CleanPipeline {
    pub name: Option<String>,
    pub nodes: Vec<CleanNode>,
    pub edges: Vec<CleanEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CleanNode {
    pub id: String,
    #[serde(rename = "type")]
    pub node_type: CleanNodeType,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub position: Option<CleanNodePosition>,
    #[serde(default)]
    pub params: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CleanNodePosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CleanEdge {
    pub id: String,
    pub source: String,
    pub target: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum CleanNodeType {
    Source,
    SelectRename,
    Trim,
    Replace,
    TypeCast,
    Filter,
    Dedupe,
    DerivedField,
    Sink,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SaveCleanPipelineReq {
    #[validate(length(min = 1, max = 80, message = "名称长度需在 1 到 80 个字符之间"))]
    pub name: String,
    pub pipeline: CleanPipeline,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CleanPipelineReq {
    pub pipeline: CleanPipeline,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CleanPreviewReq {
    pub pipeline: CleanPipeline,
    #[serde(default)]
    pub records: Vec<Value>,
    #[serde(default)]
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CleanExportReq {
    pub pipeline: CleanPipeline,
    #[serde(default)]
    pub records: Vec<Value>,
    #[serde(default = "default_export_format")]
    pub format: CleanExportFormat,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CleanExportFormat {
    Json,
    Ndjson,
    Csv,
}

fn default_export_format() -> CleanExportFormat {
    CleanExportFormat::Json
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CleanPipelineSummary {
    pub id: i64,
    pub store_id: String,
    pub name: String,
    pub pipeline: CleanPipeline,
    pub created_at: String,
    pub modified_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CleanValidationResp {
    pub valid: bool,
    pub issues: Vec<CleanValidationIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CleanValidationIssue {
    pub node_id: Option<String>,
    pub edge_id: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CleanPreviewResp {
    pub valid: bool,
    pub issues: Vec<CleanValidationIssue>,
    pub input: Vec<Value>,
    pub output: Vec<Value>,
    pub schema: Vec<String>,
    pub input_count: usize,
    pub output_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CleanExportResp {
    pub filename: String,
    pub mime_type: String,
    pub content: String,
    pub row_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FieldRename {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SelectRenameParams {
    pub fields: Vec<FieldRename>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FieldListParams {
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceParams {
    pub field: String,
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TypeCastParams {
    pub field: String,
    pub target: TypeCastTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum TypeCastTarget {
    String,
    Number,
    Boolean,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FilterParams {
    pub field: String,
    pub op: FilterOp,
    #[serde(default)]
    pub value: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum FilterOp {
    Eq,
    Ne,
    Contains,
    NotContains,
    IsEmpty,
    IsNotEmpty,
    Gt,
    Gte,
    Lt,
    Lte,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DerivedFieldParams {
    pub field: String,
    pub template: String,
}
