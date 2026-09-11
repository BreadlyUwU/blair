mod config;
mod server;
mod health;
mod renderer;
use config::BLAIR;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if &args.len() > &1 {
        match args[1].as_str() {
            "monitor" => println!("Congratulation!"),
            "status" => println!("Wow! You're a, \"real\", \"hacker\"! BeWaRe!"),
            "🥺" => println!("What the fuck is 🥺\nuse your words\nI don't speak bottom"),
            _ => cli_args(),
        } Ok(())
    } else {
        blair()
    }
}

#[actix_web::main]
async fn blair() -> std::io::Result<()> {
    println!("\x1b[1m\x1b[33mB-LAIR - The entry point to the Breadcat's Lair\x1b[0m");

    #[cfg(debug_assertions)]
    unsafe { std::env::set_var("RUST_LOG", "debug"); }

    println!(
        "- Version: \x1b[1m{}\x1b[0m\n- Mode: {}\n",
        env!("CARGO_PKG_VERSION"),
        match &BLAIR.running_mode {
            crate::config::RunningMode::Debug      => "\x1b[1m\x1b[31mDebug (°o°)\x1b[0m",
            crate::config::RunningMode::Production => "\x1b[1m\x1b[32mRelease (^_^)\x1b[0m",
        }
    );

    env_logger::init();
    println!("🐱 \x1b[1m\x1b[34mReady!\x1b[0m\n🌐 \x1b[1m{}\x1b[0m\n", &BLAIR.current_base_url);
    crate::server::launch()?.await?;
    Ok(())
}

fn cli_args() {
    println!("
B-LAIR:
    monitor\t- Open a TUI to have a glimpse of what B-LAIR is doing.
    status\t- Show the current status of B-LAIR (granted it has been launched).
")
}