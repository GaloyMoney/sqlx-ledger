use rust_decimal::Decimal;
use sqlx::error::DatabaseError;
use thiserror::Error;

use cel_interpreter::CelError;

use crate::{event::SqlxLedgerEvent, primitives::*, tx_template::ParamDataType};

#[derive(Error, Debug)]
pub enum SqlxLedgerError {
    #[error("SqlxLedgerError - Sqlx: {0}")]
    UnknwownSqlx(sqlx::Error),
    #[error("SqlxLedgerError - DuplicateKey: {0}")]
    DuplicateKey(Box<dyn DatabaseError>),
    #[error("SqlxLedgerError - SerdeJson: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("SqlxLedgerError - SendEvent: {0}")]
    SendEvent(#[from] tokio::sync::broadcast::error::SendError<SqlxLedgerEvent>),
    #[error("SqlxLedgerError - CelError: {0}")]
    CelError(#[from] CelError),
    #[error("SqlxLedgerError - TxParamTypeMismatch: expected {0:?}")]
    TxParamTypeMismatch(ParamDataType),
    #[error("SqlxLedgerError - TooManyParameters")]
    TooManyParameters,
    #[error("SqlxLedgerError - UnknownLayer: {0:?}")]
    UnknownLayer(String),
    #[error("SqlxLedgerError - UnknownDebitOrCredit: {0:?}")]
    UnknownDebitOrCredit(String),
    #[error("SqlxLedgerError - UnknownCurrency: {0}")]
    UnknownCurrency(String),
    #[error("SqlxLedgerError - UnbalancedTransaction: currency {0} amount {1}")]
    UnbalancedTransaction(Currency, Decimal),
    #[error("SqlxLedgerError - OptimisticLockingError")]
    OptimisticLockingError,
    #[error("SqlxLedgerError - EventSubscriberClosed")]
    EventSubscriberClosed,
}

impl From<sqlx::Error> for SqlxLedgerError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::Database(err) if err.message().contains("duplicate key") => {
                SqlxLedgerError::DuplicateKey(err)
            }
            e => SqlxLedgerError::UnknwownSqlx(e),
        }
    }
}
