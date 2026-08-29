use actix_web::{HttpResponse, Responder, get};

#[get("/health")]
pub async fn check() -> impl Responder {
    HttpResponse::Ok()
        //.content_type("text/html")
        .body("I'm fine thank u for asking~ :3")
}
