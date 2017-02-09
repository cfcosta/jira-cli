extern crate xdg_basedir;
extern crate toml;

use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub host: HostConfig
}

#[derive(Deserialize, Debug, Clone)]
pub struct HostConfig {
    pub hostname: String
}

pub fn read_config() -> Config {
    let mut config_home = xdg_basedir::get_config_home()
        .expect("Could not find config directory from XDG!");
    config_home.push("jira");

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
