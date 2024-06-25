use typikon::console::Console;

#[test]
fn test_console_out() {
    let mut console = Console::new();

    console.info(format_args!("This is an info message."));
    console.warn(format_args!("This is an warn message."));
    console.error(format_args!("This is an error message."));

    let arg1 = 24;
    let arg2 = "Leon";
    console.info(format_args!("hi, i am {} age {}", arg2, arg1));
}
