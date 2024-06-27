use std::process::exit;
use typikon::cli::Command;

fn main() {
    typikon::cli::out_banner_string();

    let (command, args) = typikon::cli::parse_args();

    match command {
        Command::Init => typikon::cli::handle_init_command(&args),
        Command::Help => typikon::cli::handle_help_command(&args),
        Command::Build => typikon::cli::handle_build_command(&args),
        Command::Theme => typikon::cli::handle_theme_command(&args),
        Command::Unknown(s) => {
            eprintln!("Unknown command: {}", s);
            exit(1)
        }
    }
}
