mod gts;
use actix_web::{HttpRequest, HttpResponse, Responder, get};

#[get("/api/v1/{uri:.*}")]
pub async fn responder(req: HttpRequest) -> impl Responder {
    let req = req.match_info().get("uri").unwrap_or("");
    match req {
        "check/gts" => gts::serve().await,
        "health" => HttpResponse::Ok().content_type("application/json").body(r#"{"ok":"I'm fine thank u for asking~ :3"}"#),
        _      => HttpResponse::NotFound().content_type("application/json").body(r#"{"error":"API endpoint not found"}"#)
    }
}
