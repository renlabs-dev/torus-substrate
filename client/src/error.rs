pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Call(#[from] CallError),
    #[error(transparent)]
    SubxtError(#[from] subxt::error::Error),
    #[error(transparent)]
    CodecError(#[from] codec::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum CallError {
    #[error("{0}")]
    Dropped(String),
    #[error("{0}")]
    Failed(String),
    #[error("{0}")]
    Invalid(String),
}
