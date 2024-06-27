use std::fs;
use std::path::Path;

pub fn read_file_contents(path: &Path) -> String {
    match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => String::new(),
    }
}
