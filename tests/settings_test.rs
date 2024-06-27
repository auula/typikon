use std::{fs::File, io::Read};


#[test]
fn test_settings(){
    // 指定 YAML 文件路径
    let yaml_file_path = "/Users/dings/js_workspace/typikon/settings.yaml";

    // 打开文件并读取内容到字符串
    let mut file = File::open(yaml_file_path).expect("Failed to open file");
    let mut yaml_content = String::new();
    file.read_to_string(&mut yaml_content).expect("Failed to read file");


    // 使用 serde_yaml 反序列化 YAML 到 Book 结构体
    let s: typikon::settings::Settings = match serde_yaml::from_str(&yaml_content) {
        Ok(book) => book,
        Err(err) => {
            panic!("Failed to deserialize YAML: {}", err);
        }
    };

    // 打印反序列化后的 Book 结果
    println!("{:#?}", s.settings.book);
}