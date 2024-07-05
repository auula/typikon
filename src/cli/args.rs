use std::{env, process::exit};

use colored::Colorize;

use super::Command;

const BANNER: &str = r"
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
    serve     Serve starting the static http server
    build     Builder static html file and output to book

Use typikon help <command> for more information about a command.
";

const VERSION: &str = "0.1.2";

pub fn output_banner_help() {
    println!(
        "{}\n{}",
        BANNER.replace("{}", VERSION).as_str().purple(),
        HELP_INFO.purple()
    )
}

pub fn ouput_banner() {
    println!("{}", BANNER.replace("{}", VERSION).as_str().purple())
}

// Analyze the parameters input in the command line,
pub fn parse_args() -> (Command, Vec<String>) {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        output_banner_help();
        exit(0)
    }

    let cmd = args[1].clone();
    let params = args[2..].to_vec();

    (Command::from_str(cmd.as_str()), params)
}
