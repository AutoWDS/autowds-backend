use crate::views::data_clean::{
    CleanEdge, CleanExportFormat, CleanExportResp, CleanNode, CleanNodeType, CleanPipeline,
    CleanPreviewResp, CleanValidationIssue, CleanValidationResp, DerivedFieldParams,
    FieldListParams, FilterOp, FilterParams, ReplaceParams, SelectRenameParams, TypeCastParams,
    TypeCastTarget,
};
use anyhow::{anyhow, Context, Result};
use duckdb::Connection;
use serde_json::{Map, Number, Value};
use std::collections::{HashMap, HashSet, VecDeque};
use tempfile::tempdir;

#[derive(Debug, Clone)]
pub struct RuleFieldProjection {
    pub output_name: String,
    pub node_id: String,
    pub field_id: String,
}

pub fn validate_pipeline(pipeline: &CleanPipeline) -> CleanValidationResp {
    let issues = collect_issues(pipeline);
    CleanValidationResp {
        valid: issues.is_empty(),
        issues,
    }
}

pub fn preview_pipeline(
    pipeline: &CleanPipeline,
    records: Vec<Value>,
    limit: Option<usize>,
    rule: Option<&Value>,
) -> Result<CleanPreviewResp> {
    let validation = validate_pipeline(pipeline);
    if !validation.valid {
        return Ok(CleanPreviewResp {
            valid: false,
            issues: validation.issues,
            input: records,
            output: vec![],
            schema: vec![],
            input_count: 0,
            output_count: 0,
        });
    }

    let input = flatten_records_by_rule(records, rule)?
        .into_iter()
        .take(limit.unwrap_or(100))
        .collect::<Vec<_>>();
    let output = execute_duckdb_pipeline(pipeline, input.clone(), None)?;
    let schema = infer_schema(&output);

    Ok(CleanPreviewResp {
        valid: true,
        issues: vec![],
        input_count: input.len(),
        output_count: output.len(),
        input,
        output,
        schema,
    })
}

pub fn export_pipeline(
    pipeline: &CleanPipeline,
    records: Vec<Value>,
    format: CleanExportFormat,
    store_id: &str,
    rule: Option<&Value>,
) -> Result<CleanExportResp> {
    let validation = validate_pipeline(pipeline);
    if !validation.valid {
        return Err(anyhow!("清洗流程校验失败"));
    }

    let output = execute_duckdb_pipeline(pipeline, flatten_records_by_rule(records, rule)?, None)?;
    let (extension, mime_type, content) = match format {
        CleanExportFormat::Json => (
            "json",
            "application/json",
            serde_json::to_string_pretty(&output).context("序列化 JSON 导出失败")?,
        ),
        CleanExportFormat::Ndjson => (
            "ndjson",
            "application/x-ndjson",
            output
                .iter()
                .map(serde_json::to_string)
                .collect::<std::result::Result<Vec<_>, _>>()
                .context("序列化 NDJSON 导出失败")?
                .join("\n"),
        ),
        CleanExportFormat::Csv => ("csv", "text/csv", to_csv(&output)),
    };

    Ok(CleanExportResp {
        filename: format!("data-clean-{store_id}.{extension}"),
        mime_type: mime_type.to_string(),
        content,
        row_count: output.len(),
    })
}

pub fn rule_field_projections(rule: Option<&Value>) -> Vec<RuleFieldProjection> {
    let Some(Value::Object(rule)) = rule else {
        return vec![];
    };
    let Some(Value::Array(nodes)) = rule.get("nodes") else {
        return vec![];
    };

    let mut raw = Vec::new();
    for node in nodes {
        let Value::Object(node) = node else {
            continue;
        };
        let Some(node_id) = node.get("id").and_then(Value::as_str) else {
            continue;
        };
        let Some(fields) = node
            .get("config")
            .and_then(|config| config.get("fields"))
            .and_then(Value::as_array)
        else {
            continue;
        };
        for field in fields {
            let Value::Object(field) = field else {
                continue;
            };
            let Some(field_id) = field.get("id").and_then(Value::as_str) else {
                continue;
            };
            let display_name = field
                .get("name")
                .and_then(Value::as_str)
                .filter(|name| !name.trim().is_empty())
                .unwrap_or(field_id)
                .trim()
                .to_string();
            raw.push((node_id.to_string(), field_id.to_string(), display_name));
        }
    }

    let mut name_counts = HashMap::new();
    for (_, _, name) in &raw {
        *name_counts.entry(name.clone()).or_insert(0usize) += 1;
    }

    raw.into_iter()
        .map(|(node_id, field_id, name)| {
            let output_name = if name_counts.get(&name).copied().unwrap_or_default() > 1 {
                format!("{node_id}.{name}")
            } else {
                name
            };
            RuleFieldProjection {
                output_name,
                node_id,
                field_id,
            }
        })
        .collect()
}

