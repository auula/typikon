use std::io::{self, Write};
use colored::*;

pub struct Console {
    out: io::Stdout,
}

impl Console {
    pub fn new() -> Console {
        Console { out: io::stdout() }
    }

    fn log(&mut self, level: &str, color: fn(&str) -> ColoredString, format_message: std::fmt::Arguments) {
        let formatted = format!(" {:<7} {}\n", color(level), format_message);
        self.out.write_all(formatted.as_bytes()).expect("Failed to write to stdout");
    }

    pub fn info(&mut self, format_message: std::fmt::Arguments) {
        self.log("[INFO]", |msg| msg.green(), format_message);
    }

    pub fn warn(&mut self, format_message: std::fmt::Arguments) {
        self.log("[WARN]", |msg| msg.yellow(), format_message);
    }

    pub fn error(&mut self, format_message: std::fmt::Arguments) {
        self.log("[ERROR]", |msg| msg.red(), format_message);
    }
}