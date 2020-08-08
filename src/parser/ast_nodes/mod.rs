use crate::lexer::Token;

mod macros;

use crate::create_node;

pub trait Node {
    fn token_literal(&self) -> &str;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Expression(ExpressionStatement),
    Let(LetStatement),
    Return(ReturnStatement),
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Expression(statement) => write!(f, "{}", statement),
            Self::Let(statement) => write!(f, "{}", statement),
            Self::Return(statement) => write!(f, "{}", statement),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Empty,
    Identifier(Identifier),
    Number(Number),
    PrefixOperator(PrefixOperator),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Expression[")?;
        match self {
            Self::Empty => {}
            expression => write!(f, "{}", expression)?,
        }
        write!(f, "]")
    }
}

create_node!(ExpressionStatement {
    expression: Expression,
});

impl std::fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Expression[{:?}]", self.expression)
    }
}

create_node!(Identifier);

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Identifier[{}]", self.token_literal())
    }
}

create_node!(Number { value: i64 });

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

create_node!(PrefixOperator {
    operator: String,
    right_expression: Box<Expression>,
});

impl std::fmt::Display for PrefixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}{})", self.operator, self.right_expression)
    }
}

create_node!(LetStatement {
    identifier: Identifier,
    expression: Expression,
});

impl std::fmt::Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {} = Expression[{:?}];",
            self.token_literal(),
            self.identifier,
            self.expression
        )
    }
}

create_node!(ReturnStatement {
    expression: Expression,
});

impl std::fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} Expression[{:?}];",
            self.token_literal(),
            self.expression
        )
    }
}
