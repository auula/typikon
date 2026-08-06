use notify_debouncer_mini::{DebouncedEvent, DebouncedEventKind};
use std::process::Command as ProcessCommand;
use typikon::cli::{events_require_rebuild, parse_args_from, Command};

#[test]
fn parses_help_and_version_aliases() {
    assert_eq!(parse_args_from(Vec::<String>::new()).0, Command::Help);
    assert_eq!(parse_args_from(["-h"]).0, Command::Help);
    assert_eq!(parse_args_from(["--version"]).0, Command::Version);
    assert_eq!(
        parse_args_from(["help", "build"]),
        (Command::Help, vec!["build".to_string()])
    );
    assert_eq!(Command::from_str("build"), Command::Build);
}

#[test]
fn rebuilds_only_once_for_a_nonempty_event_batch() {
    let events = vec![
        DebouncedEvent {
            path: "book/a.md".into(),
            kind: DebouncedEventKind::Any,
        },
        DebouncedEvent {
            path: "book/b.md".into(),
            kind: DebouncedEventKind::AnyContinuous,
        },
    ];
    assert!(events_require_rebuild(&events));
    assert!(!events_require_rebuild(&[]));
}

#[test]
fn build_command_returns_failure_exit_code_for_an_invalid_project() {
    let directory = tempfile::tempdir().expect("temporary command directory");
    let output = ProcessCommand::new(env!("CARGO_BIN_EXE_typikon"))
        .arg("build")
        .current_dir(directory.path())
        .output()
        .expect("run typikon build");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("UTF-8 stderr");
    assert!(stderr.contains("Error:"));
}

#[test]
fn version_command_uses_package_version() {
    let output = ProcessCommand::new(env!("CARGO_BIN_EXE_typikon"))
        .arg("--version")
        .output()
        .expect("run typikon --version");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout)
            .expect("UTF-8 stdout")
            .trim(),
        format!("typikon {}", env!("CARGO_PKG_VERSION"))
    );
}