pub fn flatten_records_by_rule(records: Vec<Value>, rule: Option<&Value>) -> Result<Vec<Value>> {
    let projections = rule_field_projections(rule);
    if projections.is_empty() {
        return Ok(records.into_iter().map(flatten_nested_record).collect());
    }

    Ok(records
        .into_iter()
        .map(|record| {
            let mut output = Map::new();
            for projection in &projections {
                let value = record
                    .get(&projection.node_id)
                    .and_then(|node| node.get(&projection.field_id))
                    .cloned()
                    .unwrap_or(Value::Null);
                output.insert(projection.output_name.clone(), value);
            }
            Value::Object(output)
        })
        .collect())
}

fn flatten_nested_record(record: Value) -> Value {
    let Value::Object(map) = record else {
        return record;
    };
    let has_nested_object = map.values().any(|value| matches!(value, Value::Object(_)));
    if !has_nested_object {
        return Value::Object(map);
    }

    let mut output = Map::new();
    for (node_id, value) in map {
        match value {
            Value::Object(fields) => {
                for (field_id, field_value) in fields {
                    output.insert(format!("{node_id}.{field_id}"), field_value);
                }
            }
            other => {
                output.insert(node_id, other);
            }
        }
    }
    Value::Object(output)
}

fn collect_issues(pipeline: &CleanPipeline) -> Vec<CleanValidationIssue> {
    let mut issues = Vec::new();
    if pipeline.nodes.is_empty() {
        issues.push(issue(None, None, "清洗流程至少需要一个节点"));
        return issues;
    }

    let mut nodes = HashMap::new();
    for node in &pipeline.nodes {
        if node.id.trim().is_empty() {
            issues.push(issue(None, None, "节点 ID 不能为空"));
            continue;
        }
        if nodes.insert(node.id.as_str(), node).is_some() {
            issues.push(issue(Some(&node.id), None, "节点 ID 重复"));
        }
        if let Err(e) = validate_node_params(node) {
            issues.push(issue(Some(&node.id), None, e.to_string()));
        }
    }

    let mut source_count = 0;
    let mut sink_count = 0;
    let mut indegree: HashMap<&str, usize> = HashMap::new();
    let mut outdegree: HashMap<&str, usize> = HashMap::new();

    for node in &pipeline.nodes {
        indegree.insert(&node.id, 0);
        outdegree.insert(&node.id, 0);
        match node.node_type {
            CleanNodeType::Source => source_count += 1,
            CleanNodeType::Sink => sink_count += 1,
            _ => {}
        }
    }

    for edge in &pipeline.edges {
        if !nodes.contains_key(edge.source.as_str()) {
            issues.push(issue(None, Some(&edge.id), "连线 source 节点不存在"));
        }
        if !nodes.contains_key(edge.target.as_str()) {
            issues.push(issue(None, Some(&edge.id), "连线 target 节点不存在"));
        }
        *outdegree.entry(&edge.source).or_default() += 1;
        *indegree.entry(&edge.target).or_default() += 1;
    }

    if source_count != 1 {
        issues.push(issue(None, None, "清洗流程必须且只能有一个 Source 节点"));
    }
    if sink_count == 0 {
        issues.push(issue(None, None, "清洗流程至少需要一个 Sink 节点"));
    }

    for node in &pipeline.nodes {
        let in_count = *indegree.get(node.id.as_str()).unwrap_or(&0);
        let out_count = *outdegree.get(node.id.as_str()).unwrap_or(&0);
        match node.node_type {
            CleanNodeType::Source => {
                if in_count != 0 || out_count == 0 {
                    issues.push(issue(
                        Some(&node.id),
                        None,
                        "Source 节点必须无输入且至少有一个输出",
                    ));
                }
            }
            CleanNodeType::Sink => {
                if in_count != 1 || out_count != 0 {
                    issues.push(issue(
                        Some(&node.id),
                        None,
                        "Sink 节点必须有一个输入且无输出",
                    ));
                }
            }
            _ => {
                if in_count != 1 || out_count == 0 {
                    issues.push(issue(
                        Some(&node.id),
                        None,
                        "转换节点必须有一个输入且至少有一个输出",
                    ));
                }
            }
        }
    }

    if topo_sort(pipeline).is_err() {
        issues.push(issue(None, None, "清洗流程不能包含环"));
    }

    issues
}

