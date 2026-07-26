pub mod errors;
pub mod pages;
mod functions;
use crate::server::PAGE_COLLECTION;

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

#[macro_export]
macro_rules! _404Error {
    () => {
        match Err404Template.render() {
            Ok(render) => HttpResponse::NotFound().content_type("text/html").body(render),
            Err(e) => HttpResponse::InternalServerError().body(crate::renderer::errors::askama_error(e)),
        }
    };
}

// Homepage
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

// Pages (Root-level)
#[derive(Template)]
#[template(path = "page.html")]
struct PageTemplate { page: pages::Page }
functions::load_BlairStandardFunctions!(PageTemplate);
#[get("/{uri}")]
pub async fn page(req: HttpRequest) -> impl Responder {
    let uri = req.match_info().get("uri").unwrap();

    if let Ok(page_collection) = &PAGE_COLLECTION.read() {
        if let Some(page) = page_collection.iter().find(|page| {
            match page.slang() {
                Some(slang) => slang == uri,
                None => false,
            }}) 
        {
            let page_template = PageTemplate { page: page.clone() };
            return match page_template.render() {
                Ok(render) => HttpResponse::Ok()
                    .content_type("text/html")
                    .body(render),
                Err(e) => HttpResponse::InternalServerError().body(crate::renderer::errors::askama_error(e)),
            };
        }
    }

    match Err404Template.render() {
        Ok(render) => HttpResponse::NotFound().content_type("text/html").body(render),
        Err(e) => HttpResponse::InternalServerError().body(crate::renderer::errors::askama_error(e)),
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
    _404Error!()
}

