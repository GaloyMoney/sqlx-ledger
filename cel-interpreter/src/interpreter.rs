use serde::{Deserialize, Serialize};

use cel_parser::{
    ast::{self, Expression},
    parser::ExpressionParser,
};

use std::rc::Rc;

use crate::{context::*, error::*, value::*};

#[derive(Clone, Deserialize, Serialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct CelExpression {
    source: String,
    expr: Expression,
}

impl CelExpression {
    pub fn try_evaluate<T: TryFrom<CelValue, Error = E>, E: From<CelError>>(
        &self,
        ctx: &CelContext,
    ) -> Result<T, E> {
        Ok(T::try_from(self.evaluate(ctx)?)?)
    }

    pub fn evaluate(&self, ctx: &CelContext) -> Result<CelValue, CelError> {
        if let EvalType::Value(val) = evaluate_expression(&self.expr, ctx)? {
            Ok(val)
        } else {
            Err(CelError::Unexpected(
                "evaluate didn't return a value".to_string(),
            ))
        }
    }
}

enum EvalType<'a> {
    Value(CelValue),
    ContextItem(&'a ContextItem),
}

impl<'a> EvalType<'a> {
    fn try_bool(&self) -> Result<bool, CelError> {
        if let EvalType::Value(val) = self {
            val.try_bool()
        } else {
            Err(CelError::Unexpected(
                "Expression didn't resolve to a bool".to_string(),
            ))
        }
    }

    fn try_key(&self) -> Result<CelKey, CelError> {
        if let EvalType::Value(val) = self {
            match val {
                CelValue::Int(i) => Ok(CelKey::Int(*i)),
                CelValue::UInt(u) => Ok(CelKey::UInt(*u)),
                CelValue::Bool(b) => Ok(CelKey::Bool(*b)),
                CelValue::String(s) => Ok(CelKey::String(s.clone())),
                _ => Err(CelError::Unexpected(
                    "Expression didn't resolve to a valid key".to_string(),
                )),
            }
        } else {
            Err(CelError::Unexpected(
                "Expression didn't resolve to value".to_string(),
            ))
        }
    }

    fn try_value(&self) -> Result<CelValue, CelError> {
        if let EvalType::Value(val) = self {
            Ok(val.clone())
        } else {
            Err(CelError::Unexpected("Couldn't unwrap value".to_string()))
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
            evaluate_member(ident, member, ctx)
        }
        Map(entries) => {
            let mut map = CelMap::new();
            for (k, v) in entries {
                let key = evaluate_expression(k, ctx)?;
                let value = evaluate_expression(v, ctx)?;
                map.insert(key.try_key()?, value.try_value()?)
            }
            Ok(EvalType::Value(CelValue::from(map)))
        }
        Ident(name) => Ok(EvalType::ContextItem(ctx.lookup(Rc::clone(name))?)),
        Literal(val) => Ok(EvalType::Value(CelValue::from(val))),
        e => Err(CelError::Unexpected(format!("unimplemented {e:?}"))),
    }
}

fn evaluate_member<'a>(
    target: EvalType,
    member: &ast::Member,
    ctx: &CelContext,
) -> Result<EvalType<'a>, CelError> {
    use ast::Member::*;
    match member {
        Attribute(name) => match target {
            EvalType::ContextItem(ContextItem::Value(CelValue::Map(map))) => {
                Ok(EvalType::Value(map.get(name)))
            }
            _ => Err(CelError::IllegalTarget),
        },
        FunctionCall(exprs) => match target {
            EvalType::ContextItem(ContextItem::Function(f)) => {
                let mut args = Vec::new();
                for e in exprs {
                    args.push(evaluate_expression(e, ctx)?.try_value()?)
                }
                Ok(EvalType::Value(f(args)?))
            }
            _ => Err(CelError::IllegalTarget),
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
    use chrono::NaiveDate;

    #[test]
    fn literals() {
        let expression = "true".parse::<CelExpression>().unwrap();
        let context = CelContext::new();
        assert_eq!(expression.evaluate(&context).unwrap(), CelValue::Bool(true));

        let expression = "1".parse::<CelExpression>().unwrap();
        assert_eq!(expression.evaluate(&context).unwrap(), CelValue::Int(1));

        let expression = "-1".parse::<CelExpression>().unwrap();
        assert_eq!(expression.evaluate(&context).unwrap(), CelValue::Int(-1));

        let expression = "'hello'".parse::<CelExpression>().unwrap();
        assert_eq!(
            expression.evaluate(&context).unwrap(),
            CelValue::String("hello".to_string().into())
        );

        // Tokenizer needs fixing
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

    #[test]
    fn function() {
        let expression = "date('2022-10-10')".parse::<CelExpression>().unwrap();
        let context = CelContext::new();
        assert_eq!(
            expression.evaluate(&context).unwrap(),
            CelValue::Date(NaiveDate::parse_from_str("2022-10-10", "%Y-%m-%d").unwrap())
        );
    }
}
