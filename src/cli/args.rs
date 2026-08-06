use std::env;

use colored::Colorize;

use super::Command;

const BANNER: &str = r"
     _             _ _
    | |_ _  _ _ __(_) |_____ _ _
    |  _| || | '_ \ | / / _ \ ' \
     \__|\_, | .__/_|_\_\___/_||_|
        |__/|_|     v{}

    Typikon lets you use Markdown to write online books.
    Website: https://typikonbook.github.io  License: Apache-2.0
";

const HELP_INFO: &str = r"
    Usage: typikon <command>

    Commands:
        init      Initialize an empty working directory
        build     Build the static book
        serve     Serve the current output directory
        watch     Rebuild changed files and serve the output
        help      Show command help
        version   Show the installed version

    Use `typikon help <command>` for command-specific examples.
";

pub fn output_banner_help() {
    println!(
        "{}\n{}",
        BANNER.replace("{}", env!("CARGO_PKG_VERSION")).purple(),
        HELP_INFO.purple()
    );
}

pub fn output_banner() {
    println!(
        "{}",
        BANNER.replace("{}", env!("CARGO_PKG_VERSION")).purple()
    );
}

// Keep the misspelled public function as a compatibility alias for existing callers.
pub fn ouput_banner() {
    output_banner();
}

pub fn parse_args() -> (Command, Vec<String>) {
    parse_args_from(env::args().skip(1))
}

pub fn parse_args_from<I, S>(args: I) -> (Command, Vec<String>)
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut args = args.into_iter().map(Into::into);
    let command = args
        .next()
        .map(|value| Command::parse(&value))
        .unwrap_or(Command::Help);
    (command, args.collect())
}
