use std::process::ExitCode;
use typikon::cli::Command;
use typikon::TypikonResult;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> TypikonResult<()> {
    let (command, args) = typikon::cli::parse_args();
    match command {
        Command::Init => typikon::cli::handle_init_command(&args),
        Command::Help => typikon::cli::handle_help_command(&args),
        Command::Build => typikon::cli::handle_build_command(&args),
        Command::Serve => typikon::cli::handle_serve_command(&args),
        Command::Watch => typikon::cli::handle_watch_command_with_args(&args),
        Command::Version => {
            ensure_no_args(&args, "version")?;
            println!("typikon {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Command::Unknown(value) => {
            typikon::cli::output_banner_help();
            Err(format!("Unknown command: {value}").into())
        }
    }
}

fn ensure_no_args(args: &[String], command: &str) -> TypikonResult<()> {
    if args.is_empty() {
        return Ok(());
    }
    Err(format!("The {command} command does not accept arguments").into())
}
