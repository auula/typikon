// 使用 book_cli::lib 以导入被测试模块
use std::path::Path;

#[test]
fn test_pages_parser_html() {
    let path = Path::new("tests/chapter_1.1.0.md");

    // Loading a Markdown file with the from_markdown function
    let md = typikon::pargs::from_markdown(path);
    if let Err(err) = &md {
        eprintln!("{}", err);
        assert!(false); // Fail the test explicitly on error
        return;
    }

    // Create Document file with the create_document function
    let hypertext = md.as_ref().unwrap().to_html();
    match typikon::pargs::create_document(&hypertext, Path::new("tests/test.html")) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
            assert!(false); // Fail the test explicitly on error
        }
    }

    println!("{}", hypertext.as_str()); // Print the HTML content
}
