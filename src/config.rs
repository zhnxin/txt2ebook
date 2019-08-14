use serde::{Deserialize, Serialize};
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    title: String,
    cover: String,
    author: String,
    chapter: String,
    #[serde(default)]
    lang: String,
    #[serde(default)]
    sub_chapter: String,
    #[serde(default)]
    encoding: String,
    #[serde(default)]
    is_html_p: bool,
    file: String,
}

impl Config {
    pub fn from(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = std::fs::File::open(path)?;
        let mut config_str = String::new();
        file.read_to_string(&mut config_str)?;
        let mut config: Config = toml::from_str(&config_str)?;
        if config.lang.is_empty(){
            config.lang = String::from("en");
        }
        Ok(config)
    }
    pub fn get_chapter(&self) -> &String {
        &self.chapter
    }
    pub fn get_subchapter(&self) -> &String {
        &self.sub_chapter
    }
    pub fn get_file(&self) -> &String {
        &self.file
    }
    pub fn get_title(&self) -> &String {
        &self.title
    }
    pub fn get_encoding(&self) -> &String {
        &self.encoding
    }
    pub fn get_is_html_p(&self) -> bool {
        self.is_html_p
    }
}
