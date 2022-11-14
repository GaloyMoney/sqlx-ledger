use std::rc::Rc;

use cel_parser::ast::Literal;

use crate::{cel_type::*, error::*};

#[derive(Debug, PartialEq)]
pub enum CelValue {
    Int(i64),
    UInt(u64),
    Double(f64),
    String(Rc<String>),
    Bytes(Rc<Vec<u8>>),
    Bool(bool),
    Null,
}

impl CelValue {
    pub(crate) fn try_bool(self) -> Result<bool, CelError> {
        if let CelValue::Bool(val) = self {
            Ok(val)
        } else {
            Err(CelError::BadType(CelType::Bool, CelType::from(self)))
        }
    }
}

impl From<CelValue> for CelType {
    fn from(v: CelValue) -> Self {
        match v {
            CelValue::Int(_) => CelType::Int,
            CelValue::UInt(_) => CelType::UInt,
            CelValue::Double(_) => CelType::Double,
            CelValue::String(_) => CelType::String,
            CelValue::Bytes(_) => CelType::Bytes,
            CelValue::Bool(_) => CelType::Bool,
            CelValue::Null => CelType::Null,
        }
    }
}

impl From<&Literal> for CelValue {
    fn from(l: &Literal) -> Self {
        use Literal::*;
        match l {
            Int(i) => CelValue::Int(*i),
            UInt(u) => CelValue::UInt(*u),
            Double(d) => CelValue::Double(*d),
            String(s) => CelValue::String(s.clone()),
            Bytes(b) => CelValue::Bytes(b.clone()),
            Bool(b) => CelValue::Bool(*b),
            Null => CelValue::Null,
        }
    }
}
