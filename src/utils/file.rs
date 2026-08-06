use reqwest::blocking::Client;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, copy, ErrorKind};
use std::path::{Component, Path, PathBuf};
use std::time::Duration;
use walkdir::WalkDir;
use zip::read::ZipArchive;

const TEMPLATE_ARCHIVE_URL: &str =
    "https://github.com/typikonbook/typikon-book-v3/archive/refs/heads/main.zip";
const DOWNLOAD_TIMEOUT: Duration = Duration::from_secs(60);

pub fn copy_file(from: &Path, to: &Path) -> io::Result<()> {
    if !to.exists() {
        fs::create_dir_all(to)?;
    }

    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(from).map_err(io::Error::other)?;
        let target_path = to.join(relative_path);

        if path.is_dir() {
            copy_file(&path, &target_path)?;
        } else if path.is_file() {
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&path, &target_path)?;
        }
    }

    Ok(())
}

pub fn remove_md_extension(filename: &str) -> String {
    let path = Path::new(filename);
    if path
        .extension()
        .and_then(OsStr::to_str)
        .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
    {
        return path.with_extension("").to_string_lossy().to_string();
    }

    filename.to_string()
}

pub fn validate_relative_path(path: &Path, field_name: &str) -> io::Result<PathBuf> {
    if path.as_os_str().is_empty() {
        return Err(invalid_input(format!("{field_name} cannot be empty")));
    }

    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(value) => normalized.push(value),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(invalid_input(format!(
                    "{field_name} must be a project-relative path without '..': {}",
                    path.display()
                )));
            }
        }
    }

    if normalized.as_os_str().is_empty() {
        return Err(invalid_input(format!(
            "{field_name} cannot point to the project root"
        )));
    }

    Ok(normalized)
}

pub fn markdown_route(path: &Path) -> io::Result<PathBuf> {
    let normalized = validate_relative_path(path, "Markdown path")?;
    let extension = normalized.extension().and_then(OsStr::to_str);
    if !extension.is_some_and(|value| value.eq_ignore_ascii_case("md")) {
        return Err(invalid_input(format!(
            "Markdown files must use the .md extension: {}",
            path.display()
        )));
    }

    let without_extension = normalized.with_extension("");
    let mut route = PathBuf::new();
    for component in without_extension.components() {
        let Component::Normal(value) = component else {
            continue;
        };
        let value = value.to_str().ok_or_else(|| {
            invalid_input(format!(
                "Markdown paths must be valid UTF-8: {}",
                path.display()
            ))
        })?;
        let route_component = value.replace(' ', "-").to_lowercase();
        if route_component.is_empty() {
            return Err(invalid_input(format!(
                "A Markdown path cannot produce an empty URL: {}",
                path.display()
            )));
        }
        route.push(route_component);
    }

    Ok(route)
}

pub fn normalize_base_url(base_url: &str) -> io::Result<String> {
    let base_url = base_url.trim();
    if base_url.is_empty() || base_url == "/" {
        return Ok(String::new());
    }
    if base_url.contains("://") || base_url.contains(['?', '#', '\\']) || base_url.starts_with("//")
    {
        return Err(invalid_input(format!(
            "base_url must be a site path prefix such as /docs: {base_url}"
        )));
    }

    let relative = base_url.trim_matches('/');
    let normalized = validate_relative_path(Path::new(relative), "base_url")?;
    let value = normalized
        .to_str()
        .ok_or_else(|| invalid_input(format!("base_url must be valid UTF-8: {base_url}")))?;

    Ok(format!("/{value}"))
}

pub fn site_url(base_url: &str, resource: &str) -> String {
    let resource = resource.trim();
    if resource.starts_with("http://")
        || resource.starts_with("https://")
        || resource.starts_with("//")
        || resource.starts_with("data:")
        || resource.starts_with('#')
    {
        return resource.to_string();
    }

    let resource = resource.trim_start_matches('/');
    if resource.is_empty() {
        return format!("{base_url}/");
    }

    format!("{base_url}/{resource}")
}

pub fn path_to_url(path: &Path) -> io::Result<String> {
    let mut components = Vec::new();
    for component in path.components() {
        let Component::Normal(value) = component else {
            return Err(invalid_input(format!(
                "The URL path contains an invalid component: {}",
                path.display()
            )));
        };
        components.push(
            value
                .to_str()
                .ok_or_else(|| invalid_input("URL paths must be valid UTF-8"))?,
        );
    }
    Ok(components.join("/"))
}

