use typikon::book;

#[test]
fn test_new_builder() {
    match book::new_builder() {
        Ok(builder) => {
            println!("{:?}", builder.engine);
            assert!(true);
        }
        Err(_) => assert!(false),
    }
}
