use crate::{
    config::tencent_ses::TencentSes,
    model::{
        marketing_attribution, marketing_campaign, marketing_delivery, marketing_event,
        marketing_lead,
        prelude::{
            MarketingAttribution, MarketingCampaign, MarketingDelivery, MarketingEvent,
            MarketingLead,
        },
    },
    router::ClientIp,
    service::tencent_ses::{ReceiverDetailWithData, TencentSesClient},
    utils::{jwt::AdminClaims, rand::rand_alphanumeric},
    views::marketing::*,
};
use anyhow::Context;
use axum_extra::headers::HeaderMap;
use axum_valid::Valid;
use chrono::Local;
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, Condition, DbConn, EntityTrait, ExprTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use summer_sea_orm::pagination::{Page, Pagination};
use summer_web::{
    axum::Json,
    error::{KnownWebError, Result},
    extractor::{Component, Config, Path, Query},
    get, post,
};

#[post("/admin/marketing/leads/import")]
async fn import_leads(
    _admin: AdminClaims,
    Component(db): Component<DbConn>,
    Valid(Json(req)): Valid<Json<ImportLeadsReq>>,
) -> Result<Json<ImportLeadsResp>> {
    let rows = parse_csv_emails(&req.csv_content);
    let mut created = 0usize;
    let mut duplicated = 0usize;
    let mut invalid = 0usize;

    for row in rows {
        let Some(email) = normalize_email(&row.email) else {
            invalid += 1;
            continue;
        };
        let exists = MarketingLead::find()
            .filter(marketing_lead::Column::Email.eq(&email))
            .one(&db)
            .await
            .context("查询营销线索失败")?;
        if exists.is_some() {
            duplicated += 1;
            continue;
        }

        marketing_lead::ActiveModel {
            id: NotSet,
            email: Set(email),
            name: Set(row.name),
            source: Set(req.source.clone()),
            tags: Set(None),
            extra: Set(Some(json!(row.extra))),
            status: Set("active".to_string()),
            unsubscribed: Set(false),
            last_seen_at: Set(None),
            ..Default::default()
        }
        .insert(&db)
        .await
        .context("保存营销线索失败")?;
        created += 1;
    }

    Ok(Json(ImportLeadsResp {
        created,
        duplicated,
        invalid,
    }))
}

#[get("/admin/marketing/leads")]
async fn query_leads(
    _admin: AdminClaims,
    Component(db): Component<DbConn>,
    Query(q): Query<LeadQuery>,
    pagination: Pagination,
) -> Result<Json<Page<MarketingLeadResp>>> {
    let mut filter = Condition::all();
    if let Some(keyword) = q.keyword.filter(|v| !v.trim().is_empty()) {
        let keyword = keyword.trim().to_string();
        filter = filter.add(
            marketing_lead::Column::Email
                .contains(&keyword)
                .or(marketing_lead::Column::Name.contains(&keyword)),
        );
    }
    if let Some(source) = q.source.filter(|v| !v.trim().is_empty()) {
        filter = filter.add(marketing_lead::Column::Source.eq(source.trim()));
    }
    if let Some(unsubscribed) = q.unsubscribed {
        filter = filter.add(marketing_lead::Column::Unsubscribed.eq(unsubscribed));
    }

    let total = MarketingLead::find()
        .filter(filter.clone())
        .count(&db)
        .await
        .context("统计营销线索失败")?;
    let rows = MarketingLead::find()
        .filter(filter)
        .order_by_desc(marketing_lead::Column::CreatedAt)
        .offset(pagination.page.saturating_mul(pagination.size))
        .limit(pagination.size)
        .all(&db)
        .await
        .context("查询营销线索失败")?;

    Ok(Json(Page::new(
        rows.into_iter().map(MarketingLeadResp::from).collect(),
        &pagination,
        total,
    )))
}

