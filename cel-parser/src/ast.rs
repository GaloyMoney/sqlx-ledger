use std::rc::Rc;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LogicOp {
    And,
    Or,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum RelationOp {
    LessThan,
    LessThanEq,
    GreaterThan,
    GreaterThanEq,
    Equals,
    NotEquals,
    In,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ArithmeticOp {
    Add,
    Subtract,
    Divide,
    Multiply,
    Modulus,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum UnaryOp {
    Not,
    DoubleNot,
    Minus,
    DoubleMinus,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LeftRightOp {
    Logic(LogicOp),
    Relation(RelationOp),
    Arithmetic(ArithmeticOp),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Ternary(Box<Expression>, Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Relation(RelationOp, Box<Expression>, Box<Expression>),
    Arithmetic(ArithmeticOp, Box<Expression>, Box<Expression>),
    Unary(UnaryOp, Box<Expression>),

    Member(Box<Expression>, Box<Member>),

    List(Vec<Expression>),
    Map(Vec<(Expression, Expression)>),
    Struct(Vec<Rc<String>>, Vec<(Rc<String>, Expression)>),

    Literal(Literal),
    Ident(Rc<String>),
}

impl Expression {
    pub(crate) fn from_op(op: LeftRightOp, left: Box<Expression>, right: Box<Expression>) -> Self {
        use LeftRightOp::*;
        match op {
            Logic(LogicOp::Or) => Expression::Or(left, right),
            Logic(LogicOp::And) => Expression::And(left, right),
            Relation(op) => Expression::Relation(op, left, right),
            Arithmetic(op) => Expression::Arithmetic(op, left, right),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Member {
    Attribute(Rc<String>),
    FunctionCall(Vec<Expression>),
    Index(Box<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Int(i64),
    UInt(u64),
    Double(f64),
    String(Rc<String>),
    Bytes(Rc<Vec<u8>>),
    Bool(bool),
    Null,
}

#[cfg(test)]
mod tests {
    use crate::parser::ExpressionParser;
    use crate::{ArithmeticOp::*, Expression, Expression::*, Literal::*, Member::*};

    fn parse(input: &str) -> Expression {
        ExpressionParser::new()
            .parse(input)
            .unwrap_or_else(|e| panic!("{}", e))
    }

    fn assert_parse_eq(input: &str, expected: Expression) {
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn logic() {
        assert_parse_eq(
            "true || false && true",
            Or(
                Literal(Bool(true)).into(),
                And(Literal(Bool(false)).into(), Literal(Bool(true)).into()).into(),
            ),
        )
    }
    #[test]
    fn op_precendence() {
        assert_parse_eq(
            "1 + 2 * 3",
            Arithmetic(
                Add,
                Literal(Int(1)).into(),
                Arithmetic(Multiply, Literal(Int(2)).into(), Literal(Int(3)).into()).into(),
            )
            .into(),
        );
        assert_parse_eq(
            "1 * 2 + 3",
            Arithmetic(
                Add,
                Arithmetic(Multiply, Literal(Int(1)).into(), Literal(Int(2)).into()).into(),
                Literal(Int(3)).into(),
            )
            .into(),
        );
        assert_parse_eq(
            "1 * (2 + 3)",
            Arithmetic(
                Multiply,
                Literal(Int(1)).into(),
                Arithmetic(Add, Literal(Int(2)).into(), Literal(Int(3)).into()).into(),
            )
            .into(),
        )
    }

    #[test]
    fn simple_int() {
        assert_parse_eq("1", Literal(Int(1)))
    }

    #[test]
    fn simple_float() {
        assert_parse_eq("1.0", Literal(Double(1.0)))
    }

    #[test]
    fn nested_attributes() {
        assert_parse_eq(
            "a.b[1]",
            Member(
                Member(
                    Ident("a".to_string().into()).into(),
                    Attribute("b".to_string().into()).into(),
                )
                .into(),
                Index(Literal(Int(1)).into()).into(),
            )
            .into(),
        )
    }
}
