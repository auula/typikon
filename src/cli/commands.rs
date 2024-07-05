use colored::Colorize;
use core::fmt;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use tokio::runtime::Runtime;

use crate::{
    book::{self, settings},
    utils::{self, Logger},
};

use super::ouput_banner;

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Command {
    Build,
    Init,
    Help,
    Serve,
    Unknown(String),
}

impl Command {
    pub fn from_str(s: &str) -> Command {
        match s {
            "build" => Command::Build,
            "init" => Command::Init,
            "help" => Command::Help,
            "serve" => Command::Serve,
            _ => Command::Unknown(s.to_string()),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Build => write!(f, "build"),
            Command::Serve => write!(f, "serve"),
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

const SERVE_HELP_TEXT: &str = r"

	Example:

	Serve starting the static http server service 👇

	$: mkdir example && typikon serve

";

static HELP_INFO: Lazy<Mutex<HashMap<Command, colored::ColoredString>>> = Lazy::new(|| {
    let mut help_info: HashMap<Command, colored::ColoredString> = HashMap::new();

    help_info.insert(Command::Build, BUILD_HELP_TEXT.green());
    help_info.insert(Command::Serve, SERVE_HELP_TEXT.green());
    help_info.insert(Command::Init, INIT_HELP_TEXT.green());

    Mutex::new(help_info)
});

pub fn handle_build_command(_args: &[String]) {
    let mut log = Logger::console_log();
    match book::new_builder() {
        Ok(builder) => match builder.generate_books() {
            Ok(_) => log.info(format_args!(
                "Rendering of static resource files complete 🎉"
            )),
            Err(err) => log.error(format_args!("{}", err)),
        },
        Err(err) => log.error(format_args!("{}", err)),
    }
}

pub fn handle_serve_command(_args: &[String]) {
    let mut log = Logger::console_log();
    ouput_banner();
    let settings = match settings::get_settings() {
        Ok(settings) => settings,
        Err(err) => {
            log.error(format_args!("Failed to get settings: {:?}", err));
            return;
        }
    };

    let runtime = match Runtime::new() {
        Ok(runtime) => runtime,
        Err(err) => {
            log.error(format_args!("Failed to create Tokio runtime: {:?}", err));
            return;
        }
    };

    let docs = warp::fs::dir(settings.directory.output.clone());

    log.info(format_args!("Starting HTTP server on port {}", settings.port));

    runtime.block_on(async {
        warp::serve(docs).run(([127, 0, 0, 1], settings.port)).await;
    });

    log.info(format_args!("HTTP server stopped."));
}

pub fn handle_help_command(args: &[String]) {
    let mut log = Logger::console_log();
    if args.is_empty() {
        log.error(format_args!("Available options: [init, serve, build]"));
        return;
    }

    let option = &args[0];
    let command = Command::from_str(option);
    let help = HELP_INFO.lock().unwrap();

    match command {
        Command::Build | Command::Init | Command::Help | Command::Serve => {
            if let Some(help_text) = help.get(&command) {
                println!("{}", help_text);
            } else {
                log.error(format_args!("No help available for command: {}", option));
            }
        }
        Command::Unknown(_) => {
            log.error(format_args!(
                "Unknown option: {:?}. Available options: [init, serve, build]",
                option
            ));
        }
    }
}

pub fn handle_init_command(_args: &[String]) {
    let mut log = Logger::console_log();
    ouput_banner();
    log.warn(format_args!("Downloading resource files, please wait..."));

    if let Err(err) = utils::download_zip() {
        log.error(format_args!("{:?}", err));
        return;
    }

    if let Err(err) = utils::move_dir_contents(Path::new("typikon-book-main"), Path::new(".")) {
        log.error(format_args!("{:?}", err));
        return;
    }

    if let Err(err) = utils::delete_folder(Path::new("typikon-book-main")) {
        log.error(format_args!("{:?}", err));
        return;
    }

    if let Err(err) = utils::delete_file(Path::new("repo.zip")) {
        log.error(format_args!("{:?}", err));
        return;
    }

    log.info(format_args!(
        "Initializing the working directory was successful"
    ));
}
