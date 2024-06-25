use std::fs;

use typikon::book::{self, Root};

#[test]
fn test_book_root_serialize() {
    match fs::read_to_string("root.yml") {
        Ok(content) => {
            let res: Result<Root, serde_yaml::Error> = serde_yaml::from_str(&content);
            match res {
                Ok(roots) => {
                    println!("{:?}", roots);
                    assert!(true)
                }
                Err(_) => assert!(false),
            }
        }
        Err(_) => assert!(false),
    };
}

#[test]
fn test_book_get_root() {
    match book::get_root() {
        Ok(root) => {
            println!("{:?}", root);
            assert!(true)
        }
        Err(_) => assert!(false),
    }
}
