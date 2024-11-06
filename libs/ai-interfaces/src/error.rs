#[derive(Debug, thiserror::Error)]
pub enum AiInterfaceError {
    #[error("DB Error: {0}")]
    DBError(String),
}
