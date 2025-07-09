#[derive(Debug, thiserror::Error)]
pub enum CallError {
    #[error("{0}")]
    Dropped(String),
    #[error("{0}")]
    Failed(String),
    #[error("{0}")]
    Invalid(String),
    #[error(transparent)]
    SubxtError(#[from] subxt::error::Error),
}
