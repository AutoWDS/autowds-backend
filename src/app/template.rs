use actix_web::{get, web, HttpResponse, Responder, Scope};

use crate::{model::task_template::TemplateQuery, utils::jwt::Claims};

pub fn template_scope() -> Scope {
    return web::scope("/template")
        .service(query_template)
        .service(find_by_id);
}

#[get("")]
async fn query_template(
    claims: Option<web::ReqData<Claims>>,
    query: web::Query<TemplateQuery>,
) -> impl Responder {
    match claims {
        None => "",
        Some(claims) => "",
    }
}

#[get("/{id}")]
async fn find_by_id(id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(id.to_string())
}
