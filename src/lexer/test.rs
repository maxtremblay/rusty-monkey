use super::*;

#[test]
fn lexer_can_read_delimiter() {
    let lexer = Lexer::from(input());

    lexer
        .tokens()
        .zip(expected_tokens().iter())
        .for_each(|(token, expected)| assert_eq!(&token, expected));
}

fn input() -> String {
    String::from("(){},;")
}

fn expected_tokens() -> Vec<Token> {
    use tokens::kind::Delimiter::*;
    vec![
        Token {
            kind: TokenKind::Delimiter(LeftParenthesis),
            literal: String::from("("),
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
            kind: TokenKind::Delimiter(RightBrace),
            literal: String::from("}"),
        },
        Token {
            kind: TokenKind::Delimiter(Comma),
            literal: String::from(","),
        },
        Token {
            kind: TokenKind::Delimiter(SemiColon),
            literal: String::from(";"),
        },
    ]
}
