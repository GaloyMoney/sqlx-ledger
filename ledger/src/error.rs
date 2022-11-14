use thiserror::Error;

use cel_interpreter::CelError;

use crate::tx_template::ParamDataType;

#[derive(Error, Debug)]
pub enum SqlxLedgerError {
    #[error("SqlxLedgerError - Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("SqlxLedgerError - SerdeJson: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("SqlxLedgerError - CelError: {0}")]
    CelError(#[from] CelError),
    #[error("SqlxLedgerError - TxParamTypeMissmatch: expected {0:?}")]
    TxParamTypeMissmatch(ParamDataType),
    #[error("SqlxLedgerError - TooManyParameters")]
    TooManyParameters,
}
