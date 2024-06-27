use std::{fs, path::Path};
use typikon::settings;

#[test]
fn test_settings_serialize() {
    let path = Path::new("settings.yml");

    match fs::read_to_string(path) {
        Ok(content) => {
            let settings: settings::Settings =
                serde_yaml::from_str(&content).expect("Failed to serialize");
            // 打印序列化后的 YAML 字符串
            println!("{:?}", settings);
            assert!(true)
        },
        Err(_) => assert!(false),
    };

}