#[post("/admin/marketing/campaigns")]
async fn create_campaign(
    admin: AdminClaims,
    Component(db): Component<DbConn>,
    Valid(Json(req)): Valid<Json<CreateCampaignReq>>,
) -> Result<Json<MarketingCampaignResp>> {
    let mut lead_ids = req.lead_ids.clone();
    lead_ids.sort_unstable();
    lead_ids.dedup();

    let leads = MarketingLead::find()
        .filter(
            marketing_lead::Column::Id
                .is_in(lead_ids)
                .and(marketing_lead::Column::Unsubscribed.eq(false)),
        )
        .all(&db)
        .await
        .context("查询活动线索失败")?;
    if leads.is_empty() {
        return Err(KnownWebError::bad_request("没有可发送的收件人"))?;
    }

    let campaign = marketing_campaign::ActiveModel {
        id: NotSet,
        name: Set(req.name),
        subject: Set(req.subject),
        landing_url: Set(req.landing_url),
        status: Set("draft".to_string()),
        created_by: Set(admin.uid),
        scheduled_at: Set(None),
        provider_receiver_id: Set(None),
        provider_template_id: Set(Some(req.provider_template_id)),
        provider_task_id: Set(None),
        ..Default::default()
    }
    .insert(&db)
    .await
    .context("创建营销活动失败")?;

    for lead in leads {
        marketing_delivery::ActiveModel {
            id: NotSet,
            campaign_id: Set(campaign.id),
            lead_id: Set(lead.id),
            email: Set(lead.email),
            token: Set(rand_alphanumeric(48)),
            status: Set("pending".to_string()),
            sent_at: Set(None),
            provider_task_id: Set(None),
            provider_message_id: Set(None),
            error_message: Set(None),
            ..Default::default()
        }
        .insert(&db)
        .await
        .context("创建营销投递记录失败")?;
    }

    campaign_resp(&db, campaign).await.map(Json)
}

#[get("/admin/marketing/campaigns")]
async fn query_campaigns(
    _admin: AdminClaims,
    Component(db): Component<DbConn>,
    Query(q): Query<CampaignQuery>,
    pagination: Pagination,
) -> Result<Json<Page<MarketingCampaignResp>>> {
    let mut filter = Condition::all();
    if let Some(keyword) = q.keyword.filter(|v| !v.trim().is_empty()) {
        filter = filter.add(marketing_campaign::Column::Name.contains(keyword.trim()));
    }
    if let Some(status) = q.status.filter(|v| !v.trim().is_empty()) {
        filter = filter.add(marketing_campaign::Column::Status.eq(status.trim()));
    }

    let total = MarketingCampaign::find()
        .filter(filter.clone())
        .count(&db)
        .await
        .context("统计营销活动失败")?;
    let campaigns = MarketingCampaign::find()
        .filter(filter)
        .order_by_desc(marketing_campaign::Column::CreatedAt)
        .offset(pagination.page.saturating_mul(pagination.size))
        .limit(pagination.size)
        .all(&db)
        .await
        .context("查询营销活动失败")?;
    let mut content = Vec::with_capacity(campaigns.len());
    for campaign in campaigns {
        content.push(campaign_resp(&db, campaign).await?);
    }

    Ok(Json(Page::new(content, &pagination, total)))
}

