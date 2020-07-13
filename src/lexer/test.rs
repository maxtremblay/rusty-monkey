use super::*;

#[test]
fn lexer_can_read_tokens() {
    let lexer = Lexer::from(input());
    lexer
        .tokens()
        .zip(expected_tokens().iter())
        .for_each(|(token, expected)| assert_eq!(&token, expected));
}

fn input() -> String {
    String::from(
        "
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        ",
    )
}

fn expected_tokens() -> Vec<Token> {
    use tokens::kind::Delimiter::*;
    use tokens::kind::Keyword::*;
    use tokens::kind::Literal::*;
    use tokens::kind::Operator::*;
    vec![
        Token {
            kind: TokenKind::Keyword(Let),
            literal: String::from("let"),
        },
        Token {
            kind: TokenKind::Identifier,
            literal: String::from("five"),
        },
        Token {
            kind: TokenKind::Operator(Assignment),
            literal: String::from("="),
        },
        Token {
            kind: TokenKind::Literal(Integer),
            literal: String::from("5"),
        },
        Token {
            kind: TokenKind::Delimiter(SemiColon),
            literal: String::from(";"),
        },
        Token {
            kind: TokenKind::Keyword(Let),
            literal: String::from("let"),
        },
        Token {
            kind: TokenKind::Identifier,
            literal: String::from("ten"),
        },
        Token {
            kind: TokenKind::Operator(Assignment),
            literal: String::from("="),
        },
        Token {
            kind: TokenKind::Literal(Integer),
            literal: String::from("10"),
        },
        Token {
            kind: TokenKind::Delimiter(SemiColon),
            literal: String::from(";"),
        },
        Token {
            kind: TokenKind::Keyword(Let),
            literal: String::from("let"),
        },
        Token {
            kind: TokenKind::Identifier,
            literal: String::from("add"),
        },
        Token {
            kind: TokenKind::Operator(Assignment),
            literal: String::from("="),
        },
        Token {
            kind: TokenKind::Keyword(Function),
            literal: String::from("fn"),
        },
        Token {
            kind: TokenKind::Delimiter(LeftParenthesis),
            literal: String::from("("),
        },
        Token {
            kind: TokenKind::Identifier,
            literal: String::from("x"),
        },
        Token {
            kind: TokenKind::Delimiter(Comma),
            literal: String::from(","),
        },
        Token {
            kind: TokenKind::Identifier,
            literal: String::from("y"),
        },
        Token {
            kind: TokenKind::Delimiter(RightParenthesis),
            literal: String::from(")"),
        },
        Token {
            kind: TokenKind::Delimiter(LeftBrace),
            literal: String::from("{"),
        },
        Token {
            kind: TokenKind::Identifier,
            literal: String::from("x"),
        },
        Token {
            kind: TokenKind::Operator(Plus),
            literal: String::from("+"),
        },
        Token {
            kind: TokenKind::Identifier,
            literal: String::from("y"),
        },
        Token {
            kind: TokenKind::Delimiter(SemiColon),
            literal: String::from(";"),
        },
        Token {
            kind: TokenKind::Keyword(Let),
            literal: String::from("let"),
        },
        Token {
            kind: TokenKind::Identifier,
            literal: String::from("result"),
        },
        Token {
            kind: TokenKind::Operator(Assignment),
            literal: String::from("="),
        },
        Token {
            kind: TokenKind::Identifier,
            literal: String::from("add"),
        },
        Token {
            kind: TokenKind::Delimiter(LeftParenthesis),
            literal: String::from("("),
        },
        Token {
            kind: TokenKind::Identifier,
            literal: String::from("five"),
        },
        Token {
            kind: TokenKind::Delimiter(Comma),
            literal: String::from(","),
        },
        Token {
            kind: TokenKind::Identifier,
            literal: String::from("ten"),
        },
        Token {
            kind: TokenKind::Delimiter(RightParenthesis),
            literal: String::from(")"),
        },
        Token {
            kind: TokenKind::Delimiter(SemiColon),
            literal: String::from(";"),
        },
    ]
}
