use typikon::book;

#[test]
fn test_new_builder() {
    let builder = book::new_builder();
    assert!(
        builder.is_ok(),
        "Failed to create builder: {:?}",
        builder.err()
    );
    let builder = builder.unwrap();
    println!("{:?}", builder);
}