#[post("/admin/marketing/campaigns/{id}/send")]
async fn send_campaign(
    _admin: AdminClaims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
    Config(config): Config<TencentSes>,
) -> Result<Json<SendCampaignResp>> {
    let campaign = MarketingCampaign::find_by_id(id)
        .one(&db)
        .await
        .context("查询营销活动失败")?
        .ok_or_else(|| KnownWebError::not_found("营销活动不存在"))?;
    let template_id = campaign
        .provider_template_id
        .as_deref()
        .and_then(|v| v.parse::<i64>().ok())
        .ok_or_else(|| KnownWebError::bad_request("腾讯云 SES 模板 ID 不正确"))?;
    if campaign.provider_task_id.is_some() {
        return Err(KnownWebError::bad_request("该活动已创建发送任务"))?;
    }

    let deliveries = MarketingDelivery::find()
        .filter(marketing_delivery::Column::CampaignId.eq(id))
        .filter(marketing_delivery::Column::Status.eq("pending"))
        .all(&db)
        .await
        .context("查询待发送收件人失败")?;
    if deliveries.is_empty() {
        return Err(KnownWebError::bad_request("没有待发送收件人"))?;
    }

    let client = TencentSesClient::new(config);
    let receiver_id = client
        .create_receiver(
            &format!("AutoWDS-{}-{}", campaign.id, campaign.name),
            "AutoWDS 营销活动收件人列表",
        )
        .await
        .context("创建腾讯云 SES 收件人列表失败")?;
    let details = deliveries
        .iter()
        .map(|d| ReceiverDetailWithData {
            email: d.email.clone(),
            template_data: json!({
                "mtk": d.token,
                "landing_url": landing_url_with_token(&campaign.landing_url, &d.token),
                "campaign_id": campaign.id,
                "delivery_id": d.id,
            })
            .to_string(),
        })
        .collect::<Vec<_>>();
    client
        .create_receiver_detail_with_data(receiver_id, details)
        .await
        .context("同步腾讯云 SES 收件人失败")?;
    let task_id = client
        .batch_send_email(receiver_id, template_id, &campaign.subject)
        .await
        .context("创建腾讯云 SES 发送任务失败")?;

    marketing_campaign::ActiveModel {
        id: Set(campaign.id),
        status: Set("sending".to_string()),
        provider_receiver_id: Set(Some(receiver_id.to_string())),
        provider_task_id: Set(Some(task_id.to_string())),
        ..Default::default()
    }
    .update(&db)
    .await
    .context("更新营销活动发送状态失败")?;

    let now = Local::now().naive_local();
    for delivery in &deliveries {
        marketing_delivery::ActiveModel {
            id: Set(delivery.id),
            status: Set("sent".to_string()),
            sent_at: Set(Some(now)),
            provider_task_id: Set(Some(task_id.to_string())),
            ..Default::default()
        }
        .update(&db)
        .await
        .context("更新投递状态失败")?;
        insert_event(
            &db,
            Some(campaign.id),
            Some(delivery.id),
            Some(delivery.lead_id),
            "sent",
            None,
            None,
            None,
            Some(json!({ "provider_task_id": task_id })),
        )
        .await?;
    }

    Ok(Json(SendCampaignResp {
        receiver_id,
        task_id,
        delivery_count: deliveries.len() as u64,
    }))
}

#[get("/admin/marketing/campaigns/{id}/funnel")]
async fn campaign_funnel(
    _admin: AdminClaims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
) -> Result<Json<CampaignFunnelResp>> {
    let sent = MarketingDelivery::find()
        .filter(marketing_delivery::Column::CampaignId.eq(id))
        .count(&db)
        .await
        .context("统计发送人数失败")? as usize;
    let events = MarketingEvent::find()
        .filter(marketing_event::Column::CampaignId.eq(id))
        .all(&db)
        .await
        .context("查询漏斗事件失败")?;
    let mut counts: HashMap<String, HashSet<i64>> = HashMap::new();
    for event in events {
        if let Some(delivery_id) = event.delivery_id {
            counts
                .entry(event.event_type)
                .or_default()
                .insert(delivery_id);
        }
    }
    let order = [
        "delivered",
        "opened",
        "email_link_clicked",
        "landing_opened",
        "plugin_installed",
        "registered",
        "purchased",
    ];
    let mut previous = sent.max(1);
    let metrics = order
        .into_iter()
        .map(|event_type| {
            let count = counts.get(event_type).map(HashSet::len).unwrap_or_default();
            let metric = FunnelMetric {
                event_type: event_type.to_string(),
                count,
                rate: count as f64 / previous as f64,
            };
            previous = count.max(1);
            metric
        })
        .collect();

    Ok(Json(CampaignFunnelResp {
        campaign_id: id,
        sent,
        metrics,
    }))
}