pub fn paths_overlap(left: &Path, right: &Path) -> bool {
    left.starts_with(right) || right.starts_with(left)
}

pub fn download_zip() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let zip_path = Path::new("repo.zip");
    let partial_path = Path::new("repo.zip.part");
    let client = Client::builder().timeout(DOWNLOAD_TIMEOUT).build()?;

    // Write into a partial file until the download completes to avoid using truncated archives.
    let mut response = client
        .get(TEMPLATE_ARCHIVE_URL)
        .send()?
        .error_for_status()?;
    let mut output = File::create(partial_path)?;
    copy(&mut response, &mut output)?;
    output.sync_all()?;
    fs::rename(partial_path, zip_path)?;

    extract_zip_safely(zip_path, Path::new("."))?;
    Ok(())
}

fn extract_zip_safely(zip_path: &Path, destination: &Path) -> io::Result<()> {
    let zip_file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(zip_file)?;

    for index in 0..archive.len() {
        let mut entry = archive.by_index(index)?;
        let relative_path = entry.enclosed_name().ok_or_else(|| {
            invalid_input(format!(
                "The ZIP archive contains an unsafe path: {}",
                entry.name()
            ))
        })?;
        let output_path = destination.join(relative_path);

        if entry.is_dir() {
            fs::create_dir_all(&output_path)?;
            continue;
        }

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut output = File::create(&output_path)?;
        copy(&mut entry, &mut output)?;
    }

    Ok(())
}

pub fn move_dir_contents(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    let entries = WalkDir::new(src)
        .min_depth(1)
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    // Check every conflict before moving files so initialization cannot partially overwrite data.
    for entry in &entries {
        let relative_path = entry.path().strip_prefix(src).map_err(io::Error::other)?;
        let destination = dst.join(relative_path);
        if destination.exists() {
            return Err(io::Error::new(
                ErrorKind::AlreadyExists,
                format!("Destination path already exists: {}", destination.display()),
            ));
        }
    }

    for entry in entries {
        let relative_path = entry.path().strip_prefix(src).map_err(io::Error::other)?;
        let destination = dst.join(relative_path);
        if entry.path().is_dir() {
            fs::create_dir_all(&destination)?;
        } else {
            if let Some(parent) = destination.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::rename(entry.path(), destination)?;
        }
    }

    Ok(())
}

pub fn delete_file(file_path: &Path) -> io::Result<()> {
    if file_path.is_file() {
        fs::remove_file(file_path)?;
    }
    Ok(())
}

pub fn delete_folder(folder_path: &Path) -> io::Result<()> {
    if folder_path.is_dir() {
        fs::remove_dir_all(folder_path)?;
    }
    Ok(())
}

pub fn remove_output_path_prefix(output_path: &str, file_path: &str) -> String {
    let output_path = Path::new(output_path);
    let file_path = Path::new(file_path);
    let canonical_output = output_path.canonicalize();
    let canonical_file = file_path.canonicalize();

    if let (Ok(output), Ok(file)) = (canonical_output, canonical_file) {
        if let Ok(relative) = file.strip_prefix(output) {
            return relative.to_string_lossy().to_string();
        }
        return file.to_string_lossy().to_string();
    }

    file_path.to_string_lossy().to_string()
}

pub fn invalid_input(message: impl Into<String>) -> io::Error {
    io::Error::new(ErrorKind::InvalidInput, message.into())
}

#[cfg(test)]
mod tests {
    use super::{markdown_route, normalize_base_url, path_to_url, site_url};
    use std::path::Path;

    #[test]
    fn route_conversion_is_deterministic() {
        let route = markdown_route(Path::new("API Reference/HTTP Client.MD"))
            .expect("convert Markdown route");
        assert_eq!(
            path_to_url(&route).expect("convert route to URL"),
            "api-reference/http-client"
        );
    }

    #[test]
    fn site_urls_preserve_external_resources() {
        assert_eq!(normalize_base_url("/guide/").expect("base URL"), "/guide");
        assert_eq!(site_url("/guide", "/assets/app.js"), "/guide/assets/app.js");
        assert_eq!(
            site_url("/guide", "https://cdn.example/app.js"),
            "https://cdn.example/app.js"
        );
    }
}
