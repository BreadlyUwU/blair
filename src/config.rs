use std::{net::Ipv4Addr, path::PathBuf, str::FromStr, sync::LazyLock};
use chrono::prelude::*;
use serde::Deserialize;

const CONFIG_FILE_NAME: &str = "blair.yml";
const CONFIG_FILE_LOCATION: &str = "../";

pub struct Configuration {
    pub website_name: String,
    pub running_mode: RunningMode,
    pub listening_ip: Ipv4Addr,
    pub listening_port: u16,
    pub current_base_url: String,
    pub compile_time: DateTime<Local>,
    pub extra: ConfigurationExtra,
}

#[derive(Deserialize)]
pub struct UserConfiguration {
    website_name: String,
    base_url: String,
    listening_ip: String,
    listening_port: u16,
    extra: ConfigurationExtra,
}

#[derive(Deserialize)]
pub struct ConfigurationExtra {
    pub fediverse: Option<Fediverse>
}

#[derive(Deserialize, Clone)]
pub struct Fediverse {
    pub name: String,
    pub url: String,
    pub health_endpoint: String,
}

#[derive(Deserialize)]
pub enum RunningMode {
    Production,
    Debug,
}

pub fn init() -> Configuration {
    let user_config_file = find_config_file();
    let user_config;
    match user_config_file {
        Ok(file) => user_config = read_config_file(file),
        Err(_) => {
            eprintln!("Cannot find 'blair.yml' in either the current directory nor it's parent one.");
            std::process::exit(1)
        },
    }

    let website_name = user_config.website_name;

    let running_mode: RunningMode = if cfg!(debug_assertions) { RunningMode::Debug }
                                    else                      { RunningMode::Production };

    let listening_ip: Ipv4Addr = Ipv4Addr::from_str(&user_config.listening_ip).unwrap();
    let listening_port: u16 = user_config.listening_port;

    let current_base_url: String = match running_mode {
        RunningMode::Production => String::from(&user_config.base_url),
        RunningMode::Debug => format!("http://{}:{}",
            &listening_ip,
            &listening_port
        ),
    };

    let compile_time: DateTime<Local> = chrono::Local::now();

    let extra = user_config.extra;

    return Configuration { website_name, running_mode, listening_ip, listening_port, current_base_url, compile_time, extra }
}

fn read_config_file(file: PathBuf) -> UserConfiguration {
    match std::fs::read_to_string(&file) {
        Ok(config) => {
            match yaml_serde::from_str::<UserConfiguration>(config.as_str()) {
                Ok(parsed_config) => {
                    log::debug!("Configuration ({}) succesfully parsed.", &file.into_string().unwrap());
                    return parsed_config
                },
                Err(e) => {
                    eprintln!("\nThere's an error inside your current config file. Please fix it.\n{}: {:?}\n", file.display(), e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Cannot open the config file {}: {:?}", file.display(), e);
            std::process::exit(1);
        }
    }
}

fn find_config_file() -> Result<PathBuf, bool> {
    let config_file_etc = std::path::Path::new(&CONFIG_FILE_LOCATION).join(&CONFIG_FILE_NAME);
    match config_file_etc.exists() {
        true  => { return Ok(config_file_etc.to_path_buf()) },
        false => {
            let config_file_local = std::path::Path::new(&CONFIG_FILE_NAME);
            match config_file_local.exists() {
                true  => return Ok(config_file_local.to_path_buf()),
                false => return Err(false)
            }
        }
    }
}

pub static BLAIR: LazyLock<Configuration> = LazyLock::new(|| crate::config::init());