#[get("/admin/marketing/campaigns/{id}/events")]
async fn campaign_events(
    _admin: AdminClaims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
) -> Result<Json<Vec<marketing_event::Model>>> {
    let rows = MarketingEvent::find()
        .filter(marketing_event::Column::CampaignId.eq(id))
        .order_by_desc(marketing_event::Column::CreatedAt)
        .limit(500)
        .all(&db)
        .await
        .context("查询营销事件失败")?;
    Ok(Json(rows))
}

#[post("/marketing/webhook/tencent-ses")]
async fn tencent_ses_webhook(
    Component(db): Component<DbConn>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<MarketingEventResp>> {
    let event = body
        .get("event")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let event_type = map_tencent_event(event)
        .ok_or_else(|| KnownWebError::bad_request("不支持的腾讯云 SES 事件"))?;
    let email = body
        .get("email")
        .and_then(Value::as_str)
        .map(str::to_ascii_lowercase);
    let delivery_id = body
        .get("X-Tencentcloudses-Cb-delivery_id")
        .and_then(Value::as_str)
        .and_then(|v| v.parse::<i64>().ok());
    let task_id = body
        .get("taskId")
        .and_then(Value::as_i64)
        .map(|v| v.to_string())
        .or_else(|| {
            body.get("bulkId")
                .and_then(Value::as_str)
                .map(str::to_string)
        });

    let delivery = match delivery_id {
        Some(id) => MarketingDelivery::find_by_id(id)
            .one(&db)
            .await
            .context("按 delivery_id 查询投递记录失败")?,
        None => {
            if let (Some(task_id), Some(email)) = (task_id.as_deref(), email.as_deref()) {
                MarketingDelivery::find()
                    .filter(marketing_delivery::Column::ProviderTaskId.eq(task_id))
                    .filter(marketing_delivery::Column::Email.eq(email))
                    .one(&db)
                    .await
                    .context("按 task/email 查询投递记录失败")?
            } else {
                None
            }
        }
    };

    let url = body.get("link").and_then(Value::as_str).map(str::to_string);
    let user_agent = body
        .get("useragent")
        .and_then(Value::as_str)
        .or_else(|| headers.get("user-agent").and_then(|v| v.to_str().ok()))
        .map(str::to_string);

    let (campaign_id, delivery_id, lead_id) = delivery
        .as_ref()
        .map(|d| (Some(d.campaign_id), Some(d.id), Some(d.lead_id)))
        .unwrap_or((None, None, None));
    insert_event(
        &db,
        campaign_id,
        delivery_id,
        lead_id,
        event_type,
        url,
        user_agent,
        None,
        Some(body.clone()),
    )
    .await?;

    if let Some(delivery) = delivery {
        let status = match event_type {
            "delivered" => Some("delivered"),
            "dropped" | "bounced" => Some("failed"),
            "deferred" => Some("deferred"),
            "unsubscribed" => Some("unsubscribed"),
            _ => None,
        };
        if let Some(status) = status {
            marketing_delivery::ActiveModel {
                id: Set(delivery.id),
                status: Set(status.to_string()),
                provider_message_id: Set(body
                    .get("messageId")
                    .and_then(Value::as_str)
                    .or_else(|| body.get("bulkId").and_then(Value::as_str))
                    .map(str::to_string)),
                error_message: Set(body
                    .get("reason")
                    .and_then(Value::as_str)
                    .map(str::to_string)),
                ..Default::default()
            }
            .update(&db)
            .await
            .context("更新投递事件状态失败")?;
        }
        if matches!(event_type, "unsubscribed" | "bounced" | "spam_reported") {
            marketing_lead::ActiveModel {
                id: Set(delivery.lead_id),
                unsubscribed: Set(true),
                status: Set(event_type.to_string()),
                ..Default::default()
            }
            .update(&db)
            .await
            .context("更新线索退订/抑制状态失败")?;
        }
    }

    Ok(Json(MarketingEventResp { recorded: true }))
}

#[post("/marketing/event")]
async fn record_marketing_event(
    Component(db): Component<DbConn>,
    ClientIp(client_ip): ClientIp,
    headers: HeaderMap,
    Json(req): Json<MarketingEventReq>,
) -> Result<Json<MarketingEventResp>> {
    let allowed = ["landing_opened", "plugin_installed"];
    if !allowed.contains(&req.event_type.as_str()) {
        return Err(KnownWebError::bad_request("不支持的营销事件"))?;
    }
    let delivery = MarketingDelivery::find()
        .filter(marketing_delivery::Column::Token.eq(&req.mtk))
        .one(&db)
        .await
        .context("查询营销归因 token 失败")?
        .ok_or_else(|| KnownWebError::bad_request("营销归因 token 无效"))?;
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(str::to_string);
    insert_event(
        &db,
        Some(delivery.campaign_id),
        Some(delivery.id),
        Some(delivery.lead_id),
        &req.event_type,
        req.url,
        user_agent,
        Some(hash_ip(&client_ip.0.to_string())),
        req.meta,
    )
    .await?;
    Ok(Json(MarketingEventResp { recorded: true }))
}

pub async fn record_register_by_token(
    db: &DbConn,
    user_id: i64,
    token: &str,
) -> anyhow::Result<()> {
    if token.trim().is_empty() {
        return Ok(());
    }
    let Some(delivery) = MarketingDelivery::find()
        .filter(marketing_delivery::Column::Token.eq(token.trim()))
        .one(db)
        .await
        .context("find marketing delivery by token failed")?
    else {
        return Ok(());
    };
    marketing_attribution::ActiveModel {
        user_id: Set(user_id),
        lead_id: Set(Some(delivery.lead_id)),
        campaign_id: Set(Some(delivery.campaign_id)),
        delivery_id: Set(Some(delivery.id)),
        ..Default::default()
    }
    .insert(db)
    .await
    .context("insert marketing attribution failed")?;
    insert_event(
        db,
        Some(delivery.campaign_id),
        Some(delivery.id),
        Some(delivery.lead_id),
        "registered",
        None,
        None,
        None,
        Some(json!({ "user_id": user_id })),
    )
    .await?;
    Ok(())
}

pub async fn record_purchase_by_user(db: &DbConn, user_id: i64) -> anyhow::Result<()> {
    let Some(attr) = MarketingAttribution::find_by_id(user_id)
        .one(db)
        .await
        .context("find marketing attribution failed")?
    else {
        return Ok(());
    };
    insert_event(
        db,
        attr.campaign_id,
        attr.delivery_id,
        attr.lead_id,
        "purchased",
        None,
        None,
        None,
        Some(json!({ "user_id": user_id })),
    )
    .await
}

async fn campaign_resp(
    db: &DbConn,
    campaign: marketing_campaign::Model,
) -> Result<MarketingCampaignResp> {
    let delivery_count = MarketingDelivery::find()
        .filter(marketing_delivery::Column::CampaignId.eq(campaign.id))
        .count(db)
        .await
        .context("统计活动收件人数失败")?;
    Ok(MarketingCampaignResp {
        id: campaign.id,
        name: campaign.name,
        subject: campaign.subject,
        landing_url: campaign.landing_url,
        status: campaign.status,
        provider_receiver_id: campaign.provider_receiver_id,
        provider_template_id: campaign.provider_template_id,
        provider_task_id: campaign.provider_task_id,
        created_at: campaign.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        delivery_count,
    })
}

async fn insert_event(
    db: &DbConn,
    campaign_id: Option<i64>,
    delivery_id: Option<i64>,
    lead_id: Option<i64>,
    event_type: &str,
    url: Option<String>,
    user_agent: Option<String>,
    ip_hash: Option<String>,
    meta: Option<Value>,
) -> anyhow::Result<()> {
    marketing_event::ActiveModel {
        id: NotSet,
        campaign_id: Set(campaign_id),
        delivery_id: Set(delivery_id),
        lead_id: Set(lead_id),
        event_type: Set(event_type.to_string()),
        url: Set(url),
        user_agent: Set(user_agent),
        ip_hash: Set(ip_hash),
        meta: Set(meta),
        ..Default::default()
    }
    .insert(db)
    .await
    .context("insert marketing event failed")?;
    Ok(())
}

fn map_tencent_event(event: &str) -> Option<&'static str> {
    match event {
        "delivered" => Some("delivered"),
        "dropped" => Some("dropped"),
        "bounce" => Some("bounced"),
        "deferred" => Some("deferred"),
        "open" => Some("opened"),
        "click" => Some("email_link_clicked"),
        "spamreport" => Some("spam_reported"),
        "unsubscribe" => Some("unsubscribed"),
        _ => None,
    }
}

