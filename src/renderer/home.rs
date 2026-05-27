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
struct HomeTemplate;

functions::load_BlairStandardFunctions!(HomeTemplate);

#[get("/")]
pub async fn home() -> impl Responder {
    let template = HomeTemplate;
    match template.render() {
        Ok(render) => HttpResponse::Ok().content_type("text/html").body(render),
        Err(e) => HttpResponse::InternalServerError().body(crate::renderer::errors::askama_error(e)),
    }
}