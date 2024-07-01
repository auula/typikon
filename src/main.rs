use typikon::cli::Command;

fn main() {
    // deconstructing command line input to parameter information
    let (command, args) = typikon::cli::parse_args();
    // Match processing of the corresponding command
    match command {
        Command::Init => typikon::cli::handle_init_command(&args),
        Command::Help => typikon::cli::handle_help_command(&args),
        Command::Build => typikon::cli::handle_build_command(&args),
        Command::Theme => typikon::cli::handle_theme_command(&args),
        Command::Unknown(_) => typikon::cli::out_banner_string(),
    }
}