fn landing_url_with_token(url: &str, token: &str) -> String {
    let sep = if url.contains('?') { '&' } else { '?' };
    format!(
        "{url}{sep}utm_source=email&utm_medium=marketing&mtk={}",
        urlencoding::encode(token)
    )
}

fn hash_ip(ip: &str) -> String {
    hex::encode(Sha256::digest(ip.as_bytes()))
}

fn normalize_email(email: &str) -> Option<String> {
    let email = email.trim().trim_matches('"').to_ascii_lowercase();
    let valid = email.contains('@') && email.len() <= 128 && !email.contains(' ');
    valid.then_some(email)
}

#[derive(Debug)]
struct CsvLeadRow {
    email: String,
    name: Option<String>,
    extra: Map<String, Value>,
}

fn parse_csv_emails(csv: &str) -> Vec<CsvLeadRow> {
    let mut lines = csv.lines().filter(|line| !line.trim().is_empty());
    let Some(first) = lines.next() else {
        return vec![];
    };
    let first_cols = parse_csv_line(first);
    let has_header = first_cols.iter().any(|v| {
        matches!(
            v.trim().to_ascii_lowercase().as_str(),
            "email" | "e-mail" | "mail"
        )
    });
    let headers = if has_header {
        first_cols
            .iter()
            .map(|v| v.trim().to_ascii_lowercase())
            .collect::<Vec<_>>()
    } else {
        vec!["email".to_string(), "name".to_string()]
    };

    let mut rows = Vec::new();
    if !has_header {
        rows.push(csv_row_from_cols(&headers, first_cols));
    }
    rows.extend(lines.map(|line| csv_row_from_cols(&headers, parse_csv_line(line))));
    rows
}

