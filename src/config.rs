use std::net::Ipv4Addr;

#[allow(dead_code)]
pub struct Configuration {
    // site_name: Option<String>,
    site_addr: Option<String>,
    blair_listening_ip: Option<Ipv4Addr>,
    blair_listening_port: Option<u16>,
    running_mode: Option<RunningMode>,
}

#[allow(dead_code)]
pub enum RunningMode {
    Production,
    Debug,
}

#[allow(dead_code)]
impl Configuration {
    fn default_config() -> Configuration {
        return Configuration { 
            site_addr: Some(String::from("https://staging.breadcat.run")),
            blair_listening_ip: Some(Ipv4Addr::new(127, 0, 0, 1)), 
            blair_listening_port: Some(8064),
            #[cfg(debug_assertions)]
            running_mode: Some(RunningMode::Debug),
            #[cfg(not(debug_assertions))]
            running_mode: Some(RunningMode::Production),
        }
    }
    
    pub fn get_listener() -> String {
        let config: Configuration = Configuration::default_config();
        return format!("{}:{}", config.blair_listening_ip.unwrap(), config.blair_listening_port.unwrap())
    }

    pub fn get_running_mode() -> RunningMode {
        let config: Configuration = Configuration::default_config();
        return config.running_mode.unwrap()
    }

    pub fn base_url() -> String {
        let config: Configuration = Configuration::default_config();
        match config.running_mode.unwrap() {
            RunningMode::Production => return config.site_addr.unwrap(),
            RunningMode::Debug => return format!("http://{}:{}", 
                config.blair_listening_ip.unwrap(), 
                config.blair_listening_port.unwrap()
            ),
        }
    }
}