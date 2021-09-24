use serde::{Deserialize, Serialize};
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub title: String,
    pub cover: String,
    pub author: String,
    pub chapter: String,
    #[serde(default)]
    pub lang: String,
    #[serde(default)]
    pub subchapter: String,
    #[serde(default)]
    pub encoding: String,
    #[serde(default)]
    pub is_html_p: bool,
    pub file: String,
}

impl Config {
    pub fn from(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = std::fs::File::open(path)?;
        let mut config_str = String::new();
        file.read_to_string(&mut config_str)?;
        let mut config: Config = toml::from_str(&config_str)?;
        if config.lang.is_empty() {
            config.lang = String::from("en");
        }
        Ok(config)
    }
}
