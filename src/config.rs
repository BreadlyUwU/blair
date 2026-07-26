use std::net::Ipv4Addr;
use lazy_static::lazy_static;

pub struct Configuration {
    // site_name: Option<String>,
    pub running_mode: RunningMode,
    pub site_addr: String,
    pub listening_ip: Ipv4Addr,
    pub listening_port: u16,
    pub current_base_url: String,
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

    let site_url: String = String::from("https://breadcat.run");
    let listening_ip: Ipv4Addr = Ipv4Addr::new(127,0,0,1);
    let listening_port: u16 = 8064;

    let current_base_url: String = match running_mode {
        RunningMode::Production => String::from(&site_url),
        RunningMode::Debug => format!("http://{}:{}",
            &listening_ip,
            &listening_port
        ),
    };

    return Configuration { running_mode, site_addr: site_url, listening_ip, listening_port, current_base_url }
}

lazy_static! {
    pub static ref BLAIR: Configuration = crate::config::init();
}