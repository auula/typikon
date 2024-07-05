use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::fs;

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

pub fn get_settings() -> anyhow::Result<Settings> {
    let content = fs::read_to_string("settings.yml").map_err(|_| Error::RootFileNotFound)?;
    let settings: Settings =
        serde_yaml::from_str(&content).map_err(|_| Error::RootFileContentNotFormatted)?;
    Ok(settings)
}
