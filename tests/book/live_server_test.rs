#[cfg(test)]
mod tests {
    use typikon::cli::commands::handle_live_serve_command;

    #[test]
    fn test_live_server() {
        assert_eq!(handle_live_serve_command(), ());
    }
}
