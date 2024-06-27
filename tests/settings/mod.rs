use std::{fs, io, path::Path};

pub(crate) mod settings_test;

pub fn read_file_contents(path: &Path) -> Result<String, io::Error> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => return Err(err),
    };
    Ok(content)
}
