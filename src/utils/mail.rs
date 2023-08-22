use askama::Template;
use envconfig::Envconfig;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

/// 验证码的邮件模板
#[derive(Template)]
#[template(path = "mail/validate_code.html")]
pub struct ValidateCodeMailTemplate<'a> {
    pub tip: &'a str,
    pub validate_code: &'a str,
}

/// 邮件服务的相关配置
#[derive(Envconfig)]
struct MailConfig {
    #[envconfig(from = "MAIL_HOST")]
    pub mail_host: String,

    #[envconfig(from = "MAIL_USERNAME")]
    pub username: String,

    #[envconfig(from = "MAIL_PASSWORD")]
    pub password: String,

    #[envconfig(from = "MAIL_FROM")]
    pub from: String,
}

/// 使用askama模板发送邮件
/// - send_to: 要发送的目的邮件地址
/// - subject: 邮件主题
/// - body: 邮件模板
pub fn send_mail(send_to: &str, subject: &str, body: &impl Template) {
    let config = MailConfig::init_from_env().unwrap();

    let email = Message::builder()
        .from(config.from.parse().unwrap())
        .to(send_to.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(body.render().unwrap())
        .unwrap();

    let creds = Credentials::new(config.username.to_owned(), config.password.to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&config.mail_host)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => log::trace!("Email sent successfully!"),
        Err(e) => log::error!("Could not send email: {:?}", e),
    }
}
