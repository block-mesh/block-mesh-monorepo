#[derive(Debug, thiserror::Error)]
pub enum ErrorCodes {
    #[error(transparent)]
    GetLatestBlockhash(#[from] anyhow::Error),
}
