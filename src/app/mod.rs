use actix_web::web;

mod template;
mod token;
mod user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(user::user_scope())
        .service(template::template_scope())
        .service(token::token_scope());
}
