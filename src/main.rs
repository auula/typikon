use typikon::cli::Command;

fn main() {
    // deconstructing command line input to parameter information
    let (command, args) = typikon::cli::parse_args();
    // match processing of the corresponding command
    match command {
        Command::Init => typikon::cli::handle_init_command(&args),
        Command::Help => typikon::cli::handle_help_command(&args),
        Command::Build => typikon::cli::handle_build_command(&args),
        Command::Serve => typikon::cli::handle_serve_command(&args),
        Command::Unknown(_) => typikon::cli::output_banner_help(),
    }
}
