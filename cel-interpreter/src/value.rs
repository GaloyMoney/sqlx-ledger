use std::{collections::HashMap, rc::Rc};

use cel_parser::ast::Literal;

use crate::{cel_type::*, error::*};

#[derive(Debug, Clone, PartialEq)]
pub enum CelValue {
    Map(Rc<CelMap>),
    Int(i64),
    UInt(u64),
    Double(f64),
    String(Rc<String>),
    Bytes(Rc<Vec<u8>>),
    Bool(bool),
    Null,
}

impl CelValue {
    pub(crate) fn try_bool(&self) -> Result<bool, CelError> {
        if let CelValue::Bool(val) = self {
            Ok(*val)
        } else {
            Err(CelError::BadType(CelType::Bool, CelType::from(self)))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CelMap {
    inner: HashMap<CelKey, CelValue>,
}

impl CelMap {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: impl Into<CelKey>, val: impl Into<CelValue>) {
        self.inner.insert(k.into(), val.into());
    }

    pub fn get(&self, key: impl Into<CelKey>) -> CelValue {
        self.inner
            .get(&key.into())
            .map(Clone::clone)
            .unwrap_or(CelValue::Null)
    }
}

impl From<CelMap> for CelValue {
    fn from(m: CelMap) -> Self {
        CelValue::Map(Rc::from(m))
    }
}

impl From<i64> for CelValue {
    fn from(i: i64) -> Self {
        CelValue::Int(i)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum CelKey {
    Integer(i64),
    Uint(u32),
    Bool(bool),
    String(Rc<String>),
}

impl From<&str> for CelKey {
    fn from(s: &str) -> Self {
        CelKey::String(Rc::from(s.to_string()))
    }
}

impl From<&Rc<String>> for CelKey {
    fn from(s: &Rc<String>) -> Self {
        CelKey::String(s.clone())
    }
}

impl From<&CelValue> for CelType {
    fn from(v: &CelValue) -> Self {
        match v {
            CelValue::Map(_) => CelType::Map,
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
