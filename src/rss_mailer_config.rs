use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub smtp: SmtpConfig,
    pub rss_mail: RssMailConfig,
}

#[derive(Debug, Deserialize)]
pub struct SmtpConfig {
    pub server: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RssMailConfig {
    pub to: Vec<String>,
    pub from: Option<String>,
}

pub fn read_from_file(file: &str) -> Config {
    let config_file = fs::read_to_string(file);
    let config: Config = toml::from_str(&config_file.unwrap()).unwrap();
    return config;
}
