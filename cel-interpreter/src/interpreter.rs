use cel_parser::{
    ast::{self, Expression},
    parser::ExpressionParser,
};

use std::rc::Rc;

use crate::{context::*, error::*, value::*};

pub struct CelExpression {
    source: String,
    expr: Expression,
}

impl CelExpression {
    pub fn evaluate(&self, ctx: &CelContext) -> Result<CelValue, CelError> {
        if let EvalType::Value(val) = evaluate_expression(&self.expr, ctx)? {
            Ok(val)
        } else {
            Err(CelError::Unexpected("evaluate didn't return a value"))
        }
    }
}

#[derive(Debug)]
enum EvalType<'a> {
    Value(CelValue),
    ContextItem(&'a ContextItem),
}

impl<'a> EvalType<'a> {
    fn try_bool(&self) -> Result<bool, CelError> {
        if let EvalType::Value(val) = self {
            val.try_bool()
        } else {
            Err(CelError::Unexpected("Expression didn't resolve to a bool"))
        }
    }
}

fn evaluate_expression<'a>(
    expr: &Expression,
    ctx: &'a CelContext,
) -> Result<EvalType<'a>, CelError> {
    use Expression::*;
    match expr {
        Ternary(cond, left, right) => {
            if evaluate_expression(cond, ctx)?.try_bool()? {
                evaluate_expression(left, ctx)
            } else {
                evaluate_expression(right, ctx)
            }
        }
        Member(expr, member) => {
            let ident = evaluate_expression(expr, ctx)?;
            evaluate_member(ident, member)
        }
        Literal(val) => Ok(EvalType::Value(CelValue::from(val))),
        Ident(name) => Ok(EvalType::ContextItem(ctx.lookup(Rc::clone(name))?)),
        _ => unimplemented!(),
    }
}

fn evaluate_member<'a>(target: EvalType, member: &ast::Member) -> Result<EvalType<'a>, CelError> {
    use ast::Member::*;
    match member {
        Attribute(name) => match target {
            EvalType::ContextItem(ContextItem::Value(CelValue::Map(map))) => {
                Ok(EvalType::Value(map.get(name)))
            }
            _ => Err(CelError::IllegalTarget(format!("{:?}", target))),
        },
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

    #[test]
    fn lookup() {
        let expression = "params.hello".parse::<CelExpression>().unwrap();
        let mut context = CelContext::new();
        let mut params = CelMap::new();
        params.insert("hello", 42);
        context.add_variable("params", params);
        assert_eq!(expression.evaluate(&context).unwrap(), CelValue::Int(42));
    }
}
