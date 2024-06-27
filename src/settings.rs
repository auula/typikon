use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub description: String,
    pub language: String,
    pub theme: String,
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
    pub book: Book,
    pub highlight: Vec<String>,
    pub directory: Directory,
    pub deployment: Deployment,
    pub custom_css: Vec<String>,
    pub custom_js: Vec<String>,
}