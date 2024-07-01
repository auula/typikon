use typikon::book;

#[test]
fn test_new_builder() {
    match book::new_builder() {
        Ok(builder) => {
            for view in builder.engine.get_template_names() {
                println!("{:?}",view);
            }
            assert!(true);
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn test_new_builder_get_hypertext() {
    match book::new_builder() {
        Ok(mut builder) => {
            println!("{:?}", &mut builder.get_chapters_hypertext());
            assert!(true);
        }
        Err(_) => assert!(false),
    }
}
