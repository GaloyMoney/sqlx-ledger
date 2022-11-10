use thiserror::Error;

#[derive(Error, Debug)]
pub enum SqlxLedgerError {
    #[error("SqlxLedgerError - Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("SqlxLedgerError - SerdeJson: {0}")]
    SerdeJson(#[from] serde_json::Error),
}
