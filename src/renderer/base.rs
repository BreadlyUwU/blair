use actix_web::{
    HttpResponse, 
    Responder, 
    get
};
use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
struct BaseTemplate<'a> {
    version: &'a str,
}

#[get("/")]
pub async fn home() -> impl Responder {
    let helloworld = BaseTemplate { 
        version: env!("CARGO_PKG_VERSION") 
    };
    match helloworld.render() {
        Ok(render) => HttpResponse::Ok().content_type("text/html").body(render),
        Err(e) => HttpResponse::InternalServerError().body(crate::renderer::error::render_error(e)),
    }
}