use crate::config::BLAIR;
use actix_web::{App, HttpServer, dev::Server, middleware, web};
use std::net::TcpListener;

pub fn launch() -> Result<Server, std::io::Error> {
    // let listener = TcpListener::bind(config::Configuration::get_listener())?;
    // let listener: Result<TcpListener, std::io::Error> = TcpListener::bind(format!("{}:{}", config.listening_ip, config.listening_port));
    match TcpListener::bind(format!("{}:{}", BLAIR.listening_ip, BLAIR.listening_port)) {
        Ok(listener) => {
            let server = HttpServer::new(move || {
                App::new()
                    .wrap(middleware::Logger::default())
                    // ATTENTION: Mettre les points d'entrés statiques EN PREMIER
                    // Sinon ils sont écrasés par les endpoints dynamiques (typiquement "pages" et "blog-posts")

                    // Service du health-check
                    .service(crate::health::check)

                    // Service de la favicon (eh wi)
                    .service(crate::renderer::favicon)

                    // Service des fichiers statiques
                    .service(
                        actix_files::Files::new("/static", "./content/static/")
                            //.show_files_listing()
                            .use_last_modified(true),
                    )

                    // Service du flux Atom du blog
                    .service(crate::renderer::atom::feed)

                    // Service du blog
                    .service(crate::renderer::blog)

                    // Service du root (sert la homepage et les… p-a-g-i-e-s 📃📃📃)
                    .service(crate::renderer::root)

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
