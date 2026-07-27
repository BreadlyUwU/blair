use crate::config::BLAIR;
use actix_web::{App, HttpServer, dev::Server, middleware, web};
use std::net::TcpListener;
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};

use crate::renderer::pages::YLMode;

lazy_static! {
    pub static ref PAGE_COLLECTION: Arc<RwLock<Vec<crate::renderer::pages::Page>>> = 
        Arc::new(
            RwLock::new(
                crate::renderer::pages::yooka_laylee(YLMode::Page)
            )
        );
    pub static ref BLOG_COLLECTION: Arc<RwLock<Vec<crate::renderer::pages::Page>>> = 
        Arc::new(
            RwLock::new(
                crate::renderer::pages::yooka_laylee(YLMode::Blog)
            )
        );

    pub static ref BLAIR_CONFIG: crate::config::Configuration = crate::config::init();
}

pub fn launch() -> Result<Server, std::io::Error> {
    // let listener = TcpListener::bind(config::Configuration::get_listener())?;
    // let listener: Result<TcpListener, std::io::Error> = TcpListener::bind(format!("{}:{}", config.listening_ip, config.listening_port));
    match TcpListener::bind(format!("{}:{}", BLAIR.listening_ip, BLAIR.listening_port)) {
        Ok(listener) => {
            let server = HttpServer::new(move || {
                App::new()
                    .wrap(middleware::Logger::default())
                    // ↓ Désactivé car foutait la merde avec la redirection automatique
                    // .wrap(middleware::NormalizePath::new(
                    //     middleware::TrailingSlash::Trim,
                    // ))

                    // ATTENTION: Mettre les points d'entrés statiques EN PREMIER
                    // Sinon ils sont écrasés par les endpoints dynamiques
                    // (typiquement "pages" et "blog-posts")

                    // Service du health-check
                    .service(crate::health::check)

                    // Service de la favicon (eh wi)
                    .service(crate::renderer::favicon)

                    // Service des fichiers statiques
                    .service(
                        actix_files::Files::new("/static", "static/")
                            //.show_files_listing()
                            .use_last_modified(true),
                    )

                    // Service de la page d'accueil
                    .service(crate::renderer::home)

                    // Service des articles de blog
                    .service(crate::renderer::blog_posts)

                    // Service des pages potentielles
                    .service(crate::renderer::page)

                    //.Error 404
                    .default_service(web::route().to(crate::renderer::_404))
            })
            .listen(listener)?
            .run();

            Ok(server)
        }
        Err(e) => {
            eprintln!("Cannot create a listener.\n{}", e);
            std::process::exit(1);
        }
    }
}