fn csv_row_from_cols(headers: &[String], cols: Vec<String>) -> CsvLeadRow {
    let mut extra = Map::new();
    let mut email = String::new();
    let mut name = None;
    for (idx, value) in cols.into_iter().enumerate() {
        let key = headers
            .get(idx)
            .cloned()
            .unwrap_or_else(|| format!("col_{idx}"));
        match key.as_str() {
            "email" | "e-mail" | "mail" => email = value,
            "name" | "username" | "用户名" | "姓名" => name = Some(value),
            _ => {
                extra.insert(key, Value::String(value));
            }
        }
    }
    CsvLeadRow { email, name, extra }
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut cols = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '"' if in_quotes && chars.peek() == Some(&'"') => {
                current.push('"');
                chars.next();
            }
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => {
                cols.push(current.trim().trim_matches('"').to_string());
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    cols.push(current.trim().trim_matches('"').to_string());
    cols
}

impl From<marketing_lead::Model> for MarketingLeadResp {
    fn from(lead: marketing_lead::Model) -> Self {
        Self {
            id: lead.id,
            email: lead.email,
            name: lead.name,
            source: lead.source,
            status: lead.status,
            unsubscribed: lead.unsubscribed,
            created_at: lead.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            last_seen_at: lead
                .last_seen_at
                .map(|v| v.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}
