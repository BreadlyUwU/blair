use crate::config;

use actix_web::{
    HttpResponse, 
    Responder, 
    get
};
use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
struct BaseTemplate {
    version: String,
}

impl BaseTemplate {
    fn get_url(&self, val: &str) -> String {
        let base_url = config::Configuration::base_url();
        return format!("{base_url}{val}");
    }
}

#[get("/")]
pub async fn home() -> impl Responder {
    let helloworld = BaseTemplate { 
        version: String::from(env!("CARGO_PKG_VERSION"))
    };
    match helloworld.render() {
        Ok(render) => HttpResponse::Ok().content_type("text/html").body(render),
        Err(e) => HttpResponse::InternalServerError().body(crate::renderer::error::render_error(e)),
    }
}

pub async fn _404() -> impl Responder {
    let template = Err404Template;
    match template.render() {
        Ok(render) => HttpResponse::NotFound().content_type("text/html").body(render),
        Err(e) => HttpResponse::InternalServerError().body(crate::renderer::error::render_error(e)),
    }
}