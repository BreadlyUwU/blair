pub mod atom;
pub mod errors;
pub mod pages;
mod functions;
use crate::config::{BLAIR, RunningMode};
use pages::{BLOG_COLLECTION, PAGE_COLLECTION, Page};
use functions::BlairStandardFunctions;

use actix_web::{
    error::ErrorNotFound,
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

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;
functions::load_BlairStandardFunctions!(HomeTemplate);

#[derive(Template)]
#[template(path = "page.html")]
struct PageTemplate<'a> { page: &'a Page }
functions::load_BlairStandardFunctions!(PageTemplate<'_>);

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate<'a> { posts: &'a [Page] }
functions::load_BlairStandardFunctions!(BlogTemplate<'_>);

#[derive(Template)]
#[template(path = "blog-post.html")]
struct BlogPostTemplate<'a> { post: &'a Page }
functions::load_BlairStandardFunctions!(BlogPostTemplate<'_>);

// Serving the website's root, so the homepage and… the p-a-g-i-e-s 📃📃📃
#[get("/{uri:.*}")]
pub async fn root(req: HttpRequest) -> impl Responder {
    let uri = req.match_info().get("uri").unwrap_or("");
    let normal_uri = if uri.ends_with("/") { uri.split_at(uri.len() - 1).0 } else { uri };
    let pages = &PAGE_COLLECTION;
    if uri.is_empty() || uri == "/" {
        // Homepage
        match HomeTemplate.render() {
            Ok(render) => HttpResponse::Ok().content_type("text/html").body(render),
            Err(e) => HttpResponse::InternalServerError().body(crate::renderer::errors::askama_error(e)),
        }
    } else {
        // Page's static files
        if pages::is_potential_opt_file(normal_uri) {
            let expected_file = format!("./content/pages/{uri}");
            match actix_files::NamedFile::open(expected_file) {
                Ok(file) => file.into_response(&req),
                Err(_) => _404Error!()
            }
        }
        else {
            // The pages
            match pages.iter().find(|page| page.are_uri_and_id_kissing(normal_uri)) {
                Some(page) => {
                    let template = PageTemplate { page };
                    return match template.render() {
                        Ok(render) => {
                            // Redirection automatique pour les pages, afin que l'URL se termine systématiquement par un slash
                            // (Sinon, ça fout la merde)
                            if !uri.ends_with("/") {
                                let canonical_path = if page.id == normal_uri { format!("/{}", normal_uri) }
                                                     else { format!("/{}/{}", page.root, page.id) };
                                return HttpResponse::MovedPermanently().append_header(("Location", format!("{canonical_path}/"))).finish()
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
}

#[get("/blog{uri:.*}")]
pub async fn blog(req: HttpRequest) -> impl Responder {
    let uri = req.match_info().get("uri").unwrap_or("");
    let normal_uri = &uri.trim_end_matches('/');

    // Blog's homepage
    if uri.is_empty() || uri == "/" {
        let template = BlogTemplate { posts: &BLOG_COLLECTION };
        match template.render() {
            Ok(render) => {
                if !uri.ends_with("/") {
                    return HttpResponse::MovedPermanently().append_header(("Location", "/blog/".to_string())).finish()
                }
                return HttpResponse::Ok().content_type("text/html").body(render)
            },
            Err(e) => return HttpResponse::InternalServerError().body(crate::renderer::errors::askama_error(e))
        }
    }

    // Verifie en premier si la requête porte vers un fichier se trouvant à l'intérieur
    // du dossier dédié à la page et le renvoi le cas échéant
    if pages::is_potential_opt_file(normal_uri) {
        let expected_file = format!("./content/blog/{}", normal_uri.trim_start_matches("/"));
        match actix_files::NamedFile::open(expected_file) {
            Ok(file) => file.into_response(&req),
            Err(_) => _404Error!()
        }
    }
    else {
        // The blog posts
        match BLOG_COLLECTION.iter().find(|post| post.id == normal_uri.trim_start_matches("/")) {
            Some(post) => {
                let template = BlogPostTemplate { post };
                if post.frontmatter.draft == Some(true) && matches!(&BLAIR.running_mode, RunningMode::Production) { _404Error!() }
                else {
                    return match template.render() {
                        Ok(render) => {
                            if !uri.ends_with("/") {
                                return HttpResponse::MovedPermanently().append_header(("Location", format!("/blog{}/", normal_uri))).finish()
                            }
                            HttpResponse::Ok().content_type("text/html").body(render)
                        },
                        Err(e) => HttpResponse::InternalServerError().body(crate::renderer::errors::askama_error(e))
                    }
                }
            },
            None => _404Error!()
        }
    }
}

// Favicon
#[get("/favicon{ext}")]
pub async fn favicon(req: HttpRequest) -> actix_web::Result<actix_files::NamedFile> {
    let ext = req.match_info().get("ext").unwrap_or("");
    match ext {
        ".ico" => Ok(actix_files::NamedFile::open("./static/favicon.ico")?),
        ".png" => Ok(actix_files::NamedFile::open("./static/favicon.png")?),
        _ => Err(ErrorNotFound(errors::bad_favicon(req)))
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
