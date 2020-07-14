use super::ast::{Node, Statement};
use super::*;

#[test]
fn parsing_let_statements() {
    let parser = Parser::from(let_statements());
    let program: Vec<ParsingResult<Statement>> = parser.statements().collect();

    assert_eq!(program.len(), 4);

    assert_eq!(program[3], Err(ParsingError::MissingIdentifier));

    program
        .into_iter()
        .zip(expected_identifiers_for_let_statements().into_iter())
        .for_each(|(statement, expected)| test_let_statement(statement.unwrap(), expected));
}

fn let_statements() -> Lexer {
    let input = String::from(
        "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        let = 12;
        ",
    );
    Lexer::from(input)
}

fn expected_identifiers_for_let_statements() -> Vec<String> {
    vec!["x".to_string(), "y".to_string(), "foobar".to_string()]
}

fn test_let_statement(Statement::Let(statement): Statement, expected_identifier: String) {
    assert_eq!(statement.token_literal(), "let");
    assert_eq!(statement.identifier.token_literal(), expected_identifier)
}