fn validate_node_params(node: &CleanNode) -> Result<()> {
    match node.node_type {
        CleanNodeType::Source | CleanNodeType::Sink => Ok(()),
        CleanNodeType::SelectRename => {
            let params: SelectRenameParams = parse_params(node)?;
            if params.fields.is_empty() {
                return Err(anyhow!("Select/Rename 至少需要一个字段"));
            }
            Ok(())
        }
        CleanNodeType::Trim | CleanNodeType::Dedupe => {
            let params: FieldListParams = parse_params(node)?;
            if params.fields.is_empty() {
                return Err(anyhow!("字段列表不能为空"));
            }
            Ok(())
        }
        CleanNodeType::Replace => {
            let params: ReplaceParams = parse_params(node)?;
            if params.field.is_empty() {
                return Err(anyhow!("替换字段不能为空"));
            }
            Ok(())
        }
        CleanNodeType::TypeCast => {
            let params: TypeCastParams = parse_params(node)?;
            if params.field.is_empty() {
                return Err(anyhow!("类型转换字段不能为空"));
            }
            Ok(())
        }
        CleanNodeType::Filter => {
            let params: FilterParams = parse_params(node)?;
            if params.field.is_empty() {
                return Err(anyhow!("过滤字段不能为空"));
            }
            Ok(())
        }
        CleanNodeType::DerivedField => {
            let params: DerivedFieldParams = parse_params(node)?;
            if params.field.is_empty() {
                return Err(anyhow!("计算字段名不能为空"));
            }
            Ok(())
        }
    }
}

