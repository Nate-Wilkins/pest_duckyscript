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

/// Parse a PEG pair REM statement.
pub fn parse_statement_command_rem(pair: Pair<grammar::Rule>) -> Result<ast::Statement> {
    let mut inner_pairs = pair.into_inner();
    let keyword_command_value: Pair<grammar::Rule> = inner_pairs.next().unwrap();
    Ok(ast::Statement::CommandRem(ast::StatementCommandRem {
        value: String::from(keyword_command_value.as_str()),
    }))
}

/// Parse a PEG pair DEFAULTDELAY statement.
pub fn parse_statement_command_defaultdelay(pair: Pair<grammar::Rule>) -> Result<ast::Statement> {
    let mut inner_pairs = pair.into_inner();
    let keyword_command_value: Pair<grammar::Rule> = inner_pairs.next().unwrap();
    Ok(ast::Statement::CommandDefaultDelay(
        ast::StatementCommandDefaultDelay {
            value: String::from(keyword_command_value.as_str()),
        },
    ))
}

/// Parse a PEG pair DELAY statement.
pub fn parse_statement_command_delay(pair: Pair<grammar::Rule>) -> Result<ast::Statement> {
    let mut inner_pairs = pair.into_inner();
    let keyword_command_value: Pair<grammar::Rule> = inner_pairs.next().unwrap();
    Ok(ast::Statement::CommandDelay(ast::StatementCommandDelay {
        value: String::from(keyword_command_value.as_str()),
    }))
}

/// Parse a PEG pair STRINGLN statement.
pub fn parse_statement_command_stringln(pair: Pair<grammar::Rule>) -> Result<ast::Statement> {
    let mut inner_pairs = pair.into_inner();
    let keyword_command_value: Pair<grammar::Rule> = inner_pairs.next().unwrap();
    Ok(ast::Statement::CommandStringln(
        ast::StatementCommandStringln {
            value: String::from(keyword_command_value.as_str()),
        },
    ))
}

/// Parse a PEG pair STRING statement.
pub fn parse_statement_command_string(pair: Pair<grammar::Rule>) -> Result<ast::Statement> {
    let mut inner_pairs = pair.into_inner();
    let keyword_command_value: Pair<grammar::Rule> = inner_pairs.next().unwrap();
    Ok(ast::Statement::CommandString(ast::StatementCommandString {
        value: String::from(keyword_command_value.as_str()),
    }))
}

/// Parse a PEG pair DEFINE statement.
pub fn parse_statement_command_define(pair: Pair<grammar::Rule>) -> Result<ast::Statement> {
    let mut inner_pairs = pair.into_inner();
    let keyword_name: Pair<grammar::Rule> = inner_pairs.next().unwrap();
    let value: Pair<grammar::Rule> = inner_pairs.next().unwrap();

    Ok(ast::Statement::CommandDefine(ast::StatementCommandDefine {
        name: String::from(keyword_name.as_str()),
        value: String::from(value.as_str()),
    }))
}

/// Parse a PEG pair EXFIL statement.
pub fn parse_statement_command_exfil(pair: Pair<grammar::Rule>) -> Result<ast::Statement> {
    let mut inner_pairs = pair.into_inner();
    let keyword_name: Pair<grammar::Rule> = inner_pairs.next().unwrap();

    Ok(ast::Statement::CommandExfil(ast::StatementCommandExfil {
        name: String::from(keyword_name.as_str()),
    }))
}

/// Parse a PEG pair command key statement.
pub fn parse_statement_command_key(pair: Pair<grammar::Rule>) -> Result<ast::Statement> {
    let keyword_command_key_inner_pairs = pair.into_inner();

    // Collect additional keys.
    let mut keyword_command_key_statements: Vec<ast::Statement> = vec![];
    let mut keyword_command_key_remaining_keys = "";
    // Is there an additional statement command key?
    for keyword_command_additional in keyword_command_key_inner_pairs {
        if keyword_command_additional.as_rule() == grammar::Rule::value_any {
            // We've reached the end of the initial command key.
            // Collect what's left.
            keyword_command_key_remaining_keys = keyword_command_additional.as_str();
        } else {
            // Collect our statement command key.
            let statement_command_keyword = parse_statement(keyword_command_additional)
                .with_context(|| "Unable to parse command key statement.")
                .unwrap();
            keyword_command_key_statements.push(statement_command_keyword);
        }
    }

    Ok(ast::Statement::CommandKey(ast::StatementCommandKey {
        statements: keyword_command_key_statements,
        remaining_keys: String::from(keyword_command_key_remaining_keys),
    }))
}

