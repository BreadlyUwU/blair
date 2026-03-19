use crate::config;
use super::functions;
use super::functions::BlairStandardFunctions;

use actix_web::{
    HttpResponse, 
    Responder, 
    get
};
use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
struct BaseTemplate;

#[derive(Template)]
#[template(path = "404.html")]
struct Err404Template;

functions::load_BlairStandardFunctions!(BaseTemplate, Err404Template);

#[get("/")]
pub async fn home() -> impl Responder {
    let template = BaseTemplate;
    match template.render() {
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