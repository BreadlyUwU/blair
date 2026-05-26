use actix_web::{App, HttpServer, dev::Server, middleware, web};
use std::net::TcpListener;

pub fn launch(bconf: &crate::config::Configuration) -> Result<Server, std::io::Error> {
    // let listener = TcpListener::bind(config::Configuration::get_listener())?;
    // let listener: Result<TcpListener, std::io::Error> = TcpListener::bind(format!("{}:{}", config.listening_ip, config.listening_port));
    match TcpListener::bind(format!("{}:{}", bconf.listening_ip, bconf.listening_port)) {
        Ok(listener) => {
            let server = HttpServer::new(move || {
                App::new()
                    .wrap(middleware::Logger::default())
                    .wrap(middleware::NormalizePath::new(
                        middleware::TrailingSlash::Trim,
                    ))
                    .service(
                        actix_files::Files::new("/static", "static/")
                            //.show_files_listing()
                            .use_last_modified(true),
                    )
                    .service(crate::health::health_check_endpoint)
                    .service(crate::renderer::base::home)
                    //.Error 404
                    .default_service(web::route().to(crate::renderer::base::_404))
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
