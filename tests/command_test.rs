use typikon::command;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_theme_command() {
        // Simulate command-line arguments
        let vec_strings = vec![
            "test".to_string(),
            "theme".to_string(),
            "--get=https://github.com/auula/typikon.git".to_string(),
        ];

        // Assert that the command identifier matches
        assert_eq!(command::THEME, "theme");

        // Call the function under test
        command::handle_theme_command(&vec_strings);
    }
}
