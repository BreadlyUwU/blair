use crate::config::BLAIR;
use crate::server::PAGE_COLLECTION;
pub mod errors;
pub mod pages;
mod functions;

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

// Pages
#[derive(Template)]
#[template(path = "page.html")]
struct PageTemplate { page: pages::Page }
functions::load_BlairStandardFunctions!(PageTemplate);
#[get("/{uri:.*}")]
pub async fn page(req: HttpRequest) -> impl Responder {
    let uri = req.match_info().get("uri").unwrap();
    let normal_uri = if uri.ends_with("/") { uri.split_at(uri.len() - 1).0 } else { uri };
    let pages = &PAGE_COLLECTION.read().unwrap();

    // Verifie en premier si la requête porte vers un fichier se trouvant à l'intérieur
    // du dossier dédié à la page et le renvoi le cas échéant
    if pages::is_potential_opt_file(normal_uri) {
        let expected_file = format!("./content/pages/{uri}");
        match actix_files::NamedFile::open(expected_file) {
            Ok(file) => file.into_response(&req),
            Err(_) => _404Error!()
        }
    } 
    else {
        match pages.iter().find(|page| page.id == normal_uri) {
            Some(page) => {
                let template = PageTemplate { page: page.clone() };
                return match template.render() {           
                    Ok(render) => { 
                        // Redirection automatique pour les pages, afin que l'URL se termine systématiquement par un slash
                        // (Sinon, ça fout la merde)
                        if !uri.ends_with("/") {
                            return HttpResponse::MovedPermanently().append_header(("Location", format!("/{}/", normal_uri))).finish()
                        }
                        HttpResponse::Ok().content_type("text/html").body(render)
                    },
                    Err(e) => HttpResponse::InternalServerError().body(crate::renderer::errors::askama_error(e))
                }
            },
            None => _404Error!()
        }
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

