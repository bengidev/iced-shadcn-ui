#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("network error: {0}")]
    Network(String),
    #[error("io error: {0}")]
    Io(String),
    #[error("{0}")]
    Message(String),
}
