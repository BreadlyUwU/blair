use std::net::Ipv4Addr;

#[allow(dead_code)]
pub struct Configuration {
    // site_name: Option<String>,
    site_addr: Option<String>,
    blair_listening_ip: Option<Ipv4Addr>,
    blair_listening_port: Option<u16>,
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
            // site_name: Some(String::from("Breadcat's Lair")), 
            site_addr: Some(String::from("https://breadcat.run")), 
            blair_listening_ip: Some(Ipv4Addr::new(127, 0, 0, 1)), 
            blair_listening_port: Some(8064) 
        }
    }
    
    pub fn get_listener() -> String {
        let config: Configuration = Configuration::default_config();
        return format!("{}:{}", config.blair_listening_ip.unwrap(), config.blair_listening_port.unwrap())
    }

    pub fn base_url(mode: RunningMode) -> String {
        let config: Configuration = Configuration::default_config();
        match mode {
            RunningMode::Production => return config.site_addr.unwrap(),
            RunningMode::Debug => return format!("{}:{}", 
                config.blair_listening_ip.unwrap(), 
                config.blair_listening_port.unwrap()
            ),
        }
    }
}