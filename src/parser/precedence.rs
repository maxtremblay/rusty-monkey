#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Precedence {
    Lowest,
    Equals,
    LessAndGreater,
    Sum,
    Product,
    Prefix,
    Call,
}
