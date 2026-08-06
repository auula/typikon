use std::fs;
use std::path::Path;
use tempfile::TempDir;

pub struct ProjectFixture {
    root: TempDir,
}

impl ProjectFixture {
    pub fn new(base_url: &str) -> Self {
        let root = tempfile::tempdir().expect("temporary project directory");
        let fixture = Self { root };
        fixture.create_directories();
        fixture.write_settings("site", base_url);
        fixture.write_root(default_root());
        fixture.write_book_files();
        fixture.write_theme();
        fixture
    }

    pub fn path(&self) -> &Path {
        self.root.path()
    }

    pub fn write_settings(&self, output: &str, base_url: &str) {
        let yaml = format!(
            r#"title: Test Book
author: Test Author
description: Test description
language: en
base_url: "{base_url}"
keywords: test,book
directory:
  theme: themes
  input: book
  output: {output}
port: 0
theme: test-theme
icon: /assets/icon.png
custom_css:
  - /assets/style.css
custom_js: []
highlight: []
"#
        );
        fs::write(self.path().join("settings.yml"), yaml).expect("write settings.yml");
    }

    pub fn write_root(&self, yaml: &str) {
        fs::write(self.path().join("root.yml"), yaml).expect("write root.yml");
    }

    fn create_directories(&self) {
        fs::create_dir_all(self.path().join("book/nested")).expect("create book directory");
        fs::create_dir_all(self.path().join("themes/test-theme/assets"))
            .expect("create theme directory");
    }

    fn write_book_files(&self) {
        fs::write(self.path().join("book/index.md"), "# Home\nWelcome home.")
            .expect("write index markdown");
        fs::write(
            self.path().join("book/chapter.md"),
            "# Chapter\nChapter body.",
        )
        .expect("write chapter markdown");
        fs::write(
            self.path().join("book/nested/child.md"),
            "# Child\nSearchable child text.",
        )
        .expect("write child markdown");
    }

    fn write_theme(&self) {
        let template = r#"<!DOCTYPE html>
<html lang="{{ language }}">
<head>
  <meta name="author" content="{{ author }}">
  {% for css in custom_css %}<link rel="stylesheet" href="{{ css }}">{% endfor %}
</head>
<body>
  <a href="{{ base_url }}/index.html">{{ title }}</a>
  {% for chapter in chapters %}
  <a href="{{ base_url }}/{{ chapter.path }}/index.html">{{ chapter.title }}</a>
  {% for child in chapter.sub_chapters %}
  <a href="{{ base_url }}/{{ chapter.path }}/{{ child.path }}.html">{{ child.title }}</a>
  {% endfor %}
  {% endfor %}
  <main>{{ content | safe }}</main>
</body>
</html>
"#;
        fs::write(self.path().join("themes/test-theme/index.html"), template)
            .expect("write theme template");
        fs::write(
            self.path().join("themes/test-theme/assets/style.css"),
            "body { color: black; }",
        )
        .expect("write theme asset");
    }
}

pub fn default_root() -> &'static str {
    r#"path: index.md
chapters:
  - title: Chapter
    path: chapter.md
    sub_chapters:
      - title: Child
        path: nested/child.md
"#
}
