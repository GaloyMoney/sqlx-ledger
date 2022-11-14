use rust_decimal::Decimal;
use thiserror::Error;

use cel_interpreter::{CelError, CelValue};

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
    #[error("SqlxLedgerError - UnknownLayer: {0:?}")]
    UnknownLayer(CelValue),
    #[error("SqlxLedgerError - UnknownDebitOrCredit: {0:?}")]
    UnknownDebitOrCredit(CelValue),
    #[error("SqlxLedgerError - UnbalancedTransaction: currency {0} amount {1}")]
    UnbalancedTransaction(String, Decimal),
}
