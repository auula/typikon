use colored::Colorize;

pub const BANNER: &str = r"
  _             _ _
 | |_ _  _ _ __(_) |_____ _ _
 |  _| || | '_ \ | / / _ \ ' \
  \__|\_, | .__/_|_\_\___/_||_|
      |__/|_|     v{}

Typikon lets you use markdown to write your online books.
GitHub: http://typikonbook.github.io  License: Apache2.0
";

const HELP_INFO: &str = r"
Usage: typikon <command> [<args>...]

The commands are:

    init      Initialize to working directory
    build     Builder static html file and output to book

Use typikon help <command> for more information about a command.
";

const VERSION: &str = "0.1.1-beta";

pub fn print_banner_help() {
    println!(
        "{}\n{}",
        BANNER.replace("{}", VERSION).as_str().purple(),
        HELP_INFO.purple()
    )
}

pub fn print_banner() {
    println!("{}", BANNER.replace("{}", VERSION).as_str().purple())
}
