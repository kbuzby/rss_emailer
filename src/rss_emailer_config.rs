use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub smtp: SmtpConfig,
    pub rss_mail: RssMailConfig,
    pub feeds: HashMap<String, FeedDefinition>,
}

#[derive(Debug, Deserialize)]
pub struct SmtpConfig {
    pub server: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RssMailTo {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct RssMailConfig {
    pub to: RssMailTo,
    pub from: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum FeedDefinition {
    Simple(String),
    Detailed(FeedDetail),
}

#[derive(Debug, Deserialize)]
pub struct FeedDetail {
    pub link: String,
    pub feed_type: FeedType,
}

#[derive(Debug, Deserialize)]
pub enum FeedType {
    Rss,
    Atom,
}

pub fn read_from_file(file: &str) -> Config {
    let config_file = fs::read_to_string(file);
    let config: Config = toml::from_str(&config_file.unwrap()).unwrap();
    return config;
}
