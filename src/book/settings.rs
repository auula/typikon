use std::fs;

use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct About {
    pub title: String,
    pub author: String,
    pub language: String,
    pub keywords: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Directory {
    pub theme: String,
    pub source: String,
    pub output: String,
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
    pub custom_css: Vec<String>,
    pub custom_js: Vec<String>,
}

pub fn get_settings() -> Result<Settings, Box<dyn Error>> {
    let content = fs::read_to_string("settings.yml")
        .map_err(|_| format!("The \"settings.yml\" mapping file was not found"))?;
    let settings: Settings = serde_yaml::from_str(&content)
        .map_err(|_| format!("The \"settings.yml\" content is not formatted properly"))?;
    Ok(settings)
}
