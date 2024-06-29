use colored::Colorize;
use core::fmt;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::book;
use crate::utils::Logger;

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Command {
    Build,
    Theme,
    Init,
    Help,
    Unknown(String),
}

impl Command {
    pub fn from_str(s: &str) -> Command {
        match s {
            "build" => Command::Build,
            "theme" => Command::Theme,
            "init" => Command::Init,
            "help" => Command::Help,
            _ => Command::Unknown(s.to_string()),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Build => write!(f, "build"),
            Command::Theme => write!(f, "theme"),
            Command::Init => write!(f, "init"),
            Command::Help => write!(f, "help"),
            Command::Unknown(s) => write!(f, "Unknown({})", s),
        }
    }
}

const THEME_HELP_TEXT: &str = r"

	Example:

	Use the --get parameter to get the custom theme repository and install it 👇

	$: typikon theme --get=https://github.com/auula/typikon-theme.git

";

const BUILD_HELP_TEXT: &str = r"

	Example:

	The construction of static html files must be done in the working directory 👇
    
	$: cd example && typikon build

";

const INIT_HELP_TEXT: &str = r"

	Example:

	Specify the initialization working directory 👇
    
	$: mkdir example && cd example && typikon init

";

static HELPS: Lazy<Mutex<HashMap<Command, colored::ColoredString>>> = Lazy::new(|| {
    let mut helps: HashMap<Command, colored::ColoredString> = HashMap::new();

    helps.insert(Command::Theme, THEME_HELP_TEXT.green());
    helps.insert(Command::Build, BUILD_HELP_TEXT.green());
    helps.insert(Command::Init, INIT_HELP_TEXT.green());

    Mutex::new(helps)
});

pub fn handle_theme_command(args: &[String]) {
    match args.get(2) {
        Some(option) => {
            if let Some(value) = option.strip_prefix("--get=") {
                println!("Fetching theme parameter: {}", value);
            } else {
                eprintln!("Unknown option: {}", option);
            }
        }
        None => {
            eprintln!("Lack of options: --get=<value>");
        }
    }
}

// 通过命令行传入的 args 参数打印 help 信息
pub fn handle_help_command(args: &[String]) {
    if let Some(option) = args.get(0) {
        let helps = HELPS.lock().unwrap();
        match Command::from_str(option) {
            Command::Build | Command::Theme | Command::Init | Command::Help => {
                if let Some(help_text) = helps.get(&Command::from_str(option)) {
                    println!("{}", help_text);
                } else {
                    eprintln!("No help available for command: {}", option);
                }
            }
            Command::Unknown(_) => {
                eprintln!(
                    "Unknown option: {} \nAvailable option: [init,theme,build]",
                    option
                );
            }
        }
    }
}

pub fn handle_init_command(_args: &[String]) {
    unimplemented!()
}

pub fn handle_build_command(_args: &[String]) {
    let mut log = Logger::console_log();
    match book::new_builder() {
        Ok(mut builder) => {
            builder.set_out_dir("./new_docs/");
            if let Err(err) = builder.render() {
                log.error(format_args!("{}", err));
            }
        }
        Err(err) => {
            log.error(format_args!("{}", err));
        }
    }
}
