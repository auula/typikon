use crate::common::ProjectFixture;
use std::fs;
use typikon::book::get_settings_from_file;

#[test]
fn defaults_base_url_for_legacy_settings() {
    let fixture = ProjectFixture::new("");
    let settings_path = fixture.path().join("settings.yml");
    let settings = fs::read_to_string(&settings_path)
        .expect("read settings")
        .replace("base_url: \"\"\n", "");
    fs::write(&settings_path, settings).expect("write legacy settings");

    let settings = get_settings_from_file(settings_path).expect("parse legacy settings");
    assert_eq!(settings.base_url, "");
    assert_eq!(settings.normalized_base_url().expect("normalize URL"), "");
}

#[test]
fn rejects_output_that_overlaps_source_directories() {
    let fixture = ProjectFixture::new("");
    fixture.write_settings("book/generated", "");
    let settings =
        get_settings_from_file(fixture.path().join("settings.yml")).expect("parse settings");

    let error = settings
        .resolve_directories(fixture.path())
        .expect_err("overlap must fail");
    assert!(error.to_string().contains("cannot overlap"));
}

#[test]
fn normalizes_project_page_base_url() {
    let fixture = ProjectFixture::new("manual/");
    let settings =
        get_settings_from_file(fixture.path().join("settings.yml")).expect("parse settings");
    assert_eq!(
        settings.normalized_base_url().expect("normalize base URL"),
        "/manual"
    );
}

#[cfg(unix)]
#[test]
fn rejects_symlinks_in_output_parent_paths() {
    use std::os::unix::fs::symlink;

    let fixture = ProjectFixture::new("");
    let outside = tempfile::tempdir().expect("outside directory");
    symlink(outside.path(), fixture.path().join("linked")).expect("create output symlink");
    fixture.write_settings("linked/site", "");
    let settings =
        get_settings_from_file(fixture.path().join("settings.yml")).expect("parse settings");

    let error = settings
        .resolve_directories(fixture.path())
        .expect_err("output symlink must fail");
    assert!(error.to_string().contains("symbolic links"));
}
