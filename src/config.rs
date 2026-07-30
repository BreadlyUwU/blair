use std::{net::Ipv4Addr, sync::LazyLock};
use chrono::prelude::*;

pub struct Configuration {
    // site_name: Option<String>,
    pub running_mode: RunningMode,
    pub site_addr: String,
    pub listening_ip: Ipv4Addr,
    pub listening_port: u16,
    pub current_base_url: String,
    pub compile_time: DateTime<Local>,
}

pub enum RunningMode {
    Production,
    Debug,
}

pub fn init() -> Configuration {
    #[cfg(debug_assertions)]
    let running_mode: RunningMode = RunningMode::Debug;
    #[cfg(not(debug_assertions))]
    let running_mode = RunningMode::Production;

    let site_addr: String = String::from("https://breadcat.run");
    let listening_ip: Ipv4Addr = Ipv4Addr::new(127,0,0,1);
    let listening_port: u16 = 8064;

    let current_base_url: String = match running_mode {
        RunningMode::Production => String::from(&site_addr),
        RunningMode::Debug => format!("http://{}:{}",
            &listening_ip,
            &listening_port
        ),
    };

    let compile_time: DateTime<Local> = chrono::Local::now();

    return Configuration { running_mode, site_addr, listening_ip, listening_port, current_base_url, compile_time }
}

pub static BLAIR: LazyLock<Configuration> = LazyLock::new(|| crate::config::init());
