use std::path::Path;

#[test]
fn test_read_content(){
    let path = Path::new("tests/utils/test.txt");
    let read_content = typikon::utils::read_file_contents(path);
    println!("{:?}",read_content);
    assert_eq!("this is content.",read_content);
}