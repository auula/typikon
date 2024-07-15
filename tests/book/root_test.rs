#[cfg(test)]
mod tests {
    use std::fs;

    use typikon::book::get_root_from_file;

    #[test]
    fn test_get_root() {
        // Write a simple root YAML string for testing
        let yaml_str = r#"
            path: "/book"
            chapters:
                - title: "Chapter 1"
                  path: "/book/chapter1"
                  sub_chapters:
                      - title: "SubChapter 1.1"
                        path: "/book/chapter1/sub1"
                      - title: "SubChapter 1.2"
                        path: "/book/chapter1/sub2"
                - title: "Chapter 2"
                  path: "/book/chapter2"
                  sub_chapters: []
        "#;

        // Write the YAML string to a temporary file for the test
        let temp_file_path = "test_root.yml";
        fs::write(temp_file_path, yaml_str).expect("Failed to write temporary file");

        // Attempt to get root from the temporary file
        let result = get_root_from_file(temp_file_path);

        // Remove the temporary file after the test
        fs::remove_file(temp_file_path).expect("Failed to remove temporary file");

        // Assert that the root was retrieved successfully
        assert!(result.is_ok());
        let root = result.unwrap();
        assert_eq!(root.path, "/book");
        assert_eq!(root.chapters.len(), 2);

        // Validate the first chapter and its sub-chapters
        let chapter1 = &root.chapters[0];
        assert_eq!(chapter1.title, "Chapter 1");
        assert_eq!(chapter1.path, "/book/chapter1");
        assert_eq!(chapter1.sub_chapters.len(), 2);
        assert_eq!(chapter1.sub_chapters[0].title, "SubChapter 1.1");
        assert_eq!(chapter1.sub_chapters[0].path, "/book/chapter1/sub1");
        assert_eq!(chapter1.sub_chapters[1].title, "SubChapter 1.2");
        assert_eq!(chapter1.sub_chapters[1].path, "/book/chapter1/sub2");

        // Validate the second chapter (which has no sub-chapters)
        let chapter2 = &root.chapters[1];
        assert_eq!(chapter2.title, "Chapter 2");
        assert_eq!(chapter2.path, "/book/chapter2");
        assert!(chapter2.sub_chapters.is_empty());
    }
}