use super::{output_banner, output_banner_help};
use crate::book::{self, settings, ResolvedDirectories, Settings};
use crate::utils::{self, invalid_input, Logger};
use crate::TypikonResult;
use colored::Colorize;
use notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_mini::{new_debouncer, DebounceEventResult, DebouncedEvent, Debouncer};
use std::collections::HashSet;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Sender};
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum Command {
    Build,
    Init,
    Help,
    Serve,
    Watch,
    Version,
    Unknown(String),
}

impl Command {
    pub fn parse(value: &str) -> Command {
        match value {
            "build" => Command::Build,
            "init" => Command::Init,
            "help" | "--help" | "-h" => Command::Help,
            "serve" => Command::Serve,
            "watch" => Command::Watch,
            "version" | "--version" | "-V" => Command::Version,
            _ => Command::Unknown(value.to_string()),
        }
    }

    // Preserve the original public helper while new code uses the less ambiguous parse name.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(value: &str) -> Command {
        Self::parse(value)
    }
}

impl fmt::Display for Command {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Build => write!(formatter, "build"),
            Command::Serve => write!(formatter, "serve"),
            Command::Init => write!(formatter, "init"),
            Command::Help => write!(formatter, "help"),
            Command::Watch => write!(formatter, "watch"),
            Command::Version => write!(formatter, "version"),
            Command::Unknown(value) => write!(formatter, "Unknown({value})"),
        }
    }
}

const BUILD_HELP_TEXT: &str = r"
    Build a project from its working directory:
        cd example && typikon build
";

const INIT_HELP_TEXT: &str = r"
    Initialize a new project in an empty directory:
        mkdir example && cd example && typikon init
";

const SERVE_HELP_TEXT: &str = r"
    Serve the most recently built output:
        cd example && typikon serve
";

const WATCH_HELP_TEXT: &str = r"
    Build, serve, and rebuild when project inputs change:
        cd example && typikon watch
";

pub fn handle_build_command(args: &[String]) -> TypikonResult<()> {
    ensure_no_args(args, "build")?;
    output_banner();
    build_book_at(&std::env::current_dir()?)
}

pub fn handle_serve_command(args: &[String]) -> TypikonResult<()> {
    ensure_no_args(args, "serve")?;
    output_banner();
    let project_root = std::env::current_dir()?;
    let settings = settings::get_settings_from_file(project_root.join("settings.yml"))?;
    let directories = settings.resolve_directories(&project_root)?;
    if !directories.output.is_dir() {
        return Err(invalid_input(format!(
            "Output directory does not exist; run `typikon build` first: {}",
            directories.output.display()
        ))
        .into());
    }

    let runtime = Runtime::new()?;
    let docs = warp::fs::dir(directories.output);
    let (address, server) = warp::serve(docs).try_bind_with_graceful_shutdown(
        ([127, 0, 0, 1], settings.port),
        async {
            let _ = tokio::signal::ctrl_c().await;
        },
    )?;

    let mut log = Logger::console_log();
    log.info(format_args!("HTTP server listening on http://{address}"));
    runtime.block_on(server);
    Ok(())
}

pub fn handle_watch_command() -> TypikonResult<()> {
    handle_watch_command_with_args(&[])
}

pub fn handle_watch_command_with_args(args: &[String]) -> TypikonResult<()> {
    ensure_no_args(args, "watch")?;
    output_banner();
    let project_root = std::env::current_dir()?.canonicalize()?;
    build_book_at(&project_root)?;

    let mut settings = settings::get_settings_from_file(project_root.join("settings.yml"))?;
    let mut directories = settings.resolve_directories(&project_root)?;
    let runtime = Runtime::new()?;
    let mut server = RunningServer::start(&runtime, &settings, &directories)?;
    let (sender, receiver) = channel();
    let mut _debouncer = create_project_debouncer(&directories, sender.clone())?;
    let settings_path = project_root.join("settings.yml");
    let mut log = Logger::console_log();

    for result in receiver {
        let events = result?;
        if !events_require_rebuild(&events) {
            continue;
        }

        log.info(format_args!("Detected {} changed path(s)", events.len()));
        if let Err(error) = build_book_at(&project_root) {
            log.error(format_args!("Rebuild failed: {error}"));
            continue;
        }

        // Reload watchers and the HTTP server when settings may have changed their paths or port.
        if events.iter().any(|event| event.path == settings_path) {
            let next_settings = match settings::get_settings_from_file(&settings_path) {
                Ok(value) => value,
                Err(error) => {
                    log.error(format_args!("Failed to reload settings: {error}"));
                    continue;
                }
            };
            let next_directories = match next_settings.resolve_directories(&project_root) {
                Ok(value) => value,
                Err(error) => {
                    log.error(format_args!("Failed to resolve updated settings: {error}"));
                    continue;
                }
            };

            let next_debouncer = create_project_debouncer(&next_directories, sender.clone())?;
            if next_settings.port != settings.port || next_directories.output != directories.output
            {
                server.stop(&runtime)?;
                server = RunningServer::start(&runtime, &next_settings, &next_directories)?;
            }
            _debouncer = next_debouncer;
            settings = next_settings;
            directories = next_directories;
        }
    }

    server.stop(&runtime)?;
    Ok(())
}

