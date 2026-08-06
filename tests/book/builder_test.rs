use crate::common::ProjectFixture;
use std::fs;
use typikon::book::{new_builder_at, Document};

#[test]
fn builds_nested_pages_with_base_url_and_stable_search_data() {
    let fixture = ProjectFixture::new("/manual");
    let mut builder = new_builder_at(fixture.path()).expect("create builder");

    builder.generate_books().expect("build fixture");
    let output = fixture.path().join("site");
    assert!(output.join("index.html").is_file());
    assert!(output.join("chapter/index.html").is_file());
    assert!(output.join("chapter/nested/child.html").is_file());
    assert!(output.join("assets/style.css").is_file());

    let index_html = fs::read_to_string(output.join("index.html")).expect("read index HTML");
    assert!(index_html.contains("href=\"&#x2F;manual/index.html\""));
    assert!(index_html.contains("content=\"Test Author\""));
    assert!(index_html.contains("href=\"&#x2F;manual&#x2F;assets&#x2F;style.css\""));

    let first_json = fs::read_to_string(output.join("data.json")).expect("read data.json");
    let documents: Vec<Document> = serde_json::from_str(&first_json).expect("parse data.json");
    let urls: Vec<&str> = documents
        .iter()
        .map(|document| document.url.as_str())
        .collect();
    assert_eq!(
        urls,
        [
            "/manual/index.html",
            "/manual/chapter/index.html",
            "/manual/chapter/nested/child.html"
        ]
    );
    assert!(documents[2].content.contains("Searchable child text"));
    assert!(!documents[2].content.contains("<h1>"));

    // Reusing a Builder must reset IDs and search entries instead of duplicating them.
    builder.generate_books().expect("rebuild fixture");
    let second_json = fs::read_to_string(output.join("data.json")).expect("read rebuilt data");
    assert_eq!(first_json, second_json);
}

#[test]
fn preserves_previous_output_when_a_rebuild_fails() {
    let fixture = ProjectFixture::new("");
    let mut builder = new_builder_at(fixture.path()).expect("create builder");
    builder.generate_books().expect("initial build");
    let published_child = fixture.path().join("site/chapter/nested/child.html");
    let published_content = fs::read_to_string(&published_child).expect("read published page");

    fs::remove_file(fixture.path().join("book/nested/child.md")).expect("remove source file");
    assert!(builder.generate_books().is_err());
    assert_eq!(
        fs::read_to_string(published_child).expect("old page remains available"),
        published_content
    );
}

#[test]
fn rejects_output_paths_outside_the_project() {
    let fixture = ProjectFixture::new("");
    fixture.write_settings("..", "");

    let error = new_builder_at(fixture.path()).expect_err("unsafe output must fail");
    assert!(error.to_string().contains("project-relative"));
}

#[test]
fn rejects_duplicate_output_routes() {
    let fixture = ProjectFixture::new("");
    fixture.write_root(
        r#"path: index.md
chapters:
  - title: Chapter
    path: chapter.md
    sub_chapters:
      - title: Duplicate
        path: index.md
"#,
    );

    let error = new_builder_at(fixture.path()).expect_err("duplicate route must fail");
    assert!(error.to_string().contains("same output path"));
}
