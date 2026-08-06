use crate::utils::{invalid_input, normalize_base_url, paths_overlap, validate_relative_path};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

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
    pub highlight: Vec<String>,
    pub custom_js: Vec<String>,
    pub custom_css: Vec<String>,
    #[serde(default)]
    pub base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Directory {
    pub theme: String,
    pub input: String,
    pub output: String,
}

#[derive(Debug, Clone)]
pub struct ResolvedDirectories {
    pub project_root: PathBuf,
    pub input: PathBuf,
    pub output: PathBuf,
    pub theme_root: PathBuf,
    pub active_theme: PathBuf,
}

impl Settings {
    pub fn get_input_path(&self) -> &str {
        &self.directory.input
    }

    pub fn get_output_path(&self) -> &str {
        &self.directory.output
    }

    pub fn get_theme_path(&self) -> &str {
        &self.directory.theme
    }

    pub fn get_theme_name(&self) -> &str {
        &self.theme
    }

    pub fn normalized_base_url(&self) -> io::Result<String> {
        normalize_base_url(&self.base_url)
    }

    pub fn resolve_directories(&self, project_root: &Path) -> io::Result<ResolvedDirectories> {
        let project_root = project_root.canonicalize().map_err(|error| {
            io::Error::new(
                ErrorKind::NotFound,
                format!(
                    "Failed to resolve project directory {}: {error}",
                    project_root.display()
                ),
            )
        })?;
        let input_relative =
            validate_relative_path(Path::new(&self.directory.input), "directory.input")?;
        let output_relative =
            validate_relative_path(Path::new(&self.directory.output), "directory.output")?;
        let theme_relative =
            validate_relative_path(Path::new(&self.directory.theme), "directory.theme")?;
        let theme_name = validate_relative_path(Path::new(&self.theme), "theme")?;

        let input = resolve_existing_directory(&project_root, &input_relative, "Input directory")?;
        let theme_root =
            resolve_existing_directory(&project_root, &theme_relative, "Theme root directory")?;
        let active_theme =
            resolve_existing_directory(&theme_root, &theme_name, "Active theme directory")?;
        let output = resolve_output_directory(&project_root, &output_relative)?;

        if paths_overlap(&output, &input) || paths_overlap(&output, &theme_root) {
            return Err(invalid_input(
                "The output directory cannot overlap the input or theme directory",
            ));
        }

        self.normalized_base_url()?;
        Ok(ResolvedDirectories {
            project_root,
            input,
            output,
            theme_root,
            active_theme,
        })
    }
}

fn resolve_existing_directory(base: &Path, relative: &Path, label: &str) -> io::Result<PathBuf> {
    let path = base.join(relative);
    let canonical = path.canonicalize().map_err(|error| {
        io::Error::new(
            ErrorKind::NotFound,
            format!("{label} does not exist {}: {error}", path.display()),
        )
    })?;
    if !canonical.starts_with(base) || !canonical.is_dir() {
        return Err(invalid_input(format!(
            "{label} must be a directory inside the project: {}",
            path.display()
        )));
    }
    Ok(canonical)
}

fn resolve_output_directory(project_root: &Path, relative: &Path) -> io::Result<PathBuf> {
    let mut output = project_root.to_path_buf();
    // Inspect every existing component so a parent symlink cannot redirect a new output path.
    for component in relative.components() {
        output.push(component.as_os_str());
        match fs::symlink_metadata(&output) {
            Ok(metadata) => {
                if metadata.file_type().is_symlink() {
                    return Err(invalid_input(format!(
                        "The output path cannot contain symbolic links: {}",
                        output.display()
                    )));
                }
                if !metadata.is_dir() {
                    return Err(invalid_input(format!(
                        "The output path must be a directory: {}",
                        output.display()
                    )));
                }
            }
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
    }

    Ok(output)
}

pub fn get_settings() -> io::Result<Settings> {
    get_settings_from_file("settings.yml")
}

pub fn get_settings_from_file(file_path: impl AsRef<Path>) -> io::Result<Settings> {
    let file_path = file_path.as_ref();
    let yaml_str = fs::read_to_string(file_path).map_err(|error| {
        io::Error::new(
            ErrorKind::NotFound,
            format!(
                "Failed to read settings file {}: {error}",
                file_path.display()
            ),
        )
    })?;
    serde_yaml::from_str(&yaml_str).map_err(|error| {
        io::Error::new(
            ErrorKind::InvalidData,
            format!(
                "Failed to parse settings file {}: {error}",
                file_path.display()
            ),
        )
    })
}
