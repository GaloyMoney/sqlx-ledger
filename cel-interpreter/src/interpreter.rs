use cel_parser::{ast::Expression, parser::ExpressionParser};

use crate::{context::*, error::*, value::*};

pub struct CelExpression {
    source: String,
    expr: Expression,
}

impl CelExpression {
    pub fn evaluate(&self, ctx: &CelContext) -> Result<CelValue, CelError> {
        Ok(evaluate_expression(&self.expr, ctx)?)
    }
}

fn evaluate_expression(expr: &Expression, ctx: &CelContext) -> Result<CelValue, CelError> {
    use Expression::*;
    match expr {
        Ternary(cond, left, right) => {
            if evaluate_expression(cond, ctx)?.try_bool()? {
                evaluate_expression(left, ctx)
            } else {
                evaluate_expression(right, ctx)
            }
        }
        Literal(val) => Ok(CelValue::from(val)),
        _ => unimplemented!(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literals() {
        let expression = "true".parse::<CelExpression>().unwrap();
        let context = CelContext::new();
        assert_eq!(expression.evaluate(&context).unwrap(), CelValue::Bool(true));

        let expression = "1".parse::<CelExpression>().unwrap();
        assert_eq!(expression.evaluate(&context).unwrap(), CelValue::Int(1));

        let expression = "-1".parse::<CelExpression>().unwrap();
        assert_eq!(expression.evaluate(&context).unwrap(), CelValue::Int(-1));

        // Tokenizer needs rewriting
        // let expression = "'hello'".parse::<CelExpression>().unwrap();
        // assert_eq!(
        //     expression.evaluate(&context).unwrap(),
        //     CelValue::String("hello".to_string().into())
        // );

        // Tokenizer needs rewriting
        // let expression = "1u".parse::<CelExpression>().unwrap();
        // assert_eq!(expression.evaluate(&context).unwrap(), CelValue::UInt(1))
    }

    #[test]
    fn logic() {
        let expression = "true || false ? false && true : true"
            .parse::<CelExpression>()
            .unwrap();
        let context = CelContext::new();
        assert_eq!(
            expression.evaluate(&context).unwrap(),
            CelValue::Bool(false)
        );
        let expression = "true && false ? false : true || false"
            .parse::<CelExpression>()
            .unwrap();
        assert_eq!(expression.evaluate(&context).unwrap(), CelValue::Bool(true))
    }
}
