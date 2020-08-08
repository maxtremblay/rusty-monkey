use super::*;
use super::{Node, Statement};

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

fn test_let_statement(statement: Statement, expected_identifier: String) {
    if let Statement::Let(statement) = statement {
        assert_eq!(statement.token_literal(), "let");
        assert_eq!(statement.identifier.token_literal(), expected_identifier)
    } else {
        panic!("not a let statement");
    }
}

#[test]
fn parsing_return_statements() {
    let parser = Parser::from(return_statements());
    let program: Vec<ParsingResult<Statement>> = parser.statements().collect();

    assert_eq!(program.len(), 4);

    assert_eq!(program[3], Err(ParsingError::ReachEndOfFile));

    program
        .into_iter()
        .take(3)
        .for_each(|statement| test_return_statement(statement.unwrap()));
}

fn return_statements() -> Lexer {
    let input = String::from(
        "
        return 5;
        return 10;
        return 838383;
        return
        ",
    );
    Lexer::from(input)
}

fn test_return_statement(statement: Statement) {
    if let Statement::Return(statement) = statement {
        assert_eq!(statement.token_literal(), "return");
    } else {
        panic!("not a return statement");
    }
}

#[test]
fn parsing_identifier_expression() {
    let parser = Parser::from(identifier_expression());
    let program: Vec<ParsingResult<Statement>> = parser.statements().collect();

    assert_eq!(program.len(), 1);
    let statement = program[0].as_ref().unwrap();
    test_identifier_expression(statement);
}

fn identifier_expression() -> Lexer {
    let input = String::from(
        "
        foobar;
        ",
    );
    Lexer::from(input)
}

fn test_identifier_expression(statement: &Statement) {
    if let Statement::Expression(expression) = statement {
        if let Expression::Identifier(identifier) = &expression.expression {
            assert_eq!(identifier.token_literal(), "foobar");
        } else {
            panic!("not an identifier");
        }
    } else {
        panic!("not an expression");
    }
}

#[test]
fn parsing_number_expression() {
    let parser = Parser::from(number_expression());
    let program: Vec<ParsingResult<Statement>> = parser.statements().collect();

    assert_eq!(program.len(), 1);
    let statement = program[0].as_ref().unwrap();
    test_number_expression(statement);
}

fn number_expression() -> Lexer {
    let input = String::from("5;");
    Lexer::from(input)
}

fn test_number_expression(statement: &Statement) {
    if let Statement::Expression(expression) = statement {
        if let Expression::IntegerLiteral(integer) = &expression.expression {
            assert_eq!(integer.token_literal(), "5");
        } else {
            panic!("not an number");
        }
    } else {
        panic!("not an expression");
    }
}
