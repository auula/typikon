use typikon::{cli::Command, utils::Logger};

fn main() {
    // global logger ouput to console
    let mut log = Logger::console_log();
    // deconstructing command line input to parameter information
    let (command, args) = typikon::cli::parse_args();
    // Match processing of the corresponding command
    match command {
        Command::Init => typikon::cli::handle_init_command(&args, &mut log),
        Command::Help => typikon::cli::handle_help_command(&args, &mut log),
        Command::Build => typikon::cli::handle_build_command(&args, &mut log),
        Command::Theme => typikon::cli::handle_theme_command(&args, &mut log),
        Command::Unknown(_) => typikon::cli::out_banner_string(),
    }
}
