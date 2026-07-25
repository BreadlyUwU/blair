// pub mod root;
pub mod errors;
mod pages;
mod functions;

use crate::config;
// use super::functions;
use functions::BlairStandardFunctions;
use actix_web::{
    HttpResponse,
    HttpRequest,
    Responder,
    get
};
use askama::Template;

// Root page
#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;
functions::load_BlairStandardFunctions!(HomeTemplate);
#[get("/")]
pub async fn home() -> impl Responder {
    match HomeTemplate.render() {
        Ok(render) => HttpResponse::Ok().content_type("text/html").body(render),
        Err(e) => HttpResponse::InternalServerError().body(crate::renderer::errors::askama_error(e)),
    }
}

// Potential pages
#[get("/{uri}")]
pub async fn root(req: HttpRequest) -> impl Responder {
    let uri = req.match_info().get("uri").unwrap();
    match uri {
        "foo" => HttpResponse::Ok().body("bar"),
        // Render the regular 404 error page if any invalid input is received
        _ => {
            // let template = Err404Template;
            match Err404Template.render() {
                Ok(render) => HttpResponse::NotFound().content_type("text/html").body(render),
                Err(e) => HttpResponse::InternalServerError().body(crate::renderer::errors::askama_error(e)),
            }
        },
    }
}

// Favicon
#[get("/favicon{ext}")]
pub async fn favicon(req: HttpRequest) -> actix_web::Result<actix_files::NamedFile> {
    let ext = req.match_info().get("ext").unwrap();
    match ext {
        ".ico" => Ok(actix_files::NamedFile::open("./static/favicon.ico")?),
        ".png" => Ok(actix_files::NamedFile::open("./static/favicon.png")?),
        // Fallback: favicon.ico | IDK how to deal with errors here
        "" => Ok(actix_files::NamedFile::open("./static/favicon.ico")?),
        _  => Ok(actix_files::NamedFile::open("./static/favicon.ico")?)
    }
}

// 404 Error Page
#[derive(Template)]
#[template(path = "404.html")]
pub struct Err404Template;
functions::load_BlairStandardFunctions!(Err404Template);
pub async fn _404() -> impl Responder {
    match Err404Template.render() {
        Ok(render) => HttpResponse::NotFound().content_type("text/html").body(render),
        Err(e) => HttpResponse::InternalServerError().body(crate::renderer::errors::askama_error(e)),
    }
}
