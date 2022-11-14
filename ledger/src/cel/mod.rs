use cel_interpreter::{context::Context, objects::CelType};
use cel_parser::{ast::Expression, parser::ExpressionParser};
use serde::{Deserialize, Serialize};

use crate::error::*;

pub struct CelContext {
    inner: Context,
}
impl Default for CelContext {
    fn default() -> Self {
        Self {
            inner: Context::default(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct CelExpression {
    source: String,
    expr: Expression,
}

impl CelExpression {
    pub fn evaluate(&self, ctx: &CelContext) -> CelType {
        CelType::resolve(&self.expr, &ctx.inner)
    }
}

impl From<CelExpression> for String {
    fn from(expr: CelExpression) -> Self {
        expr.source
    }
}

impl TryFrom<String> for CelExpression {
    type Error = CelError;

    fn try_from(source: String) -> Result<Self, Self::Error> {
        let expr = ExpressionParser::new()
            .parse(&source)
            .map_err(|e| CelError::CelParseError(e.to_string()))?;
        Ok(Self { source, expr })
    }
}
impl TryFrom<&str> for CelExpression {
    type Error = CelError;

    fn try_from(source: &str) -> Result<Self, Self::Error> {
        Self::try_from(source.to_string())
    }
}
impl std::str::FromStr for CelExpression {
    type Err = CelError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Self::try_from(source.to_string())
    }
}
