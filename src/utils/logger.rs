use chrono::Local;
use colored::*;
use std::io::{self, Write};

// DATE TIME Formmated
const DATETIME_FORMAT: &str = "%Y/%m/%d %H:%M:%S";

pub struct Logger {
    out: io::Stdout,
}

impl Logger {
    pub fn console_log() -> Logger {
        Logger { out: io::stdout() }
    }

    fn log(&mut self, level: &str, color: fn(&str) -> ColoredString, format_message: &str) {
        let time = Local::now().format(DATETIME_FORMAT);
        let formatted = format!(" {:<8} {} 💬 {}\n", color(level), time, format_message);
        self.out
            .write_all(formatted.as_bytes())
            .expect("Failed to write to stdout");
    }

    pub fn info(&mut self, format_message: std::fmt::Arguments) {
        self.log(
            "[INFO]",
            |level: &str| level.on_green(),
            &format!("{}", format!("{}", format_message).as_str().green()),
        );
    }

    pub fn warn(&mut self, format_message: std::fmt::Arguments) {
        self.log(
            "[WARING]",
            |level| level.on_purple(),
            &format!("{}", format!("{}", format_message).as_str().yellow()),
        );
    }

    pub fn error(&mut self, format_message: std::fmt::Arguments) {
        self.log(
            "[ERROR]",
            |level: &str| level.on_red(),
            &format!("{}", format!("{}", format_message).as_str().red()),
        );
    }
}
