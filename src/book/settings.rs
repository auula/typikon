use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct About {
    pub title: String,
    pub author: String,
    pub description: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Directory {
    pub theme: String,
    pub source: String,
    pub output: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deployment {
    pub repository: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub settings: InnerSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerSettings {
    pub about: About,
    pub theme: String,
    pub directory: Directory,
    pub deployment: Deployment,
    pub custom_css: Vec<String>,
    pub custom_js: Vec<String>,
}

impl Settings {
    pub fn new() -> Option<Settings> {
        match fs::read_to_string("settings.yml") {
            Ok(content) => match serde_yaml::from_str(&content) {
                Ok(settings) => Some(settings),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }
}
