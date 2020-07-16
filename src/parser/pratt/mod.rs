use super::Expression;

pub type InfixParsingFunction = Box<dyn Fn(Expression) -> Expression>;
pub type PrefixParsingFunction = Box<dyn Fn() -> Expression>;