fn execute_linear_pipeline(
    pipeline: &CleanPipeline,
    mut records: Vec<Value>,
) -> Result<Vec<Value>> {
    for node in topo_sort(pipeline)? {
        match node.node_type {
            CleanNodeType::Source | CleanNodeType::Sink => {}
            CleanNodeType::SelectRename => {
                records = apply_select_rename(records, parse_params(node)?);
            }
            CleanNodeType::Trim => {
                records = apply_trim(records, parse_params(node)?);
            }
            CleanNodeType::Replace => {
                records = apply_replace(records, parse_params(node)?);
            }
            CleanNodeType::TypeCast => {
                records = apply_type_cast(records, parse_params(node)?);
            }
            CleanNodeType::Filter => {
                records = apply_filter(records, parse_params(node)?);
            }
            CleanNodeType::Dedupe => {
                records = apply_dedupe(records, parse_params(node)?);
            }
            CleanNodeType::DerivedField => {
                records = apply_derived_field(records, parse_params(node)?);
            }
        }
    }
    Ok(records)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DuckFieldType {
    String,
    Number,
    Boolean,
}

struct DuckExecutionState {
    table: String,
    fields: Vec<String>,
    field_types: HashMap<String, DuckFieldType>,
}

fn execute_duckdb_pipeline(
    pipeline: &CleanPipeline,
    records: Vec<Value>,
    limit: Option<usize>,
) -> Result<Vec<Value>> {
    if records.is_empty() {
        return Ok(vec![]);
    }

    let conn = Connection::open_in_memory().context("创建 DuckDB 内存连接失败")?;
    let temp_dir = tempdir().context("创建 Parquet 临时目录失败")?;
    let parquet_path = temp_dir.path().join("clean-input.parquet");
    let parquet_path = parquet_path
        .to_str()
        .ok_or_else(|| anyhow!("Parquet 临时路径不是有效 UTF-8"))?;

    let fields = infer_schema(&records);
    if fields.is_empty() {
        return Ok(records);
    }
    create_input_table(&conn, &records, &fields)?;
    conn.execute_batch(&format!(
        "COPY input_records TO {} (FORMAT PARQUET); \
         CREATE TABLE clean_step_0 AS SELECT * FROM read_parquet({});",
        sql_literal(parquet_path),
        sql_literal(parquet_path),
    ))
    .context("写入并读取 Parquet 临时数据失败")?;

    let mut state = DuckExecutionState {
        table: "clean_step_0".to_string(),
        fields: fields.clone(),
        field_types: fields
            .into_iter()
            .map(|field| (field, DuckFieldType::String))
            .collect(),
    };

    for (step, node) in topo_sort(pipeline)?.into_iter().enumerate() {
        if matches!(node.node_type, CleanNodeType::Source | CleanNodeType::Sink) {
            continue;
        }
        let next_table = format!("clean_step_{}", step + 1);
        apply_duckdb_node(&conn, &mut state, node, &next_table)?;
        state.table = next_table;
    }

    query_duckdb_rows(&conn, &state, limit)
}

fn create_input_table(conn: &Connection, records: &[Value], fields: &[String]) -> Result<()> {
    let columns = fields
        .iter()
        .map(|field| format!("{} VARCHAR", quote_ident(field)))
        .collect::<Vec<_>>()
        .join(", ");
    conn.execute_batch(&format!("CREATE TABLE input_records ({columns});"))
        .context("创建 DuckDB 输入表失败")?;

    for record in records {
        let Value::Object(map) = record else {
            continue;
        };
        let values = fields
            .iter()
            .map(|field| {
                map.get(field)
                    .filter(|value| !value.is_null())
                    .map(|value| sql_literal(&value_to_string(value)))
                    .unwrap_or_else(|| "NULL".to_string())
            })
            .collect::<Vec<_>>()
            .join(", ");
        conn.execute_batch(&format!("INSERT INTO input_records VALUES ({values});"))
            .context("写入 DuckDB 输入数据失败")?;
    }
    Ok(())
}

fn apply_duckdb_node(
    conn: &Connection,
    state: &mut DuckExecutionState,
    node: &CleanNode,
    next_table: &str,
) -> Result<()> {
    match node.node_type {
        CleanNodeType::Source | CleanNodeType::Sink => {}
        CleanNodeType::SelectRename => {
            let params: SelectRenameParams = parse_params(node)?;
            let select = params
                .fields
                .iter()
                .map(|field| format!("{} AS {}", quote_ident(&field.from), quote_ident(&field.to)))
                .collect::<Vec<_>>()
                .join(", ");
            conn.execute_batch(&format!(
                "CREATE TABLE {} AS SELECT {select} FROM {};",
                quote_ident(next_table),
                quote_ident(&state.table),
            ))
            .context("执行 Select/Rename 清洗失败")?;
            state.field_types = params
                .fields
                .iter()
                .map(|field| {
                    (
                        field.to.clone(),
                        state
                            .field_types
                            .get(&field.from)
                            .copied()
                            .unwrap_or(DuckFieldType::String),
                    )
                })
                .collect();
            state.fields = params.fields.into_iter().map(|field| field.to).collect();
        }
        CleanNodeType::Trim => {
            let params: FieldListParams = parse_params(node)?;
            let fields = params.fields.into_iter().collect::<HashSet<_>>();
            let select = state
                .fields
                .iter()
                .map(|field| {
                    if fields.contains(field) {
                        format!(
                            "TRIM(CAST({} AS VARCHAR)) AS {}",
                            quote_ident(field),
                            quote_ident(field)
                        )
                    } else {
                        quote_ident(field)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            create_duckdb_select(conn, next_table, &select, &state.table, None)
                .context("执行 Trim 清洗失败")?;
        }
        CleanNodeType::Replace => {
            let params: ReplaceParams = parse_params(node)?;
            let select = state
                .fields
                .iter()
                .map(|field| {
                    if *field == params.field {
                        format!(
                            "REPLACE(CAST({} AS VARCHAR), {}, {}) AS {}",
                            quote_ident(field),
                            sql_literal(&params.from),
                            sql_literal(&params.to),
                            quote_ident(field)
                        )
                    } else {
                        quote_ident(field)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            create_duckdb_select(conn, next_table, &select, &state.table, None)
                .context("执行 Replace 清洗失败")?;
            state
                .field_types
                .insert(params.field, DuckFieldType::String);
        }
        CleanNodeType::TypeCast => {
            let params: TypeCastParams = parse_params(node)?;
            let (sql_type, field_type) = match params.target {
                TypeCastTarget::String => ("VARCHAR", DuckFieldType::String),
                TypeCastTarget::Number => ("DOUBLE", DuckFieldType::Number),
                TypeCastTarget::Boolean => ("BOOLEAN", DuckFieldType::Boolean),
            };
            let select = state
                .fields
                .iter()
                .map(|field| {
                    if *field == params.field {
                        format!(
                            "TRY_CAST({} AS {sql_type}) AS {}",
                            quote_ident(field),
                            quote_ident(field)
                        )
                    } else {
                        quote_ident(field)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            create_duckdb_select(conn, next_table, &select, &state.table, None)
                .context("执行 TypeCast 清洗失败")?;
            state.field_types.insert(params.field, field_type);
        }
        CleanNodeType::Filter => {
            let params: FilterParams = parse_params(node)?;
            let select = state
                .fields
                .iter()
                .map(|field| quote_ident(field))
                .collect::<Vec<_>>()
                .join(", ");
            let where_sql = filter_to_sql(&params);
            create_duckdb_select(conn, next_table, &select, &state.table, Some(&where_sql))
                .context("执行 Filter 清洗失败")?;
        }
        CleanNodeType::Dedupe => {
            let params: FieldListParams = parse_params(node)?;
            let select = state
                .fields
                .iter()
                .map(|field| quote_ident(field))
                .collect::<Vec<_>>()
                .join(", ");
            let partition = params
                .fields
                .iter()
                .map(|field| quote_ident(field))
                .collect::<Vec<_>>()
                .join(", ");
            conn.execute_batch(&format!(
                "CREATE TABLE {} AS \
                 SELECT {select} FROM ( \
                   SELECT *, ROW_NUMBER() OVER (PARTITION BY {partition} ORDER BY 1) AS __rn \
                   FROM {} \
                 ) WHERE __rn = 1;",
                quote_ident(next_table),
                quote_ident(&state.table),
            ))
            .context("执行 Dedupe 清洗失败")?;
        }
        CleanNodeType::DerivedField => {
            let params: DerivedFieldParams = parse_params(node)?;
            let mut fields = state.fields.clone();
            if !fields.contains(&params.field) {
                fields.push(params.field.clone());
            }
            let select = fields
                .iter()
                .map(|field| {
                    if *field == params.field {
                        format!(
                            "{} AS {}",
                            template_to_duckdb_concat(&params.template),
                            quote_ident(field)
                        )
                    } else {
                        quote_ident(field)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            create_duckdb_select(conn, next_table, &select, &state.table, None)
                .context("执行 DerivedField 清洗失败")?;
            state.fields = fields;
            state
                .field_types
                .insert(params.field, DuckFieldType::String);
        }
    }
    Ok(())
}

fn create_duckdb_select(
    conn: &Connection,
    next_table: &str,
    select: &str,
    source_table: &str,
    where_sql: Option<&str>,
) -> Result<()> {
    let where_clause = where_sql
        .filter(|sql| !sql.is_empty())
        .map(|sql| format!(" WHERE {sql}"))
        .unwrap_or_default();
    conn.execute_batch(&format!(
        "CREATE TABLE {} AS SELECT {select} FROM {}{where_clause};",
        quote_ident(next_table),
        quote_ident(source_table),
    ))?;
    Ok(())
}

fn filter_to_sql(params: &FilterParams) -> String {
    let field = quote_ident(&params.field);
    match params.op {
        FilterOp::Eq => format!(
            "CAST({field} AS VARCHAR) = {}",
            sql_literal(&value_to_string(
                params.value.as_ref().unwrap_or(&Value::Null)
            ))
        ),
        FilterOp::Ne => format!(
            "CAST({field} AS VARCHAR) <> {}",
            sql_literal(&value_to_string(
                params.value.as_ref().unwrap_or(&Value::Null)
            ))
        ),
        FilterOp::Contains => format!(
            "CONTAINS(CAST({field} AS VARCHAR), {})",
            sql_literal(&value_to_string(
                params.value.as_ref().unwrap_or(&Value::Null)
            ))
        ),
        FilterOp::NotContains => format!(
            "NOT CONTAINS(CAST({field} AS VARCHAR), {})",
            sql_literal(&value_to_string(
                params.value.as_ref().unwrap_or(&Value::Null)
            ))
        ),
        FilterOp::IsEmpty => format!("{field} IS NULL OR TRIM(CAST({field} AS VARCHAR)) = ''"),
        FilterOp::IsNotEmpty => {
            format!("{field} IS NOT NULL AND TRIM(CAST({field} AS VARCHAR)) <> ''")
        }
        FilterOp::Gt | FilterOp::Gte | FilterOp::Lt | FilterOp::Lte => {
            let op = match params.op {
                FilterOp::Gt => ">",
                FilterOp::Gte => ">=",
                FilterOp::Lt => "<",
                FilterOp::Lte => "<=",
                _ => unreachable!(),
            };
            format!(
                "TRY_CAST({field} AS DOUBLE) {op} TRY_CAST({} AS DOUBLE)",
                sql_literal(&value_to_string(
                    params.value.as_ref().unwrap_or(&Value::Null)
                ))
            )
        }
    }
}

fn template_to_duckdb_concat(template: &str) -> String {
    let mut parts = Vec::new();
    let mut rest = template;
    while let Some(start) = rest.find("${") {
        let (literal, after_start) = rest.split_at(start);
        if !literal.is_empty() {
            parts.push(sql_literal(literal));
        }
        if let Some(end) = after_start.find('}') {
            let field = &after_start[2..end];
            parts.push(format!(
                "COALESCE(CAST({} AS VARCHAR), '')",
                quote_ident(field)
            ));
            rest = &after_start[end + 1..];
        } else {
            parts.push(sql_literal(after_start));
            rest = "";
        }
    }
    if !rest.is_empty() {
        parts.push(sql_literal(rest));
    }
    if parts.is_empty() {
        sql_literal("")
    } else {
        format!("CONCAT({})", parts.join(", "))
    }
}

fn query_duckdb_rows(
    conn: &Connection,
    state: &DuckExecutionState,
    limit: Option<usize>,
) -> Result<Vec<Value>> {
    if state.fields.is_empty() {
        return Ok(vec![]);
    }
    let select = state
        .fields
        .iter()
        .map(|field| format!("CAST({} AS VARCHAR)", quote_ident(field)))
        .collect::<Vec<_>>()
        .join(", ");
    let limit_clause = limit.map(|n| format!(" LIMIT {n}")).unwrap_or_default();
    let sql = format!(
        "SELECT {select} FROM {}{limit_clause}",
        quote_ident(&state.table),
    );
    let mut stmt = conn.prepare(&sql).context("准备 DuckDB 输出查询失败")?;
    let rows = stmt
        .query_map([], |row| {
            let mut map = Map::new();
            for (idx, field) in state.fields.iter().enumerate() {
                let raw: Option<String> = row.get(idx)?;
                let value = raw
                    .map(|value| cast_duckdb_output(&value, state.field_types.get(field).copied()))
                    .unwrap_or(Value::Null);
                map.insert(field.clone(), value);
            }
            Ok(Value::Object(map))
        })
        .context("查询 DuckDB 输出数据失败")?;

    rows.collect::<std::result::Result<Vec<_>, _>>()
        .context("读取 DuckDB 输出数据失败")
}

fn cast_duckdb_output(value: &str, field_type: Option<DuckFieldType>) -> Value {
    match field_type.unwrap_or(DuckFieldType::String) {
        DuckFieldType::String => Value::String(value.to_string()),
        DuckFieldType::Number => value
            .parse::<f64>()
            .ok()
            .and_then(Number::from_f64)
            .map(Value::Number)
            .unwrap_or(Value::Null),
        DuckFieldType::Boolean => match value.to_ascii_lowercase().as_str() {
            "true" | "t" | "1" => Value::Bool(true),
            "false" | "f" | "0" => Value::Bool(false),
            _ => Value::Null,
        },
    }
}

fn quote_ident(ident: &str) -> String {
    format!("\"{}\"", ident.replace('"', "\"\""))
}

fn sql_literal(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

fn topo_sort(pipeline: &CleanPipeline) -> Result<Vec<&CleanNode>> {
    let node_map = pipeline
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect::<HashMap<_, _>>();
    let mut indegree = pipeline
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), 0usize))
        .collect::<HashMap<_, _>>();
    let mut adjacency: HashMap<&str, Vec<&str>> = HashMap::new();

    for CleanEdge { source, target, .. } in &pipeline.edges {
        if !node_map.contains_key(source.as_str()) || !node_map.contains_key(target.as_str()) {
            continue;
        }
        adjacency.entry(source).or_default().push(target);
        *indegree.entry(target).or_default() += 1;
    }

    let mut queue = indegree
        .iter()
        .filter_map(|(id, degree)| (*degree == 0).then_some(*id))
        .collect::<VecDeque<_>>();
    let mut sorted = Vec::with_capacity(pipeline.nodes.len());

    while let Some(id) = queue.pop_front() {
        let node = node_map
            .get(id)
            .copied()
            .ok_or_else(|| anyhow!("节点不存在: {id}"))?;
        sorted.push(node);
        for next in adjacency.get(id).into_iter().flatten() {
            if let Some(degree) = indegree.get_mut(next) {
                *degree -= 1;
                if *degree == 0 {
                    queue.push_back(next);
                }
            }
        }
    }

    if sorted.len() == pipeline.nodes.len() {
        Ok(sorted)
    } else {
        Err(anyhow!("清洗流程包含环"))
    }
}

fn parse_params<T>(node: &CleanNode) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_value(node.params.clone())
        .with_context(|| format!("解析节点参数失败: {}", node.id))
}

fn issue(
    node_id: Option<&str>,
    edge_id: Option<&str>,
    message: impl Into<String>,
) -> CleanValidationIssue {
    CleanValidationIssue {
        node_id: node_id.map(str::to_string),
        edge_id: edge_id.map(str::to_string),
        message: message.into(),
    }
}

fn as_object_mut(record: &mut Value) -> Option<&mut Map<String, Value>> {
    match record {
        Value::Object(map) => Some(map),
        _ => None,
    }
}

fn apply_select_rename(records: Vec<Value>, params: SelectRenameParams) -> Vec<Value> {
    records
        .into_iter()
        .map(|record| {
            let Value::Object(map) = record else {
                return record;
            };
            let mut next = Map::new();
            for field in &params.fields {
                let value = map.get(&field.from).cloned().unwrap_or(Value::Null);
                next.insert(field.to.clone(), value);
            }
            Value::Object(next)
        })
        .collect()
}

fn apply_trim(mut records: Vec<Value>, params: FieldListParams) -> Vec<Value> {
    for record in &mut records {
        let Some(map) = as_object_mut(record) else {
            continue;
        };
        for field in &params.fields {
            if let Some(Value::String(value)) = map.get_mut(field) {
                *value = value.trim().to_string();
            }
        }
    }
    records
}

fn apply_replace(mut records: Vec<Value>, params: ReplaceParams) -> Vec<Value> {
    for record in &mut records {
        let Some(map) = as_object_mut(record) else {
            continue;
        };
        if let Some(Value::String(value)) = map.get_mut(&params.field) {
            *value = value.replace(&params.from, &params.to);
        }
    }
    records
}

fn apply_type_cast(mut records: Vec<Value>, params: TypeCastParams) -> Vec<Value> {
    for record in &mut records {
        let Some(map) = as_object_mut(record) else {
            continue;
        };
        let Some(value) = map.get_mut(&params.field) else {
            continue;
        };
        *value = match params.target {
            TypeCastTarget::String => Value::String(value_to_string(value)),
            TypeCastTarget::Number => value_to_number(value)
                .map(Value::Number)
                .unwrap_or(Value::Null),
            TypeCastTarget::Boolean => value_to_bool(value).map(Value::Bool).unwrap_or(Value::Null),
        };
    }
    records
}

fn apply_filter(records: Vec<Value>, params: FilterParams) -> Vec<Value> {
    records
        .into_iter()
        .filter(|record| {
            let Value::Object(map) = record else {
                return false;
            };
            let actual = map.get(&params.field).unwrap_or(&Value::Null);
            filter_match(actual, &params)
        })
        .collect()
}

fn apply_dedupe(records: Vec<Value>, params: FieldListParams) -> Vec<Value> {
    let mut seen = HashSet::new();
    records
        .into_iter()
        .filter(|record| {
            let Value::Object(map) = record else {
                return true;
            };
            let key = params
                .fields
                .iter()
                .map(|field| map.get(field).map(value_to_string).unwrap_or_default())
                .collect::<Vec<_>>()
                .join("\u{1f}");
            seen.insert(key)
        })
        .collect()
}

fn apply_derived_field(mut records: Vec<Value>, params: DerivedFieldParams) -> Vec<Value> {
    for record in &mut records {
        let Some(map) = as_object_mut(record) else {
            continue;
        };
        let mut value = params.template.clone();
        for (key, field_value) in map.iter() {
            value = value.replace(&format!("${{{key}}}"), &value_to_string(field_value));
        }
        map.insert(params.field.clone(), Value::String(value));
    }
    records
}

fn filter_match(actual: &Value, params: &FilterParams) -> bool {
    match params.op {
        FilterOp::Eq => params
            .value
            .as_ref()
            .is_some_and(|expected| actual == expected),
        FilterOp::Ne => params
            .value
            .as_ref()
            .is_none_or(|expected| actual != expected),
        FilterOp::Contains => params
            .value
            .as_ref()
            .is_some_and(|expected| value_to_string(actual).contains(&value_to_string(expected))),
        FilterOp::NotContains => params
            .value
            .as_ref()
            .is_none_or(|expected| !value_to_string(actual).contains(&value_to_string(expected))),
        FilterOp::IsEmpty => is_empty_value(actual),
        FilterOp::IsNotEmpty => !is_empty_value(actual),
        FilterOp::Gt => compare_number(actual, params.value.as_ref(), |a, b| a > b),
        FilterOp::Gte => compare_number(actual, params.value.as_ref(), |a, b| a >= b),
        FilterOp::Lt => compare_number(actual, params.value.as_ref(), |a, b| a < b),
        FilterOp::Lte => compare_number(actual, params.value.as_ref(), |a, b| a <= b),
    }
}

fn compare_number(
    actual: &Value,
    expected: Option<&Value>,
    compare: impl FnOnce(f64, f64) -> bool,
) -> bool {
    let Some(actual) = value_to_f64(actual) else {
        return false;
    };
    let Some(expected) = expected.and_then(value_to_f64) else {
        return false;
    };
    compare(actual, expected)
}

fn is_empty_value(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::String(s) => s.trim().is_empty(),
        Value::Array(items) => items.is_empty(),
        Value::Object(map) => map.is_empty(),
        _ => false,
    }
}

fn value_to_bool(value: &Value) -> Option<bool> {
    match value {
        Value::Bool(v) => Some(*v),
        Value::Number(n) => n.as_f64().map(|v| v != 0.0),
        Value::String(s) => match s.trim().to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" | "y" => Some(true),
            "false" | "0" | "no" | "n" => Some(false),
            _ => None,
        },
        _ => None,
    }
}

fn value_to_number(value: &Value) -> Option<Number> {
    value_to_f64(value).and_then(Number::from_f64)
}

fn value_to_f64(value: &Value) -> Option<f64> {
    match value {
        Value::Number(n) => n.as_f64(),
        Value::String(s) => s.trim().parse::<f64>().ok(),
        Value::Bool(v) => Some(if *v { 1.0 } else { 0.0 }),
        _ => None,
    }
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::String(s) => s.clone(),
        Value::Bool(v) => v.to_string(),
        Value::Number(n) => n.to_string(),
        _ => serde_json::to_string(value).unwrap_or_default(),
    }
}

fn infer_schema(records: &[Value]) -> Vec<String> {
    let mut fields = Vec::new();
    let mut seen = HashSet::new();
    for record in records {
        let Value::Object(map) = record else {
            continue;
        };
        for key in map.keys() {
            if seen.insert(key.clone()) {
                fields.push(key.clone());
            }
        }
    }
    fields
}

fn to_csv(records: &[Value]) -> String {
    let fields = infer_schema(records);
    let mut lines = vec![fields
        .iter()
        .map(|v| csv_escape(v))
        .collect::<Vec<_>>()
        .join(",")];
    for record in records {
        let Value::Object(map) = record else {
            continue;
        };
        let line = fields
            .iter()
            .map(|field| map.get(field).map(value_to_string).unwrap_or_default())
            .map(|v| csv_escape(&v))
            .collect::<Vec<_>>()
            .join(",");
        lines.push(line);
    }
    lines.join("\n")
}

fn csv_escape(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') || value.contains('\r') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::views::data_clean::{CleanEdge, CleanNode, CleanNodeType, CleanPipeline};
    use serde_json::json;

    #[test]
    fn validates_cycle() {
        let pipeline = CleanPipeline {
            name: None,
            nodes: vec![
                node("source", CleanNodeType::Source, json!({})),
                node("trim", CleanNodeType::Trim, json!({ "fields": ["name"] })),
                node("sink", CleanNodeType::Sink, json!({})),
            ],
            edges: vec![
                edge("e1", "source", "trim"),
                edge("e2", "trim", "sink"),
                edge("e3", "sink", "trim"),
            ],
        };

        let resp = validate_pipeline(&pipeline);
        assert!(!resp.valid);
    }

    #[test]
    fn executes_basic_pipeline() {
        let pipeline = CleanPipeline {
            name: None,
            nodes: vec![
                node("source", CleanNodeType::Source, json!({})),
                node("trim", CleanNodeType::Trim, json!({ "fields": ["name"] })),
                node(
                    "rename",
                    CleanNodeType::SelectRename,
                    json!({ "fields": [{ "from": "name", "to": "title" }] }),
                ),
                node("sink", CleanNodeType::Sink, json!({})),
            ],
            edges: vec![
                edge("e1", "source", "trim"),
                edge("e2", "trim", "rename"),
                edge("e3", "rename", "sink"),
            ],
        };

        let resp = preview_pipeline(&pipeline, vec![json!({ "name": " Alice " })], None).unwrap();
        assert!(resp.valid);
        assert_eq!(resp.output, vec![json!({ "title": "Alice" })]);
    }

    fn node(id: &str, node_type: CleanNodeType, params: Value) -> CleanNode {
        CleanNode {
            id: id.to_string(),
            node_type,
            label: None,
            position: None,
            params,
        }
    }

    fn edge(id: &str, source: &str, target: &str) -> CleanEdge {
        CleanEdge {
            id: id.to_string(),
            source: source.to_string(),
            target: target.to_string(),
        }
    }
}
