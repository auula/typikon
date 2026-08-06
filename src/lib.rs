use std::error::Error;

pub mod book;
pub mod cli;
pub mod html;
pub mod utils;

pub type TypikonError = Box<dyn Error + Send + Sync>;
pub type TypikonResult<T> = Result<T, TypikonError>;
