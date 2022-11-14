use thiserror::Error;

use std::rc::Rc;

use crate::cel_type::*;

#[derive(Error, Debug)]
pub enum CelError {
    #[error("CelError - CelParseError: {0}")]
    CelParseError(String),
    #[error("CelError - BadType: expected {0:?} found {1:?}")]
    BadType(CelType, CelType),
    #[error("CelError - UnknownIdentifier: {0}")]
    UnknownIdent(Rc<String>),
    #[error("CelError - IllegalTarget: {0}")]
    IllegalTarget(String),
    #[error("CelError - Unexpected: {0}")]
    Unexpected(String),
}
