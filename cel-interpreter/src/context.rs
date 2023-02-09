use std::{collections::HashMap, sync::Arc};

use crate::{builtins, error::*, value::*};

type CelFunction = Box<dyn Fn(Vec<CelValue>) -> Result<CelValue, CelError>>;
#[derive(Debug)]
pub struct CelContext {
    idents: HashMap<String, ContextItem>,
}

impl CelContext {
    pub fn new() -> Self {
        let mut idents = HashMap::new();
        idents.insert(
            "date".to_string(),
            ContextItem::Function(Box::new(builtins::date)),
        );
        idents.insert(
            "uuid".to_string(),
            ContextItem::Function(Box::new(builtins::uuid)),
        );
        idents.insert(
            "SETTLED".to_string(),
            ContextItem::Value(CelValue::String(Arc::from("SETTLED".to_string()))),
        );
        idents.insert(
            "PENDING".to_string(),
            ContextItem::Value(CelValue::String(Arc::from("PENDING".to_string()))),
        );
        idents.insert(
            "ENCUMBERED".to_string(),
            ContextItem::Value(CelValue::String(Arc::from("ENCUMBERED".to_string()))),
        );
        idents.insert(
            "DEBIT".to_string(),
            ContextItem::Value(CelValue::String(Arc::from("DEBIT".to_string()))),
        );
        idents.insert(
            "CREDIT".to_string(),
            ContextItem::Value(CelValue::String(Arc::from("CREDIT".to_string()))),
        );
        Self { idents }
    }
}
impl Default for CelContext {
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) enum ContextItem {
    Value(CelValue),
    Function(CelFunction),
}

impl std::fmt::Debug for ContextItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextItem::Value(val) => write!(f, "Value({val:?})"),
            ContextItem::Function(_) => write!(f, "Function"),
        }
    }
}

impl CelContext {
    pub(crate) fn lookup(&self, name: Arc<String>) -> Result<&ContextItem, CelError> {
        self.idents
            .get(name.as_ref())
            .ok_or_else(|| CelError::UnknownIdent(name.to_string()))
    }

    pub fn add_variable(&mut self, name: impl Into<String>, value: impl Into<CelValue>) {
        self.idents
            .insert(name.into(), ContextItem::Value(value.into()));
    }
}
