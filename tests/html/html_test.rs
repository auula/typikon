use std::fs;
use typikon::html::{from_markdown, Hypertext, Markdown};

#[test]
fn converts_markdown_features_to_html() {
    let markdown = Markdown::new(
        r#"# Heading

Paragraph with **bold text**.

| Name | Value |
|------|-------|
| A    | 1     |
"#,
    );
    let html = markdown.to_html();

    assert!(html.contains("<h1>Heading</h1>"));
    assert!(html.contains("<strong>bold text</strong>"));
    assert!(html.contains("<table>"));
}

#[test]
fn extracts_plain_text_for_search_index() {
    let markdown = Markdown::new("# Heading\n\nUse `cargo test` with **confidence**.");
    let plain_text = markdown.to_plain_text();

    assert!(plain_text.contains("Heading"));
    assert!(plain_text.contains("cargo test"));
    assert!(plain_text.contains("confidence"));
    assert!(!plain_text.contains("<h1>"));
    assert!(!plain_text.contains("**"));
}

#[test]
fn reads_markdown_and_wraps_it_as_hypertext() {
    let directory = tempfile::tempdir().expect("temporary Markdown directory");
    let markdown_path = directory.path().join("chapter.md");
    fs::write(&markdown_path, "## Chapter").expect("write Markdown file");

    let markdown = from_markdown(&markdown_path).expect("read Markdown file");
    let hypertext = Hypertext::new("Chapter", "chapter.md", markdown);
    assert_eq!(hypertext.title, "Chapter");
    assert_eq!(hypertext.to_html(), "<h2>Chapter</h2>\n");
}
