use typikon::utils::Logger;

#[test]
fn test_logger() {
    let mut log = Logger::console_log();

    log.info(format_args!("This is {} level message.", "INFO"));
    log.error(format_args!("This is {} level message.", "ERROR"));
    log.warn(format_args!("This is {} level message.", "WARING"));

    assert!(true)
}
