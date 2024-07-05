use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, ErrorKind},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub port: u16,
    pub icon: String,
    pub theme: String,
    pub title: String,
    pub author: String,
    pub language: String,
    pub keywords: String,
    pub description: String,
    pub directory: Directory,
    pub custom_js: Vec<String>,
    pub custom_css: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Directory {
    pub theme: String,
    pub input: String,
    pub output: String,
}

impl Settings {
    pub fn get_input_path(&self) -> &String {
        &self.directory.input
    }

    pub fn get_output_path(&self) -> &String {
        &self.directory.output
    }

    pub fn get_theme_path(&self) -> &String {
        &self.directory.theme
    }

    pub fn get_theme_name(&self) -> &String {
        &self.theme
    }
}

pub fn get_settings() -> io::Result<Settings> {
    let yaml_str = fs::read_to_string("settings.yml").map_err(|err| {
        io::Error::new(
            ErrorKind::InvalidData,
            format!("File not found settings.yml file: {:?}", err),
        )
    })?;
    let settings = serde_yaml::from_str(&yaml_str).map_err(|err| {
        io::Error::new(
            ErrorKind::InvalidData,
            format!("Failed to parse settings.yml file: {:?}", err),
        )
    })?;
    Ok(settings)
}

pub fn get_settings_from_file(file_path: &str) -> io::Result<Settings> {
    let yaml_str = fs::read_to_string(file_path).map_err(|err| {
        io::Error::new(
            ErrorKind::InvalidData,
            format!("File not found {}: {:?}", file_path, err),
        )
    })?;
    let settings = serde_yaml::from_str(&yaml_str).map_err(|err| {
        io::Error::new(
            ErrorKind::InvalidData,
            format!("Failed to parse {}: {:?}", file_path, err),
        )
    })?;
    Ok(settings)
}
