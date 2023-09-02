use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use self::user::UserDoc;

pub mod template;
pub mod token;
pub mod user;

pub fn config(cfg: &mut web::ServiceConfig) {
    let openapi = UserDoc::openapi();
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
    )
    .service(user::user_scope())
    .service(template::template_scope())
    .service(token::token_scope());
}
