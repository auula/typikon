use std::env;
use typikon::command::Command;

const BANNER: &str = r"
  _             _ _            
 | |_ _  _ _ __(_) |_____ _ _  
 |  _| || | '_ \ | / / _ \ ' \ 
  \__|\_, | .__/_|_\_\___/_||_|
      |__/|_|     v{}         
";

const VERSION: &str = "0.1.1-beta";

pub fn to_banner_string() -> String {
    format!("{}", BANNER.replace("{}", VERSION))
}

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if there are enough arguments
    if args.len() < 2 {
        println!("{}\nUsage: typikon <command> [<args>]", to_banner_string());
        return;
    }

    // Get the first argument, which represents the sub-command
    // Match on the enum variant
    match Command::from_str(&args[1]) {
        Command::Init => typikon::command::handle_init(),
        Command::Build => typikon::command::handle_build(),
        Command::Theme => typikon::command::handle_theme(&args),
        Command::Unknown(s) => {
            eprintln!("Unknown command: {}", s);
        }
    }
}
