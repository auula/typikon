use super::print_banner;
use crate::cli::BANNER;
use crate::utils::Logger;
use crate::{book, utils};
use clap::{ColorChoice, Parser};
use core::fmt;
use std::fmt::Display;
use std::path::Path;

#[derive(Parser, Debug, Eq, Hash, PartialEq)]
#[clap(name = "Typikon", version = "0.1.1-beta", author = "Typikon")]
#[command(before_help = format!("{}", BANNER))]
#[command(help_template = "{before-help}{usage-heading}\n{usage}\n\n{all-args}{after-help}")]
#[command(color = ColorChoice::Always)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: Command,
}

impl Cli {
    pub fn run(&self) {
        match &self.cmd {
            Command::Build(build) => build.run(),
            Command::Init(init) => init.run(),
        }
    }
}

#[derive(Parser, Debug, Eq, Hash, PartialEq)]
pub enum Command {
    #[clap(
        name = "build",
        about = "The construction of static html files must be done in the working directory 👇"
    )]
    Build(Build),
    /// Specify the initialization working directory 👇
    #[clap(name = "init")]
    Init(Init),
}

impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Build(_) => write!(f, "build"),
            Command::Init(_) => write!(f, "init"),
        }
    }
}

#[derive(Parser, Debug, Eq, Hash, PartialEq)]
pub struct Build {}

impl Build {
    pub fn run(&self) {
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
}

#[derive(Parser, Debug, Eq, Hash, PartialEq)]
pub struct Init {}

impl Init {
    pub fn run(&self) {
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
}
