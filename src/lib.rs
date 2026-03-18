mod config;
mod health;
mod renderer;

use actix_web::{
    App, 
    HttpServer, 
    dev::Server, 
    middleware
};
use std::net::TcpListener;

pub fn launch() -> Result<Server, std::io::Error> {
    let listener = TcpListener::bind(config::Configuration::get_listener())?;
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
            .service(health::health_check_endpoint)
            .service(renderer::base::home)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
