#[allow(dead_code)]
mod config;
mod server;
mod health;
mod renderer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("\x1b[1m\x1b[33mB-LAIR - The entry point to the Breadcat's Lair\x1b[0m");

    #[cfg(debug_assertions)]
    unsafe { std::env::set_var("RUST_LOG", "debug"); }

    let blair_config: crate::config::Configuration = crate::config::init();

    println!(
        "- Version: \x1b[1m{}\x1b[0m\n- Mode: {}\n",
        env!("CARGO_PKG_VERSION"),
        match &blair_config.running_mode {
            crate::config::RunningMode::Debug => "\x1b[1m\x1b[31mDebug (°o°)\x1b[0m",
            crate::config::RunningMode::Production => "\x1b[1m\x1b[32mRelease (^_^)\x1b[0m",
        }
    );

    env_logger::init();
    println!("\x1b[1m\x1b[34mReady! c:\nServing to: \x1b[0m{}\n", blair_config.current_base_url);
    crate::server::launch(&blair_config)?.await?;
    Ok(())
}
