use super::ouput_banner;
use crate::{
    book::{self, settings},
    utils::{self, Logger},
};
use colored::Colorize;
use core::fmt;
use notify::RecursiveMode;
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::Mutex;
use std::{path::Path, time::Duration};
use tokio::runtime::Runtime;

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Command {
    Build,
    Init,
    Help,
    Serve,
    Watch,
    Unknown(String),
}

impl Command {
    pub fn from_str(s: &str) -> Command {
        match s {
            "build" => Command::Build,
            "init" => Command::Init,
            "help" => Command::Help,
            "serve" => Command::Serve,
            "watch" => Command::Watch,
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
            Command::Watch => write!(f, "watch"),
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

const WATCH_HELP_TEXT: &str = r"

    Example:

    Watch the file changes and rebuild the book 👇

    $: mkdir example && typikon watch
";

static HELP_INFO: Lazy<Mutex<HashMap<Command, colored::ColoredString>>> = Lazy::new(|| {
    let mut help_info: HashMap<Command, colored::ColoredString> = HashMap::new();

    help_info.insert(Command::Build, BUILD_HELP_TEXT.green());
    help_info.insert(Command::Serve, SERVE_HELP_TEXT.green());
    help_info.insert(Command::Init, INIT_HELP_TEXT.green());
    help_info.insert(Command::Watch, WATCH_HELP_TEXT.green());

    Mutex::new(help_info)
});

fn build_book() {
    let mut log = Logger::console_log();
    match book::new_builder() {
        Ok(mut builder) => match builder.generate_books() {
            Ok(_) => log.info(format_args!(
                "Rendering of static resource files complete 🎉"
            )),
            Err(err) => log.error(format_args!("{}", err)),
        },
        Err(err) => log.error(format_args!("{}", err)),
    }
}

pub fn handle_build_command(_args: &[String]) {
    build_book()
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

    log.info(format_args!(
        "Starting HTTP server on port {}",
        settings.port
    ));

    runtime.block_on(async {
        warp::serve(docs).run(([127, 0, 0, 1], settings.port)).await;
    });

    log.info(format_args!("HTTP server stopped."));
}

pub fn handle_watch_command() {
    let mut log = Logger::console_log();
    let settings = match settings::get_settings() {
        Ok(settings) => settings,
        Err(err) => {
            log.error(format_args!("Failed to get settings: {:?}", err));
            return;
        }
    };
    // create a new builder
    build_book();
    let runtime = match Runtime::new() {
        Ok(runtime) => runtime,
        Err(err) => {
            log.error(format_args!("Failed to create Tokio runtime: {:?}", err));
            return;
        }
    };

    let docs = warp::fs::dir(settings.directory.output.clone());

    log.info(format_args!(
        "Starting HTTP server on port {}",
        settings.port
    ));

    runtime.spawn(async move {
        warp::serve(docs).run(([127, 0, 0, 1], settings.port)).await;
    });
    // create a channel to receive file change events
    let (tx, rx) = channel();

    let mut debouncer = new_debouncer(Duration::from_secs(1), tx).unwrap();

    debouncer
        .watcher()
        .watch(
            Path::new(&settings.get_input_path()),
            RecursiveMode::Recursive,
        )
        .unwrap();
    runtime.block_on(async {
        for res in rx {
            match res {
                Ok(events) => events.iter().for_each(|event| match event.kind {
                    DebouncedEventKind::Any => {
                        log.info(format_args!("File changed: {:?}", event.path));
                        log.info(format_args!("Rebuilding book..."));
                        build_book();
                    }
                    _ => {
                        return;
                    }
                }),
                Err(error) => log.error(format_args!("watch error: {:?}", error)),
            }
        }
    });
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
        Command::Build | Command::Init | Command::Help | Command::Serve | Command::Watch => {
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

    if let Err(err) = utils::move_dir_contents(Path::new("typikon-book-v3-main"), Path::new(".")) {
        log.error(format_args!("{:?}", err));
        return;
    }

    if let Err(err) = utils::delete_folder(Path::new("typikon-book-v3-main")) {
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