/// Parse a PEG pair command key value statement.
pub fn parse_statement_command_key_value(pair: Pair<grammar::Rule>) -> Result<ast::Statement> {
    let keyword_command_key_name = String::from(pair.as_str());

    Ok(ast::Statement::CommandKeyValue(
        ast::StatementCommandKeyValue {
            name: keyword_command_key_name,
        },
    ))
}

/// Parse a PEG pair single command statement.
pub fn parse_statement_single_command(pair: Pair<grammar::Rule>) -> Result<ast::Statement> {
    Ok(ast::Statement::SingleCommand(ast::StatementSingleCommand {
        name: String::from(pair.as_str()),
    }))
}

/// Parse a PEG pair variable assignment statement.
pub fn parse_statement_command_variable_assignment(
    pair: Pair<grammar::Rule>,
) -> Result<ast::Statement> {
    let mut inner_pairs = pair.into_inner();
    let keyword_name_variable: Pair<grammar::Rule> = inner_pairs.next().unwrap();
    let value_variable: Pair<grammar::Rule> = inner_pairs.next().unwrap();
    Ok(ast::Statement::VariableAssignment(
        ast::StatementVariableAssignment {
            name: String::from(keyword_name_variable.as_str()),
            assignment: String::from(value_variable.as_str()),
        },
    ))
}

/// Parse a PEG pair variable declaration statement.
pub fn parse_statement_command_variable_declaration(
    pair: Pair<grammar::Rule>,
) -> Result<ast::Statement> {
    let mut inner_pairs = pair.into_inner();
    let keyword_name_variable: Pair<grammar::Rule> = inner_pairs.next().unwrap();
    let value_variable: Pair<grammar::Rule> = inner_pairs.next().unwrap();
    Ok(ast::Statement::VariableDeclaration(
        ast::StatementVariableDeclaration {
            name: String::from(keyword_name_variable.as_str()),
            assignment: String::from(value_variable.as_str()),
        },
    ))
}

/// Parse a PEG pair IMPORT statement.
pub fn parse_statement_command_import(pair: Pair<grammar::Rule>) -> Result<ast::Statement> {
    let mut inner_pairs = pair.into_inner();
    let keyword_command_value = inner_pairs.next().unwrap().as_str();
    Ok(ast::Statement::CommandImport(ast::StatementCommandImport {
        value: String::from(keyword_command_value),
    }))
}

