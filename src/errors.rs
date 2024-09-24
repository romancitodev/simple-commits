use thiserror::Error;

use crate::tui::BuildError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error building the commit")]
    BuildError(BuildError),
    #[error("Error on I/O: {0:?}")]
    IO(#[from] std::io::Error),
    #[error("Error on Step: {0:?}")]
    Step(String),
}
