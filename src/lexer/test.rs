use super::*;

#[test]
fn lexer_can_read_simple_program() {
    test_lexer(simple_program(), expected_tokens_for_simple_program());
}

fn test_lexer(input: String, expected_tokens: Vec<Token>) {
    let lexer = Lexer::from(input);
    lexer
        .tokens()
        .zip(expected_tokens.iter())
        .for_each(|(token, expected)| assert_eq!(&token, expected));
}

fn simple_program() -> String {
    String::from(
        "
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);

        !-/*5;
        5 < 10 > 6;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        ",
    )
}

fn expected_tokens_for_simple_program() -> Vec<Token> {
    vec![
        Token::from_keyword("let".to_string()),
        Token::from_identifier("five".to_string()),
        Token::from_operator("=".to_string()),
        Token::from_number("5".to_string()),
        Token::from_delimiter(';'),
        Token::from_keyword("let".to_string()),
        Token::from_identifier("ten".to_string()),
        Token::from_operator("=".to_string()),
        Token::from_number("10".to_string()),
        Token::from_delimiter(';'),
        Token::from_keyword("let".to_string()),
        Token::from_identifier("add".to_string()),
        Token::from_operator("=".to_string()),
        Token::from_keyword("fn".to_string()),
        Token::from_delimiter('('),
        Token::from_identifier("x".to_string()),
        Token::from_delimiter(','),
        Token::from_identifier("y".to_string()),
        Token::from_delimiter(')'),
        Token::from_delimiter('{'),
        Token::from_identifier("x".to_string()),
        Token::from_operator("+".to_string()),
        Token::from_identifier("y".to_string()),
        Token::from_delimiter(';'),
        Token::from_delimiter('}'),
        Token::from_delimiter(';'),
        Token::from_keyword("let".to_string()),
        Token::from_identifier("result".to_string()),
        Token::from_operator("=".to_string()),
        Token::from_identifier("add".to_string()),
        Token::from_delimiter('('),
        Token::from_identifier("five".to_string()),
        Token::from_delimiter(','),
        Token::from_identifier("ten".to_string()),
        Token::from_delimiter(')'),
        Token::from_delimiter(';'),
        Token::from_operator("!".to_string()),
        Token::from_operator("-".to_string()),
        Token::from_operator("/".to_string()),
        Token::from_operator("*".to_string()),
        Token::from_number("5".to_string()),
        Token::from_delimiter(';'),
        Token::from_number("5".to_string()),
        Token::from_operator("<".to_string()),
        Token::from_number("10".to_string()),
        Token::from_operator(">".to_string()),
        Token::from_number("6".to_string()),
        Token::from_delimiter(';'),
        Token::from_keyword("if".to_string()),
        Token::from_delimiter('('),
        Token::from_number("5".to_string()),
        Token::from_operator("<".to_string()),
        Token::from_number("10".to_string()),
        Token::from_delimiter(')'),
        Token::from_delimiter('{'),
        Token::from_keyword("return".to_string()),
        Token::from_keyword("true".to_string()),
        Token::from_delimiter(';'),
        Token::from_delimiter('}'),
        Token::from_keyword("else".to_string()),
        Token::from_delimiter('{'),
        Token::from_keyword("return".to_string()),
        Token::from_keyword("false".to_string()),
        Token::from_delimiter(';'),
        Token::from_delimiter('}'),
        Token::from_number("10".to_string()),
        Token::from_operator("==".to_string()),
        Token::from_number("10".to_string()),
        Token::from_delimiter(';'),
        Token::from_number("10".to_string()),
        Token::from_operator("!=".to_string()),
        Token::from_number("9".to_string()),
        Token::from_delimiter(';'),
    ]
}
