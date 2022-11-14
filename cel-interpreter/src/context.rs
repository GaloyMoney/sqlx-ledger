use std::{collections::HashMap, rc::Rc};

use crate::{error::*, value::*};

// type CelFunction = Box<dyn Fn(Option<&Value>, &[Expression], &Context) -> CelValue>;
pub struct CelContext {
    idents: HashMap<String, ContextItem>,
}

impl CelContext {
    pub fn new() -> Self {
        Self {
            idents: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum ContextItem {
    Value(CelValue),
    // Function(CelFunction),
}

impl CelContext {
    pub(crate) fn lookup(&self, name: Rc<String>) -> Result<&ContextItem, CelError> {
        self.idents
            .get(name.as_ref())
            .ok_or_else(|| CelError::UnknownIdent(name.clone()))
    }

    pub fn add_variable(&mut self, name: impl Into<String>, value: impl Into<CelValue>) {
        self.idents
            .insert(name.into(), ContextItem::Value(value.into()));
    }
}
