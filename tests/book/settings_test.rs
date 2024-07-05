#[cfg(test)]
mod tests {
    use std::fs;

    use typikon::book::get_settings_from_file;


    #[test]
    fn test_get_settings() {
        // Write a simple settings YAML string for testing
        let yaml_str = r#"
            port: 8080
            icon: "icon.png"
            theme: "default"
            title: "Test Title"
            author: "Test Author"
            language: "English"
            keywords: "test, example"
            description: "Testing settings"
            directory:
                theme: "themes/default"
                input: "input/"
                output: "output/"
            custom_js:
                - "custom.js"
            custom_css:
                - "custom.css"
        "#;

        // Write the YAML string to a temporary file for the test
        let temp_file_path = "test_settings.yml";
        fs::write(temp_file_path, yaml_str).expect("Failed to write temporary file");

        // Attempt to get settings from the temporary file
        let result = get_settings_from_file(temp_file_path);

        // Remove the temporary file after the test
        fs::remove_file(temp_file_path).expect("Failed to remove temporary file");

        // Assert that the settings were retrieved successfully
        assert!(result.is_ok());
        let settings = result.unwrap();
        assert_eq!(settings.port, 8080);
        assert_eq!(settings.icon, "icon.png");
        assert_eq!(settings.theme, "default");
        assert_eq!(settings.title, "Test Title");
        assert_eq!(settings.author, "Test Author");
        assert_eq!(settings.language, "English");
        assert_eq!(settings.keywords, "test, example");
        assert_eq!(settings.description, "Testing settings");
        assert_eq!(settings.directory.theme, "themes/default");
        assert_eq!(settings.directory.input, "input/");
        assert_eq!(settings.directory.output, "output/");
        assert_eq!(settings.custom_js, vec!["custom.js"]);
        assert_eq!(settings.custom_css, vec!["custom.css"]);
    }
}
