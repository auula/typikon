use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The \"root.yml\" mapping file was not found")]
    RootFileNotFound,
    #[error("The \"root.yml\" content is not formatted properly")]
    RootFileContentNotFormatted,
}
