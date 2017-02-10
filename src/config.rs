extern crate xdg_basedir;
extern crate toml;

use std::fs;
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub host: HostConfig,
    pub auth: AuthConfig
}

impl Default for Config { fn default() -> Config { Config {
            host: HostConfig {
                hostname: String::from("http://example.com")
            },
            auth: AuthConfig {
                enabled: false,
                username: String::from("username"),
                password: String::from("password")
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HostConfig {
    pub hostname: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthConfig {
    pub enabled: bool,
    pub username: String,
    pub password: String
}

pub fn read() -> Config {
    let mut config_home = xdg_basedir::get_config_home()
        .expect("Could not find config directory from XDG!");
    config_home.push("jira-cli");

    debug!("[Config] Found config directory: {:?}", config_home);

    let mut config_file = config_home.clone();
    config_file.push("config");
    config_file.set_extension("toml");

    debug!("[Config] Loading config file: {:?}", config_file);

    let mut file = File::open(&config_file).unwrap();
    let mut config = String::new();

    file.read_to_string(&mut config)
        .expect("Could not read configuration file!");

    toml::from_str(&*config)
        .expect("Could not parse configuration file!")
}

pub fn write_defaults() {
    let mut config_home = xdg_basedir::get_config_home()
        .expect("Could not find config directory from XDG!");
    config_home.push("jira-cli");

    fs::create_dir_all(&config_home)
        .expect("Failed to create directory for config!");

    let mut config_file = xdg_basedir::get_config_home()
        .expect("Could not find config directory from XDG!");
    config_file.push("jira-cli");
    config_file.push("config");
    config_file.set_extension("toml");

    let defaults: Config = Default::default();
    let config = toml::to_string(&defaults)
        .expect("Failed to serialize default config!");

    let mut file = File::create(config_file)
        .expect("Failed to open config file!");

    write!(file, "{}", config)
        .expect("Failed to write to config file!");
}