pub fn events_require_rebuild(events: &[DebouncedEvent]) -> bool {
    !events.is_empty()
}

pub fn handle_help_command(args: &[String]) -> TypikonResult<()> {
    if args.is_empty() {
        output_banner_help();
        return Ok(());
    }
    if args.len() > 1 {
        return Err(invalid_input("The help command accepts at most one command name").into());
    }

    let command = Command::parse(&args[0]);
    let help_text = match command {
        Command::Build => BUILD_HELP_TEXT,
        Command::Init => INIT_HELP_TEXT,
        Command::Serve => SERVE_HELP_TEXT,
        Command::Watch => WATCH_HELP_TEXT,
        Command::Help => {
            output_banner_help();
            return Ok(());
        }
        Command::Version => "\n    Show the installed Typikon version.\n",
        Command::Unknown(value) => {
            return Err(invalid_input(format!("Unknown help topic: {value}")).into())
        }
    };
    println!("{}", help_text.green());
    Ok(())
}

pub fn handle_init_command(args: &[String]) -> TypikonResult<()> {
    ensure_no_args(args, "init")?;
    output_banner();
    ensure_current_directory_is_empty()?;

    let mut log = Logger::console_log();
    log.warn(format_args!("Downloading project resources..."));
    let result = (|| -> TypikonResult<()> {
        utils::download_zip()?;
        utils::move_dir_contents(Path::new("typikon-book-v3-main"), Path::new("."))?;
        Ok(())
    })();

    // Cleanup download artifacts on both success and failure.
    let cleanup_result = cleanup_init_artifacts();
    result?;
    cleanup_result?;
    log.info(format_args!("Working directory initialized successfully"));
    Ok(())
}

fn ensure_no_args(args: &[String], command: &str) -> TypikonResult<()> {
    if args.is_empty() {
        return Ok(());
    }
    Err(invalid_input(format!("The {command} command does not accept arguments")).into())
}

fn build_book_at(project_root: &Path) -> TypikonResult<()> {
    let mut builder = book::new_builder_at(project_root)?;
    builder.generate_books()?;
    let mut log = Logger::console_log();
    log.info(format_args!("Static book rendered successfully"));
    Ok(())
}

fn create_project_debouncer(
    directories: &ResolvedDirectories,
    sender: Sender<DebounceEventResult>,
) -> TypikonResult<Debouncer<RecommendedWatcher>> {
    let mut debouncer = new_debouncer(Duration::from_secs(1), sender)?;
    let targets = watch_targets(directories);
    for (path, recursive_mode) in targets {
        debouncer.watcher().watch(&path, recursive_mode)?;
    }
    Ok(debouncer)
}

fn watch_targets(directories: &ResolvedDirectories) -> Vec<(PathBuf, RecursiveMode)> {
    let candidates = [
        (directories.input.clone(), RecursiveMode::Recursive),
        (directories.active_theme.clone(), RecursiveMode::Recursive),
        (
            directories.project_root.join("settings.yml"),
            RecursiveMode::NonRecursive,
        ),
        (
            directories.project_root.join("root.yml"),
            RecursiveMode::NonRecursive,
        ),
    ];
    let mut seen = HashSet::new();
    candidates
        .into_iter()
        .filter(|(path, _)| seen.insert(path.clone()))
        .collect()
}

fn ensure_current_directory_is_empty() -> TypikonResult<()> {
    if std::fs::read_dir(".")?.next().is_none() {
        return Ok(());
    }
    Err(invalid_input("The init command requires an empty working directory").into())
}

fn cleanup_init_artifacts() -> TypikonResult<()> {
    utils::delete_folder(Path::new("typikon-book-v3-main"))?;
    utils::delete_file(Path::new("repo.zip"))?;
    utils::delete_file(Path::new("repo.zip.part"))?;
    Ok(())
}

struct RunningServer {
    shutdown: Option<oneshot::Sender<()>>,
    task: JoinHandle<()>,
}

impl RunningServer {
    fn start(
        runtime: &Runtime,
        settings: &Settings,
        directories: &ResolvedDirectories,
    ) -> TypikonResult<Self> {
        let docs = warp::fs::dir(directories.output.clone());
        let (shutdown, receiver) = oneshot::channel();
        let (address, server) = warp::serve(docs).try_bind_with_graceful_shutdown(
            ([127, 0, 0, 1], settings.port),
            async {
                let _ = receiver.await;
            },
        )?;
        let task = runtime.spawn(server);
        let mut log = Logger::console_log();
        log.info(format_args!("HTTP server listening on http://{address}"));
        Ok(Self {
            shutdown: Some(shutdown),
            task,
        })
    }

    fn stop(mut self, runtime: &Runtime) -> TypikonResult<()> {
        if let Some(shutdown) = self.shutdown.take() {
            let _ = shutdown.send(());
        }
        runtime.block_on(self.task)?;
        Ok(())
    }
}
