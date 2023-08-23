use actix_web::{get, web, HttpResponse, Responder, Scope};
use serde::{Deserialize, Serialize};

use crate::utils::jwt::Claims;

pub fn template_scope() -> Scope {
    return web::scope("/template")
        .service(query_template)
        .service(find_by_id);
}

#[get("/")]
async fn query_template(
    claims: Option<web::ReqData<Claims>>,
    query: web::Query<TemplateQuery>,
) -> impl Responder {
    HttpResponse::Ok().body(query.name.clone())
}

#[get("/{id}")]
async fn find_by_id(id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(id.to_string())
}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateQuery {
    name: String,
    topic: String,
    edition: String,
}
