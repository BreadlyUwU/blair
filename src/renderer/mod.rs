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

pub fn is_potential_opt_file(uri: &str) -> bool { uri.contains(".") && !uri.ends_with(".md") }

macro_rules! blair_template {
    ($vis:vis struct $name:ident<$lt:lifetime> { $($fields:tt)* } , $path:literal $(,)?) => {
        #[allow(dead_code)]
        #[derive(Template)]
        #[template(path = $path)]
        $vis struct $name<$lt> {
            config: &$lt crate::config::Configuration,
            $($fields)*
        }
        functions::load_BlairStandardFunctions!($name<'_>);
    };
}

blair_template! { struct HomeTemplate<'a> { blog_last_post: &'a Page }, "home.html" }
blair_template! { struct PageTemplate<'a> { page: &'a Page }, "page.html" }
blair_template! { struct BlogTemplate<'a> { posts: &'a [Page] }, "blog.html" }
blair_template! { struct BlogPostTemplate<'a> { post: &'a Page }, "blog-post.html" }
blair_template! { pub struct Err404Template<'a> {}, "404.html" }

macro_rules! _404Error {
    () => {{
        let config = &BLAIR;
        let template: Err404Template = Err404Template { config };
        match template.render() {
            Ok(render) => HttpResponse::NotFound().content_type("text/html").body(render),
            Err(e) => HttpResponse::InternalServerError().body(crate::renderer::errors::askama_error(e)),
        }
    }};
}

// Serving the website's root, so the homepage and… the p-a-g-i-e-s 📃📃📃
#[get("/{uri:.*}")]
pub async fn root(req: HttpRequest) -> impl Responder {
    let uri = req.match_info().get("uri").unwrap_or("");
    let normal_uri = if uri.ends_with("/") { uri.split_at(uri.len() - 1).0 } else { uri };
    let config = &BLAIR;
    let pages = &PAGE_COLLECTION;
    let posts = &BLOG_COLLECTION;
    if uri.is_empty() || uri == "/" {
        // Homepage
        let blog_last_post = &posts[0];
        let template = HomeTemplate { blog_last_post, config };
        return match template.render() {
            Ok(render) => HttpResponse::Ok().content_type("text/html").body(render),
            Err(e) => HttpResponse::InternalServerError().body(crate::renderer::errors::askama_error(e)),
        }
    } else {
        // Page's static files
        if is_potential_opt_file(normal_uri) {
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
                    let template = PageTemplate { page, config };
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
    let config = &BLAIR;

    // Blog's homepage
    if uri.is_empty() || uri == "/" {
        let template = BlogTemplate { posts: &BLOG_COLLECTION, config };
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
    if is_potential_opt_file(normal_uri) {
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
                let template = BlogPostTemplate { post, config };
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
        ".ico" => Ok(actix_files::NamedFile::open("./content/static/favicon.ico")?),
        ".png" => Ok(actix_files::NamedFile::open("./content/static/favicon.png")?),
        _ => Err(ErrorNotFound(errors::bad_favicon(req)))
    }
}

// 404 Error Page
pub async fn _404() -> impl Responder {
    _404Error!()
}
