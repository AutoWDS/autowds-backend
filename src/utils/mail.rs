use anyhow::Context;
use askama::Template;
use summer_mail::{header::ContentType, AsyncTransport, Mailer, Message};
use summer_web::error::Result;

async fn send_raw_mail(
    mailer: &Mailer,
    from: &str,
    to: &str,
    subject: &str,
    content_type: ContentType,
    body: String,
) -> Result<bool> {
    let from_mail_box = from
        .parse()
        .with_context(|| format!("email {} is invalid", from))?;
    let to_mailbox = to
        .parse()
        .with_context(|| format!("email {} is invalid", to))?;
    let message = Message::builder()
        .from(from_mail_box)
        .to(to_mailbox)
        .subject(subject)
        .header(content_type)
        .body(body)
        .context("mail build error")?;

    let resp = mailer
        .send(message)
        .await
        .with_context(|| format!("send mail to {to} failed"))?;

    Ok(resp.is_positive())
}

pub async fn send_mail<T: Template>(
    mailer: &Mailer,
    from: &str,
    to: &str,
    subject: &str,
    body: &T,
) -> Result<bool> {
    let content_type = ContentType::parse(T::MIME_TYPE)
        .with_context(|| format!("content type parse failed: {}", T::MIME_TYPE))?;
    let body = body.render().context("template render failed")?;
    send_raw_mail(mailer, from, to, subject, content_type, body).await
}

/// 发送 HTML 正文邮件（营销邮件等）
pub async fn send_html_mail(
    mailer: &Mailer,
    from: &str,
    to: &str,
    subject: &str,
    html_body: &str,
) -> Result<bool> {
    let content_type = ContentType::parse("text/html; charset=utf-8")
        .context("content type parse failed for text/html")?;
    send_raw_mail(
        mailer,
        from,
        to,
        subject,
        content_type,
        html_body.to_string(),
    )
    .await
}
