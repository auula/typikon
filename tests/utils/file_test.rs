use std::fs;
use std::path::Path;
use typikon::utils::{
    markdown_route, move_dir_contents, normalize_base_url, path_to_url, site_url,
    validate_relative_path,
};

#[test]
fn validates_and_converts_cross_platform_routes() {
    assert!(validate_relative_path(Path::new("../outside"), "path").is_err());
    assert!(validate_relative_path(Path::new("/absolute"), "path").is_err());

    let route = markdown_route(Path::new("Guides/Quick Start.MD")).expect("convert route");
    assert_eq!(
        path_to_url(&route).expect("convert URL"),
        "guides/quick-start"
    );
}

#[test]
fn normalizes_base_urls_and_local_resources() {
    assert_eq!(normalize_base_url("").expect("empty base URL"), "");
    assert_eq!(normalize_base_url("manual/").expect("base URL"), "/manual");
    assert!(normalize_base_url("../manual").is_err());
    assert_eq!(
        site_url("/manual", "/assets/app.js"),
        "/manual/assets/app.js"
    );
    assert_eq!(
        site_url("/manual", "https://cdn.example/app.js"),
        "https://cdn.example/app.js"
    );
}

#[test]
fn refuses_to_overwrite_existing_files_during_move() {
    let directory = tempfile::tempdir().expect("temporary move directory");
    let source = directory.path().join("source");
    let destination = directory.path().join("destination");
    fs::create_dir_all(&source).expect("create source");
    fs::create_dir_all(&destination).expect("create destination");
    fs::write(source.join("settings.yml"), "new").expect("write source file");
    fs::write(destination.join("settings.yml"), "existing").expect("write destination file");

    assert!(move_dir_contents(&source, &destination).is_err());
    assert_eq!(
        fs::read_to_string(destination.join("settings.yml")).expect("read destination"),
        "existing"
    );
    assert!(source.join("settings.yml").is_file());
}
