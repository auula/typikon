use chrono::Local;
use colored::*;
use std::fmt;
use std::io::{self, Write};

const DATETIME_FORMAT: &str = "%Y/%m/%d %H:%M:%S";

#[derive(Debug)]
pub struct Logger {
    out: io::Stdout,
}

impl Logger {
    pub fn console_log() -> Logger {
        Logger { out: io::stdout() }
    }

    fn log(&mut self, level: &str, color: fn(&str) -> ColoredString, format_message: &str) {
        let time = Local::now().format(DATETIME_FORMAT);
        let formatted = format!("{} {} {}\n", color(level), time, format_message);
        if let Err(error) = self.out.write_all(formatted.as_bytes()) {
            eprintln!("Failed to write log output: {error}");
        }
    }

    pub fn info(&mut self, format_message: fmt::Arguments) {
        self.log(
            "[INFO]",
            |level: &str| level.green(),
            &format_message.to_string().green().to_string(),
        );
    }

    pub fn warn(&mut self, format_message: fmt::Arguments) {
        self.log(
            "[WARN]",
            |level| level.purple(),
            &format_message.to_string().yellow().to_string(),
        );
    }

    pub fn error(&mut self, format_message: fmt::Arguments) {
        self.log(
            "[ERROR]",
            |level: &str| level.red(),
            &format_message.to_string().red().to_string(),
        );
    }
}
