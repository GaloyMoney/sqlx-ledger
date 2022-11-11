use crate::error::*;
use cel_interpreter::{context::Context, Program};
use std::rc::Rc;

pub enum CelValue {
    Int(i32),
    UInt(u32),
    Float(f64),
    String(Rc<String>),
    Bytes(Rc<Vec<u8>>),
    Bool(bool),
    Null,
}

pub struct Params {
    inner: Context,
}

pub struct CelExpression {
    source: String,
    expression: Program,
}

impl CelExpression {
    fn evaluate(&self, params: Params) -> Result<CelValue, CelError> {
        unimplemented!()
    }
}
impl TryFrom<String> for CelExpression {
    type Error = CelError;

    fn try_from(source: String) -> Result<Self, Self::Error> {
        let expression = Program::compile(&source)?;
        Ok(Self { source, expression })
    }
}

impl std::str::FromStr for CelExpression {
    type Err = CelError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        CelExpression::try_from(source.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds() {
        // let cel_expression = CelExpression {
        //     expression: "expression".to_string(),
        // };
        // assert_eq!(cel_expression.expression, "expression");
    }
}