/// Parse a PEG pair statement into it's AST equivalent.
pub fn parse_statement(pair: Pair<grammar::Rule>) -> Result<ast::Statement> {
    return match pair.as_rule() {
        grammar::Rule::statement_command_rem => parse_statement_command_rem(pair),
        grammar::Rule::statement_command_defaultdelay => parse_statement_command_defaultdelay(pair),
        grammar::Rule::statement_command_delay => parse_statement_command_delay(pair),
        grammar::Rule::statement_command_stringln => parse_statement_command_stringln(pair),
        grammar::Rule::statement_command_string => parse_statement_command_string(pair),
        grammar::Rule::statement_command_define => parse_statement_command_define(pair),
        grammar::Rule::statement_command_exfil => parse_statement_command_exfil(pair),
        grammar::Rule::statement_variable_assignment => {
            parse_statement_command_variable_assignment(pair)
        }
        grammar::Rule::statement_variable_declaration => {
            parse_statement_command_variable_declaration(pair)
        }
        grammar::Rule::statement_command_key => parse_statement_command_key(pair),
        grammar::Rule::statement_command_backspace
        | grammar::Rule::statement_command_delete
        | grammar::Rule::statement_command_del
        | grammar::Rule::statement_command_downarrow
        | grammar::Rule::statement_command_down
        | grammar::Rule::statement_command_end
        | grammar::Rule::statement_command_home
        | grammar::Rule::statement_command_insert
        | grammar::Rule::statement_command_leftarrow
        | grammar::Rule::statement_command_left
        | grammar::Rule::statement_command_pagedown
        | grammar::Rule::statement_command_pageup
        | grammar::Rule::statement_command_rightarrow
        | grammar::Rule::statement_command_right
        | grammar::Rule::statement_command_space
        | grammar::Rule::statement_command_tab
        | grammar::Rule::statement_command_uparrow
        | grammar::Rule::statement_command_up
        | grammar::Rule::statement_command_app
        | grammar::Rule::statement_command_break
        | grammar::Rule::statement_command_enter
        | grammar::Rule::statement_command_escape
        | grammar::Rule::statement_command_f10
        | grammar::Rule::statement_command_f11
        | grammar::Rule::statement_command_f12
        | grammar::Rule::statement_command_f0
        | grammar::Rule::statement_command_f1
        | grammar::Rule::statement_command_f2
        | grammar::Rule::statement_command_f3
        | grammar::Rule::statement_command_f4
        | grammar::Rule::statement_command_f5
        | grammar::Rule::statement_command_f6
        | grammar::Rule::statement_command_f7
        | grammar::Rule::statement_command_f8
        | grammar::Rule::statement_command_f9
        | grammar::Rule::statement_command_menu
        | grammar::Rule::statement_command_pause
        | grammar::Rule::statement_command_printscreen
        | grammar::Rule::statement_command_alt
        | grammar::Rule::statement_command_command
        | grammar::Rule::statement_command_control
        | grammar::Rule::statement_command_ctrl
        | grammar::Rule::statement_command_gui
        | grammar::Rule::statement_command_shift
        | grammar::Rule::statement_command_windows
        | grammar::Rule::statement_command_option
        | grammar::Rule::statement_command_capslock
        | grammar::Rule::statement_command_numlock
        | grammar::Rule::statement_command_scrollock => parse_statement_command_key_value(pair),
        grammar::Rule::statement_command_inject_mod
        | grammar::Rule::statement_command_wait_for_button_press
        | grammar::Rule::statement_command_button_def
        | grammar::Rule::statement_command_disable_button
        | grammar::Rule::statement_command_enable_button
        | grammar::Rule::statement_command_led_off
        | grammar::Rule::statement_command_led_r
        | grammar::Rule::statement_command_led_g
        | grammar::Rule::statement_command_save_attackmode
        | grammar::Rule::statement_command_restore_attackmode
        | grammar::Rule::statement_command_attackmode
        | grammar::Rule::statement_command_random_lowercase_letter
        | grammar::Rule::statement_command_random_uppercase_letter
        | grammar::Rule::statement_command_random_letter
        | grammar::Rule::statement_command_random_number
        | grammar::Rule::statement_command_random_special
        | grammar::Rule::statement_command_random_char
        | grammar::Rule::statement_command_vid_random
        | grammar::Rule::statement_command_pid_random
        | grammar::Rule::statement_command_man_random
        | grammar::Rule::statement_command_prod_random
        | grammar::Rule::statement_command_serial_random
        | grammar::Rule::statement_command_hold
        | grammar::Rule::statement_command_release
        | grammar::Rule::statement_command_restart_payload
        | grammar::Rule::statement_command_stop_payload
        | grammar::Rule::statement_command_reset
        | grammar::Rule::statement_command_hide_payload
        | grammar::Rule::statement_command_restore_payload
        | grammar::Rule::statement_command_wait_for_caps_on
        | grammar::Rule::statement_command_wait_for_caps_off
        | grammar::Rule::statement_command_wait_for_caps_change
        | grammar::Rule::statement_command_wait_for_num_on
        | grammar::Rule::statement_command_wait_for_num_off
        | grammar::Rule::statement_command_wait_for_num_change
        | grammar::Rule::statement_command_wait_for_scroll_on
        | grammar::Rule::statement_command_wait_for_scroll_off
        | grammar::Rule::statement_command_wait_for_scroll_change
        | grammar::Rule::statement_command_save_host_keyboard_lock_state
        | grammar::Rule::statement_command_restore_host_keyboard_lock_state => {
            parse_statement_single_command(pair)
        }
        grammar::Rule::statement_command_import => parse_statement_command_import(pair),
        grammar::Rule::EOI => Ok(ast::Statement::End(ast::StatementEnd {})),
        _ => Err(anyhow!(
            "Provided pair was not a valid statement.\n{:#?}",
            pair
        )),
    };
}
