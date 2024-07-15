#[cfg(test)]
mod tests {
    use typikon::book::new_builder;

    #[test]
    fn test_generate_books() {
        // Arrange
        let mut builder = new_builder().unwrap();
        // Act
        let result = builder.generate_books();
        // Assert
        assert!(result.is_ok(), "Failed to generate books: {:?}", result.err());
    }
}
