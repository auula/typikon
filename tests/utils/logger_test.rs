use typikon::utils::Logger;

#[test]
fn test_logger() {
    //
    let mut log = Logger::console_log();

    log.info(format_args!("This is {} level message.\n","INFO"));
    log.error(format_args!("This is {} level message.\n","ERROR"));
    log.warn(format_args!("This is {} level message.\n","WARING"));

    assert!(true)
}
