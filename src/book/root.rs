use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    pub root: InnerRoot,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InnerRoot {
    pub title: String,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chapter {
    pub title: String,
    pub path: String,
    pub sub_chapters: Vec<SubChapter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubChapter {
    pub title: String,
    pub path: String,
}

pub fn get_root() -> Result<Root, Box<dyn Error>> {
    let path = Path::new("root.yml");
    let content = fs::read_to_string(path)?;
    let root: Root = serde_yaml::from_str(&content)?;
    Ok(root)
}
