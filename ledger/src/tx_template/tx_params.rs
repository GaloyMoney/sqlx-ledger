use cel_interpreter::{CelContext, CelMap, CelValue};
use std::collections::HashMap;

use super::param_definition::{ParamDataType, ParamDefinition};
use crate::error::SqlxLedgerError;

pub struct TxParams {
    values: HashMap<String, CelValue>,
}

impl TxParams {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: impl Into<String>, v: impl Into<CelValue>) {
        self.values.insert(k.into(), v.into());
    }

    pub fn to_context(
        mut self,
        defs: Option<Vec<ParamDefinition>>,
    ) -> Result<CelContext, SqlxLedgerError> {
        let mut ctx = CelContext::new();
        if let Some(defs) = defs {
            let mut cel_map = CelMap::new();
            for d in defs {
                if let Some(v) = self.values.remove(&d.name) {
                    match ParamDataType::try_from(&v) {
                        Ok(t) if t == d.r#type => {
                            cel_map.insert(d.name, v);
                            continue;
                        }
                        _ => return Err(SqlxLedgerError::TxParamTypeMissmatch(d.r#type)),
                    }
                }
                if let Some(expr) = d.default_expr() {
                    cel_map.insert(d.name, expr.evaluate(&ctx)?);
                }
            }
            ctx.add_variable("params", cel_map);
        }

        if !self.values.is_empty() {
            return Err(SqlxLedgerError::TooManyParameters);
        }

        Ok(ctx)
    }
}

impl Default for TxParams {
    fn default() -> Self {
        Self::new()
    }
}
