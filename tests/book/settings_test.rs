use typikon::book;

#[test]
fn test_settings_serialize() {
    match book::Settings::new() {
        Some(settings) => {
            println!("{:?}", settings);
            assert!(true);
        }
        None => assert!(false),
    };
}

#[test]
fn test_get_settings() {
    match book::get_settings() {
        Ok(settings) => {
            println!("{:?}", settings);
            assert!(true);
        }
        Err(_) => assert!(false),
    };
}
