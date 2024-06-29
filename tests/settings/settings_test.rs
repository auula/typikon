use std::{fs, path::Path};

use typikon::book::Settings;

#[test]
fn test_settings_serialize() {
    let path = Path::new("settings.yml");

    match fs::read_to_string(path) {
        Ok(content) => {
            let res: Result<Settings, serde_yaml::Error> = serde_yaml::from_str(&content);
            match res {
                Ok(settings) => println!("{:?}", settings),
                Err(_) => assert!(false),
            }
            assert!(true)
        }
        Err(_) => assert!(false),
    };
}
