extern crate anyhow;
extern crate pest;
use self::anyhow::{anyhow, Context, Result};
use pest::iterators::Pair;
use pest::Parser;

use mallardscript::ast;
use mallardscript::grammar;

/// Parse provided MallardScript input into it's AST equivalent.
pub fn parse_document(input: String) -> Result<Vec<ast::Statement>> {
    let pairs = grammar::ParserMalardScript::parse(grammar::Rule::document, &input)
        .with_context(|| "Unable to parse provided document.")?
        .next()
        .unwrap()
        .into_inner();

    let mut statements: Vec<ast::Statement> = Vec::new();
    for pair in pairs {
        statements
            .push(parse_statement(pair).with_context(|| "Unable to parse provided statement.")?);
    }

    Ok(statements)
}

/// Parse a PEG pair statement into it's AST equivalent.
pub fn parse_statement(pair: Pair<grammar::Rule>) -> Result<ast::Statement> {
    return match pair.as_rule() {
        grammar::Rule::statement_variable_assignment => {
            let mut inner_pairs = pair.into_inner();
            let keyword_name_variable: Pair<grammar::Rule> = inner_pairs.next().unwrap();
            let value_variable: Pair<grammar::Rule> = inner_pairs.next().unwrap();
            return Ok(ast::Statement::VariableAssignment(
                ast::StatementVariableAssignment {
                    name: String::from(keyword_name_variable.as_str()),
                    assignment: String::from(value_variable.as_str()),
                },
            ));
        }

        grammar::Rule::statement_command => {
            let mut inner_pairs = pair.into_inner();
            let keyword_command: Pair<grammar::Rule> = inner_pairs.next().unwrap();
            let keyword_command_value: Pair<grammar::Rule> = inner_pairs.next().unwrap();
            return Ok(ast::Statement::Command(ast::StatementCommand {
                name: String::from(keyword_command.as_str()),
                value: String::from(keyword_command_value.as_str()),
            }));
        }

        grammar::Rule::statement_variable_declaration => {
            let mut inner_pairs = pair.into_inner();
            let keyword_name_variable: Pair<grammar::Rule> = inner_pairs.next().unwrap();
            let value_variable: Pair<grammar::Rule> = inner_pairs.next().unwrap();
            return Ok(ast::Statement::VariableDeclaration(
                ast::StatementVariableDeclaration {
                    name: String::from(keyword_name_variable.as_str()),
                    assignment: String::from(value_variable.as_str()),
                },
            ));
        }

        grammar::Rule::EOI => {
            return Ok(ast::Statement::End(ast::StatementEnd {}));
        }

        _ => Err(anyhow!("Provided pair was not a valid statement.")),
    };
}
