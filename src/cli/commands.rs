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

static HELP_INFO: Lazy<Mutex<HashMap<Command, colored::ColoredString>>> = Lazy::new(|| {
    let mut help_info: HashMap<Command, colored::ColoredString> = HashMap::new();

    help_info.insert(Command::Theme, THEME_HELP_TEXT.green());
    help_info.insert(Command::Build, BUILD_HELP_TEXT.green());
    help_info.insert(Command::Init, INIT_HELP_TEXT.green());

    Mutex::new(help_info)
});

pub fn handle_theme_command(args: &[String]) {
    let mut log = Logger::console_log();
    match args.get(2) {
        Some(option) => {
            if let Some(value) = option.strip_prefix("--get=") {
                log.info(format_args!("Fetching theme parameter: {}", value))
            } else {
                log.error(format_args!("{}", option))
            }
        }
        None => log.warn(format_args!("Lack of options: --get=<value>")),
    }
}

// 通过命令行传入的 args 参数打印 help 信息
pub fn handle_help_command(args: &[String]) {
    let mut log = Logger::console_log();
    if let Some(option) = args.get(0) {
        let help = HELP_INFO.lock().unwrap();
        match Command::from_str(option) {
            Command::Build | Command::Theme | Command::Init | Command::Help => {
                if let Some(help_text) = help.get(&Command::from_str(option)) {
                    println!("{}", help_text);
                } else {
                    log.error(format_args!("No help available for command: {}", option))
                }
            }
            Command::Unknown(_) => log.error(format_args!(
                "Unknown option: <{}> Available option: [init,theme,build]",
                option
            )),
        }
    } else {
        log.error(format_args!(
            "Lack of options! Available option: [init,theme,build]"
        ))
    }
}

pub fn handle_init_command(_args: &[String]) {
    let mut log = Logger::console_log();
    unimplemented!()
}

pub fn handle_build_command(_args: &[String]) {
    let mut log = Logger::console_log();
    match book::new_builder() {
        Ok(mut builder) => match builder.render() {
            Ok(_) => {}
            Err(err) => log.error(format_args!("{}", err)),
        },
        Err(err) => log.error(format_args!("{}", err)),
    }
}
