#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Syn parsing error: {0}")]
    SynError(#[from] syn::Error),
    #[error("API module not found in the provided file")]
    ApiModuleNotFound,
    #[error("Storage module not found in pallet: {pallet}")]
    StorageModuleNotFound { pallet: String },
    #[error("StorageApi implementation not found in pallet: {pallet}")]
    StorageApiNotFound { pallet: String },
    #[error("Invalid method signature: {method}")]
    InvalidMethodSignature { method: String },
    #[error("Type information not found for storage: {storage}")]
    TypeInfoNotFound { storage: String },
}
