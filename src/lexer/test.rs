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
    String::from("=+(){},;")
}

fn expected_tokens() -> Vec<Token> {
    vec![
        Token {
            kind: TokenKind::Assignment,
            literal: String::from("="),
        },
        Token {
            kind: TokenKind::Plus,
            literal: String::from("+"),
        },
        Token {
            kind: TokenKind::LeftParenthesis,
            literal: String::from("("),
        },
        Token {
            kind: TokenKind::RightParenthesis,
            literal: String::from(")"),
        },
        Token {
            kind: TokenKind::LeftBrace,
            literal: String::from("{"),
        },
        Token {
            kind: TokenKind::RightBrace,
            literal: String::from("}"),
        },
        Token {
            kind: TokenKind::Comma,
            literal: String::from(","),
        },
        Token {
            kind: TokenKind::Semicolon,
            literal: String::from(";"),
        },
    ]
}
