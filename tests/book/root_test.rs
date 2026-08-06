use crate::common::{default_root, ProjectFixture};
use std::fs;
use typikon::book::get_root_from_file;

#[test]
fn parses_ordered_chapters_and_nested_paths() {
    let fixture = ProjectFixture::new("");
    fixture.write_root(default_root());
    let root = get_root_from_file(fixture.path().join("root.yml")).expect("parse root.yml");

    assert_eq!(root.path, "index.md");
    assert_eq!(root.chapters.len(), 1);
    assert_eq!(root.chapters[0].title, "Chapter");
    assert_eq!(root.chapters[0].sub_chapters[0].path, "nested/child.md");
}

#[test]
fn reports_invalid_root_yaml() {
    let fixture = ProjectFixture::new("");
    fs::write(fixture.path().join("root.yml"), "chapters: [").expect("write invalid root.yml");

    let error =
        get_root_from_file(fixture.path().join("root.yml")).expect_err("invalid YAML must fail");
    assert!(error.to_string().contains("Failed to parse root file"));
}
