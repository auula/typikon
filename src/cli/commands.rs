use colored::Colorize;
use core::fmt;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;
use std::sync::Mutex;

use crate::utils::Logger;
use crate::{book, utils};

use super::print_banner;

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Command {
    Build,
    Init,
    Help,
    Unknown(String),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ret = match s {
            "build" => Command::Build,
            "init" => Command::Init,
            "help" => Command::Help,
            _ => Command::Unknown(s.to_string()),
        };
        Ok(ret)
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Build => write!(f, "build"),
            Command::Init => write!(f, "init"),
            Command::Help => write!(f, "help"),
            Command::Unknown(s) => write!(f, "Unknown({})", s),
        }
    }
}

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

    help_info.insert(Command::Build, BUILD_HELP_TEXT.green());
    help_info.insert(Command::Init, INIT_HELP_TEXT.green());

    Mutex::new(help_info)
});

// 通过命令行传入的 args 参数打印 help 信息
pub fn handle_help_command(args: &[String]) {
    let mut log = Logger::console_log();
    if let Some(option) = args.first() {
        let help = HELP_INFO.lock().unwrap();
        match Command::from_str(option) {
            Ok(Command::Build) | Ok(Command::Init) | Ok(Command::Help) => {
                if let Some(help_text) = help.get(&Command::from_str(option).expect("NEVER Failed"))
                {
                    println!("{}", help_text);
                } else {
                    log.error(format_args!("No help available for command: {}", option))
                }
            }
            Ok(Command::Unknown(_)) => log.error(format_args!(
                "Unknown option: <{}> Available option: [init,build]",
                option
            )),
            Err(_) => log.error(format_args!(
                "Unknown option: <{}> Available option: [init,build]",
                option
            )),
        }
    } else {
        log.error(format_args!(
            "Lack of options! Available option: [init,build]"
        ))
    }
}

pub fn handle_init_command(_args: &[String]) {
    let mut log = Logger::console_log();
    print_banner();
    log.warn(format_args!("Downloading resource files please wait..."));

    if let Err(err) = utils::download_zip() {
        log.error(format_args!("{:?}", err));
    }

    if let Err(err) = utils::move_dir_contents(Path::new("typikon-book-main"), Path::new(".")) {
        log.error(format_args!("{:?}", err));
    }

    if let Err(err) = utils::delete_folder(Path::new("typikon-book-main")) {
        log.error(format_args!("{:?}", err));
    }

    if let Err(err) = utils::delete_file(Path::new("repo.zip")) {
        log.error(format_args!("{:?}", err));
    }

    log.info(format_args!(
        "Initializing the working directory successful"
    ));
}

pub fn handle_build_command(_args: &[String]) {
    let mut log = Logger::console_log();
    match book::new_builder() {
        Ok(mut builder) => match builder.render() {
            Ok(_) => log.info(format_args!(
                "Rendering of static resource files complete 🎉"
            )),
            Err(err) => log.error(format_args!("{}", err)),
        },
        Err(err) => log.error(format_args!("{}", err)),
    }
}
