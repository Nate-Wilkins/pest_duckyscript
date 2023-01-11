extern crate pest_duckyscript;
extern crate predicates;
extern crate pretty_assertions;

use pest_duckyscript::mallardscript::{
    ast::{
        Statement, StatementBlockIf, StatementBlockWhile, StatementCommandDefaultDelay,
        StatementCommandDefine, StatementCommandDelay, StatementCommandExfil,
        StatementCommandImport, StatementCommandKey, StatementCommandKeyValue, StatementCommandRem,
        StatementCommandString, StatementCommandStringln, StatementEnd, StatementSingleCommand,
        StatementVariableAssignment, StatementVariableDeclaration,
    },
    parser::parse_document,
};
use pretty_assertions::assert_eq;

/// Test valid DuckyScript input.
macro_rules! test_parser_input_valid {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() -> Result<(), Box<dyn std::error::Error>> {
            // Given a DuckyScript parser.
            // And a DuckyScript script input that is valid.
            let (input, expected_statements) = $value;

            // When parsing the script.
            let output = parse_document(String::from(input))?;

            // Then the parse was successfull.
            assert_eq!(output, expected_statements);

            return Ok(());
        }
    )*
    }
}

/// Test invalid DuckyScript input.
macro_rules! test_parser_input_invalid {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() -> Result<(), Box<dyn std::error::Error>> {
            // Given a DuckyScript parser.
            // And a DuckyScript script input that is invalid.
            let (input, expected_error) = $value;

            // When parsing the script.
            let output = parse_document(String::from(input))
                .err()
                .unwrap();

            // Then the parse was not successfull.
            assert_eq!(format!("{:?}", output), expected_error);

            return Ok(());
        }
    )*
    }
}

test_parser_input_valid! {
    // Empty.
    test_parser_input_valid_empty: (
r#""#, vec![
        Statement::End(StatementEnd {}),
    ]),

    // Delays.
        // DELAY
        test_parser_input_valid_command_delay: (
            r#"DELAY 5000"#,
            vec![
                Statement::CommandDelay(StatementCommandDelay {
                    value: String::from("5000"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        test_parser_input_valid_command_delay_variable: (
            r#"DELAY ($CUSTOM_DELAY)"#,
            vec![
                Statement::CommandDelay(StatementCommandDelay {
                    value: String::from("($CUSTOM_DELAY)"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // DEFAULTDELAY
        test_parser_input_valid_command_defaultdelay: (
            r#"DEFAULTDELAY 500"#,
            vec![
                Statement::CommandDefaultDelay(StatementCommandDefaultDelay {
                    value: String::from("500"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        test_parser_input_valid_command_defaultdelay_variable: (
            r#"DEFAULTDELAY $CUSTOM_DELAY"#,
            vec![
                Statement::CommandDefaultDelay(StatementCommandDefaultDelay {
                    value: String::from("$CUSTOM_DELAY"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // Keystroke Injection.
        // STRINGLN
        test_parser_input_valid_command_stringln: (
            r#"STRINGLN Hello, Friend"#,
            vec![
                Statement::CommandStringln(StatementCommandStringln {
                    value: String::from("Hello, Friend"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // STRING
        test_parser_input_valid_command_string: (
            r#"STRING Hello, Friend"#,
            vec![
                Statement::CommandString(StatementCommandString {
                    value: String::from("Hello, Friend"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        test_parser_input_valid_command_string_url: (
            r#"STRING https://www.youtube.com/watch?v=dQw4w9WgXcQ"#,
            vec![
                Statement::CommandString(StatementCommandString {
                    value: String::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // Cursor Keys.
            // BACKSPACE
            test_parser_input_valid_command_backspace: (
                r#"BACKSPACE"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("BACKSPACE"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // DELETE
            test_parser_input_valid_command_delete: (
                r#"DELETE"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("DELETE"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // DEL
            test_parser_input_valid_command_del: (
                r#"DEL"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("DEL"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // DOWNARROW
            test_parser_input_valid_command_downarrow: (
                r#"DOWNARROW"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("DOWNARROW"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // DOWN
            test_parser_input_valid_command_down: (
                r#"DOWN"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("DOWN"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // END
            test_parser_input_valid_command_end: (
                r#"END"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("END"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // HOME
            test_parser_input_valid_command_home: (
                r#"HOME"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("HOME"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // INSERT
            test_parser_input_valid_command_insert: (
                r#"INSERT"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("INSERT"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // LEFTARROW
            test_parser_input_valid_command_leftarrow: (
                r#"LEFTARROW"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("LEFTARROW"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // LEFT
            test_parser_input_valid_command_left: (
                r#"LEFT"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("LEFT"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // PAGEDOWN
            test_parser_input_valid_command_pagedown: (
                r#"PAGEDOWN"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("PAGEDOWN"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // PAGEUP
            test_parser_input_valid_command_pageup: (
                r#"PAGEUP"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("PAGEUP"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // RIGHTARROW
            test_parser_input_valid_command_rightarrow: (
                r#"RIGHTARROW"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("RIGHTARROW"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // RIGHT
            test_parser_input_valid_command_right: (
                r#"RIGHT"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("RIGHT"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // SPACE
            test_parser_input_valid_command_space: (
                r#"SPACE"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("SPACE"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // TAB
            test_parser_input_valid_command_tab: (
                r#"TAB"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("TAB"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // UPARROW
            test_parser_input_valid_command_uparrow: (
                r#"UPARROW"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("UPARROW"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // UP
            test_parser_input_valid_command_up: (
                r#"UP"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("UP"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
        // System Keys.
            // APP
            test_parser_input_valid_command_app: (
                r#"APP"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("APP"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // BREAK
            test_parser_input_valid_command_break: (
                r#"BREAK"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("BREAK"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // ENTER
            test_parser_input_valid_command_enter: (
                r#"ENTER"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("ENTER"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // ESCAPE
            test_parser_input_valid_command_escape: (
                r#"ESCAPE"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("ESCAPE"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F0
            test_parser_input_valid_command_f0: (
                r#"F0"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("F0"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F10
            test_parser_input_valid_command_f10: (
                r#"F10"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("F10"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F11
            test_parser_input_valid_command_f11: (
                r#"F11"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("F11"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F12
            test_parser_input_valid_command_f12: (
                r#"F12"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("F12"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F1
            test_parser_input_valid_command_f1: (
                r#"F1"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("F1"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F2
            test_parser_input_valid_command_f2: (
                r#"F2"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("F2"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F3
            test_parser_input_valid_command_f3: (
                r#"F3"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("F3"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F4
            test_parser_input_valid_command_f4: (
                r#"F4"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("F4"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F5
            test_parser_input_valid_command_f5: (
                r#"F5"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("F5"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F6
            test_parser_input_valid_command_f6: (
                r#"F6"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("F6"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F7
            test_parser_input_valid_command_f7: (
                r#"F7"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("F7"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F8
            test_parser_input_valid_command_f8: (
                r#"F8"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("F8"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F9
            test_parser_input_valid_command_f9: (
                r#"F9"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("F9"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // MENU
            test_parser_input_valid_command_menu: (
                r#"MENU"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("MENU"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // PAUSE
            test_parser_input_valid_command_pause: (
                r#"PAUSE"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("PAUSE"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // PRINTSCREEN
            test_parser_input_valid_command_printscreen: (
                r#"PRINTSCREEN"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("PRINTSCREEN"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
        // Basic Modifier Keys.
            // ALT
            test_parser_input_valid_command_alt: (
                r#"ALT"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("ALT"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // COMMAND
            test_parser_input_valid_command_command: (
                r#"COMMAND"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("COMMAND"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // CONTROL
            test_parser_input_valid_command_control: (
                r#"CONTROL"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("CONTROL"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            test_parser_input_valid_command_control_shift_escape: (
                r#"CONTROL SHIFT ESCAPE"#,
                vec![
                    Statement::CommandKey(
                        StatementCommandKey {
                            statements: vec![
                                Statement::CommandKeyValue(
                                    StatementCommandKeyValue {
                                        name: String::from("CONTROL"),
                                    },
                                ),
                                Statement::CommandKey(
                                    StatementCommandKey {
                                        statements: vec![
                                            Statement::CommandKeyValue(
                                                StatementCommandKeyValue {
                                                    name: String::from("SHIFT"),
                                                },
                                            ),
                                            Statement::CommandKey(
                                                StatementCommandKey {
                                                    statements: vec![
                                                        Statement::CommandKeyValue(
                                                            StatementCommandKeyValue {
                                                                name: String::from("ESCAPE"),
                                                            },
                                                        ),
                                                    ],
                                                    remaining_keys: String::from(""),
                                                },
                                            ),
                                        ],
                                        remaining_keys: String::from(""),
                                    },
                                ),
                            ],
                            remaining_keys: String::from(""),
                        },
                    ),
                    Statement::End(StatementEnd { }),
                ]
            ),
            test_parser_input_valid_command_control_s: (
                r#"CONTROL s"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("CONTROL"),
                            }),
                        ],
                        remaining_keys: String::from("s"),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // CTRL
            test_parser_input_valid_command_ctrl: (
                r#"CTRL"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("CTRL"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // GUI
            test_parser_input_valid_command_gui: (
                r#"GUI"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("GUI"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // SHIFT
            test_parser_input_valid_command_shift: (
                r#"SHIFT"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("SHIFT"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // WINDOWS
            test_parser_input_valid_command_windows: (
                r#"WINDOWS r"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("WINDOWS"),
                            }),
                        ],
                        remaining_keys: String::from("r"),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
        // Advanced Modifier Keys.
            // OPTION
            test_parser_input_valid_command_option: (
                r#"OPTION"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("OPTION"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
        // Standalone Modifier Keys.
            // INJECT_MOD
            test_parser_input_valid_command_inject_mod: (
                r#"INJECT_MOD"#,
                vec![
                    Statement::SingleCommand(StatementSingleCommand {
                        name: String::from("INJECT_MOD"),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
        // Lock Keys.
            // CAPSLOCK
            test_parser_input_valid_command_capslock: (
                r#"CAPSLOCK"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("CAPSLOCK"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // NUMLOCK
            test_parser_input_valid_command_numlock: (
                r#"NUMLOCK"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("NUMLOCK"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // SCROLLOCK
            test_parser_input_valid_command_scrollock: (
                r#"SCROLLOCK"#,
                vec![
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("SCROLLOCK"),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
    // Button.
        // WAIT_FOR_BUTTON_PRESS
        test_parser_input_valid_command_wait_for_button_press: (
            r#"WAIT_FOR_BUTTON_PRESS"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("WAIT_FOR_BUTTON_PRESS"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // BUTTON_DEF
        test_parser_input_valid_command_button_def: (
            r#"BUTTON_DEF"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("BUTTON_DEF"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // DISABLE_BUTTON
        test_parser_input_valid_command_disable_button: (
            r#"DISABLE_BUTTON"#,
            vec![
                    Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("DISABLE_BUTTON"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // ENABLE_BUTTON
        test_parser_input_valid_command_enable_button: (
            r#"ENABLE_BUTTON"#,
            vec![
                    Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("ENABLE_BUTTON"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // LED.
        // LED_OFF
        test_parser_input_valid_command_led_off: (
            r#"LED_OFF"#,
            vec![
                    Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("LED_OFF"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // LED_R
        test_parser_input_valid_command_led_r: (
            r#"LED_R"#,
            vec![
                    Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("LED_R"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // LED_G
        test_parser_input_valid_command_led_g: (
            r#"LED_G"#,
            vec![
                    Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("LED_G"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // Attack Mode.
        // SAVE_ATTACKMODE
        test_parser_input_valid_command_save_attackmode: (
            r#"SAVE_ATTACKMODE"#,
            vec![
                    Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("SAVE_ATTACKMODE"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // RESTORE_ATTACKMODE
        test_parser_input_valid_command_restore_attackmode: (
            r#"RESTORE_ATTACKMODE"#,
            vec![
                    Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("RESTORE_ATTACKMODE"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // ATTACKMODE
        test_parser_input_valid_command_attackmode: (
            r#"ATTACKMODE"#,
            vec![
                    Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("ATTACKMODE"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // Constants.
        // DEFINE
        test_parser_input_valid_command_define: (
            r#"DEFINE WAIT 1000"#,
            vec![
                Statement::CommandDefine(StatementCommandDefine {
                    name: String::from("WAIT"),
                    value: String::from("1000"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // Variables.
        // VAR - Declaration.
        test_parser_input_valid_command_var_declaration: (
            r#"VAR $BLINK = TRUE"#,
            vec![
                Statement::VariableDeclaration(StatementVariableDeclaration {
                    name: String::from("BLINK"),
                    assignment: String::from("TRUE"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // VAR - Assignment.
        test_parser_input_valid_command_var_assignment: (
            r#"$BLINK = TRUE"#,
            vec![
                Statement::VariableAssignment(StatementVariableAssignment {
                    name: String::from("BLINK"),
                    assignment: String::from("TRUE"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // Randomization.
        // RANDOM_LOWERCASE_LETTER
        test_parser_input_valid_command_random_lowercase_letter: (
            r#"RANDOM_LOWERCASE_LETTER"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("RANDOM_LOWERCASE_LETTER"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // RANDOM_UPPERCASE_LETTER
        test_parser_input_valid_command_random_uppercase_letter: (
            r#"RANDOM_UPPERCASE_LETTER"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("RANDOM_UPPERCASE_LETTER"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // RANDOM_LETTER
        test_parser_input_valid_command_random_letter: (
            r#"RANDOM_LETTER"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("RANDOM_LETTER"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // RANDOM_NUMBER
        test_parser_input_valid_command_random_number: (
            r#"RANDOM_NUMBER"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("RANDOM_NUMBER"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // RANDOM_SPECIAL
        test_parser_input_valid_command_random_special: (
            r#"RANDOM_SPECIAL"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("RANDOM_SPECIAL"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // RANDOM_CHAR
        test_parser_input_valid_command_random_char: (
            r#"RANDOM_CHAR"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("RANDOM_CHAR"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // ATTACKMODE.
        // VID_RANDOM
        test_parser_input_valid_command_vid_random: (
            r#"VID_RANDOM"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("VID_RANDOM"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // PID_RANDOM
        test_parser_input_valid_command_pid_random: (
            r#"PID_RANDOM"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("PID_RANDOM"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // MAN_RANDOM
        test_parser_input_valid_command_man_random: (
            r#"MAN_RANDOM"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("MAN_RANDOM"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // PROD_RANDOM
        test_parser_input_valid_command_prod_random: (
            r#"PROD_RANDOM"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("PROD_RANDOM"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // SERIAL_RANDOM
        test_parser_input_valid_command_serial_random: (
            r#"SERIAL_RANDOM"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("SERIAL_RANDOM"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // Holding Keys.
        // HOLD
        test_parser_input_valid_command_hold: (
            r#"HOLD"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("HOLD"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // RELEASE
        test_parser_input_valid_command_release: (
            r#"RELEASE"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("RELEASE"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // Payload Control.
        // RESTART_PAYLOAD
        test_parser_input_valid_command_restart_payload: (
            r#"RESTART_PAYLOAD"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("RESTART_PAYLOAD"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // STOP_PAYLOAD
        test_parser_input_valid_command_stop_payload: (
            r#"STOP_PAYLOAD"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("STOP_PAYLOAD"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // RESET
        test_parser_input_valid_command_reset: (
            r#"RESET"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("RESET"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // Payload Hiding.
        // HIDE_PAYLOAD
        test_parser_input_valid_command_hide_payload: (
            r#"HIDE_PAYLOAD"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("HIDE_PAYLOAD"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // RESTORE_PAYLOAD
        test_parser_input_valid_command_restore_payload: (
            r#"RESTORE_PAYLOAD"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("RESTORE_PAYLOAD"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // Lock Keys.
        // WAIT_FOR_CAPS_ON
        test_parser_input_valid_command_wait_for_caps_on: (
            r#"WAIT_FOR_CAPS_ON"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("WAIT_FOR_CAPS_ON"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // WAIT_FOR_CAPS_OFF
        test_parser_input_valid_command_wait_for_caps_off: (
            r#"WAIT_FOR_CAPS_OFF"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("WAIT_FOR_CAPS_OFF"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // WAIT_FOR_CAPS_CHANGE
        test_parser_input_valid_command_wait_for_caps_change: (
            r#"WAIT_FOR_CAPS_CHANGE"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("WAIT_FOR_CAPS_CHANGE"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // WAIT_FOR_NUM_ON
        test_parser_input_valid_command_wait_for_num_on: (
            r#"WAIT_FOR_NUM_ON"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("WAIT_FOR_NUM_ON"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // WAIT_FOR_NUM_OFF
        test_parser_input_valid_command_wait_for_num_off: (
            r#"WAIT_FOR_NUM_OFF"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("WAIT_FOR_NUM_OFF"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // WAIT_FOR_NUM_CHANGE
        test_parser_input_valid_command_wait_for_num_change: (
            r#"WAIT_FOR_NUM_CHANGE"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("WAIT_FOR_NUM_CHANGE"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // WAIT_FOR_SCROLL_ON
        test_parser_input_valid_command_wait_for_scroll_on: (
            r#"WAIT_FOR_SCROLL_ON"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("WAIT_FOR_SCROLL_ON"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // WAIT_FOR_SCROLL_OFF
        test_parser_input_valid_command_wait_for_scroll_off: (
            r#"WAIT_FOR_SCROLL_OFF"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("WAIT_FOR_SCROLL_OFF"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // WAIT_FOR_SCROLL_CHANGE
        test_parser_input_valid_command_wait_for_scroll_change: (
            r#"WAIT_FOR_SCROLL_CHANGE"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("WAIT_FOR_SCROLL_CHANGE"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // 'SAVE' & 'RESTORE' Commands.
        // SAVE_HOST_KEYBOARD_LOCK_STATE
        test_parser_input_valid_command_save_host_keyboard_lock_state: (
            r#"SAVE_HOST_KEYBOARD_LOCK_STATE"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("SAVE_HOST_KEYBOARD_LOCK_STATE"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // RESTORE_HOST_KEYBOARD_LOCK_STATE
        test_parser_input_valid_command_restore_host_keyboard_lock_state: (
            r#"RESTORE_HOST_KEYBOARD_LOCK_STATE"#,
            vec![
                Statement::SingleCommand(StatementSingleCommand {
                    name: String::from("RESTORE_HOST_KEYBOARD_LOCK_STATE"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // Exfiltration.
        // EXFIL
        test_parser_input_valid_command_exfil: (
            r#"EXFIL $FOO"#,
            vec![
                Statement::CommandExfil(StatementCommandExfil {
                    name: String::from("FOO"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // Control Flow
        // IF
        test_parser_input_valid_if: (
            r#"IF TRUE THEN
  STRING Hello, Friend
END_IF"#,
            vec![
                Statement::BlockIf(StatementBlockIf {
                    expression: String::from("TRUE"),
                    statements_true: vec![
                        Statement::CommandString(StatementCommandString {
                            value: String::from("Hello, Friend"),
                        })
                    ],
                    statements_false: vec![],
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        test_parser_input_valid_if_with_operator_equals: (
            r#"IF $MY_VALUE > 0 THEN
  STRING Hello, Friend
END_IF"#,
            vec![
                Statement::BlockIf(StatementBlockIf {
                    expression: String::from("$MY_VALUE > 0"),
                    statements_true: vec![
                        Statement::CommandString(StatementCommandString {
                            value: String::from("Hello, Friend"),
                        })
                    ],
                    statements_false: vec![],
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        test_parser_input_valid_if_and_else: (
            r#"IF TRUE THEN
  STRING Hello, Friend
ELSE
  STRING Hello, Dog?
END_IF"#,
            vec![
                Statement::BlockIf(StatementBlockIf {
                    expression: String::from("TRUE"),
                    statements_true: vec![
                        Statement::CommandString(StatementCommandString {
                            value: String::from("Hello, Friend"),
                        })
                    ],
                    statements_false: vec![
                        Statement::CommandString(StatementCommandString {
                            value: String::from("Hello, Dog?"),
                        })
                    ],
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        test_parser_input_valid_if_and_else_nested_if_and_else: (
            r#"IF TRUE THEN
  IF $MY_CONDITION_ONE THEN
    STRING Hello, Friend One
  ELSE
    STRING Hello, Friend Two
  END_IF
ELSE
  IF $MY_CONDITION_TWO THEN
    STRING Hello, Dog One?
  ELSE
    STRING Hello, Dog Two?
  END_IF
END_IF"#,
            vec![
                Statement::BlockIf(StatementBlockIf {
                    expression: String::from("TRUE"),
                    statements_true: vec![
                        Statement::BlockIf(StatementBlockIf {
                            expression: String::from("$MY_CONDITION_ONE"),
                            statements_true: vec![
                                Statement::CommandString(StatementCommandString {
                                    value: String::from("Hello, Friend One"),
                                })
                            ],
                            statements_false: vec![
                                Statement::CommandString(StatementCommandString {
                                    value: String::from("Hello, Friend Two"),
                                })
                            ]
                        })
                    ],
                    statements_false: vec![
                        Statement::BlockIf(StatementBlockIf {
                            expression: String::from("$MY_CONDITION_TWO"),
                            statements_true: vec![
                                Statement::CommandString(StatementCommandString {
                                    value: String::from("Hello, Dog One?"),
                                })
                            ],
                            statements_false: vec![
                                Statement::CommandString(StatementCommandString {
                                    value: String::from("Hello, Dog Two?"),
                                })
                            ]
                        })
                    ],
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // WHILE
        test_parser_input_valid_while: (
            r#"WHILE TRUE
  STRING Hello, Friend
END_WHILE"#,
            vec![
                Statement::BlockWhile(StatementBlockWhile {
                    expression: String::from("TRUE"),
                    statements: vec![
                        Statement::CommandString(StatementCommandString {
                            value: String::from("Hello, Friend"),
                        })
                    ],
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        test_parser_input_valid_while_with_operator_equals: (
            r#"WHILE $MY_VALUE > 0
  STRING Hello, Friend
END_WHILE"#,
            vec![
                Statement::BlockWhile(StatementBlockWhile {
                    expression: String::from("$MY_VALUE > 0"),
                    statements: vec![
                        Statement::CommandString(StatementCommandString {
                            value: String::from("Hello, Friend"),
                        })
                    ],
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        test_parser_input_valid_while_nested: (
            r#"WHILE TRUE
  WHILE $MY_CONDITION_ONE < 10
    STRING Hello, Friend
  END_WHILE
END_WHILE"#,
            vec![
                Statement::BlockWhile(StatementBlockWhile {
                    expression: String::from("TRUE"),
                    statements: vec![
                        Statement::BlockWhile(StatementBlockWhile {
                            expression: String::from("$MY_CONDITION_ONE < 10"),
                            statements: vec![
                                Statement::CommandString(StatementCommandString {
                                    value: String::from("Hello, Friend"),
                                })
                            ],
                        })
                    ],
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // MallardScript.
        // IMPORT
        test_parser_input_valid_import: (
            r#"IMPORT "./some_file.ducky""#,
            vec![
                Statement::CommandImport(StatementCommandImport {
                    value: String::from("./some_file.ducky"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),

    // Multiple Instructions.
    test_parser_input_valid_rem_string_import_string: (
r#"REM Hello, Friend
STRING Yes, this is dog.
IMPORT "./some_file.ducky"
DELAY 30
STRING Is that so?
CONTROL SHIFT ESCAPE"#,
        vec![
            Statement::CommandRem(StatementCommandRem {
                value: String::from("Hello, Friend"),
            }),
            Statement::CommandString(StatementCommandString {
                value: String::from("Yes, this is dog."),
            }),
            Statement::CommandImport(StatementCommandImport {
                value: String::from("./some_file.ducky"),
            }),
            Statement::CommandDelay(StatementCommandDelay {
                value: String::from("30"),
            }),
            Statement::CommandString(StatementCommandString {
                value: String::from("Is that so?"),
            }),
            Statement::CommandKey(StatementCommandKey {
                statements: vec![
                    Statement::CommandKeyValue(StatementCommandKeyValue {
                        name: String::from("CONTROL"),
                    }),
                    Statement::CommandKey(StatementCommandKey {
                        statements: vec![
                            Statement::CommandKeyValue(StatementCommandKeyValue {
                                name: String::from("SHIFT"),
                            }),
                            Statement::CommandKey(StatementCommandKey {
                                statements: vec![
                                    Statement::CommandKeyValue(StatementCommandKeyValue {
                                        name: String::from("ESCAPE"),
                                    }),
                                ],
                                remaining_keys: String::from(""),
                            }),
                        ],
                        remaining_keys: String::from(""),
                    }),
                ],
                remaining_keys: String::from(""),
            }),
            Statement::End(StatementEnd {}),
        ]
    ),
}

test_parser_input_invalid! {
    // Giberish.
    test_parser_input_invalid_giberish: (
        r#"inDOFNlsl.m"#,
        "Unable to parse provided document.

Caused by:
     --> 1:1
      |
    1 | inDOFNlsl.m
      | ^---
      |
      = expected document"
    ),

    // VAR.
    test_parser_input_invalid_var: (
        r#"
REM Hello, Friend
VAR $1MY_VAR = 1000
"#,
        "Unable to parse provided document.

Caused by:
     --> 3:6
      |
    3 | VAR $1MY_VAR = 1000
      |      ^---
      |
      = expected keyword_name"
    ),

    // MallardScript Syntax.
    test_parser_input_invalid_mallard: (
        r#"
REM Hello, Friend
IMPORT ./some_file.ducky
"#,
        "Unable to parse provided document.

Caused by:
     --> 3:1
      |
    3 | IMPORT ./some_file.ducky
      | ^---
      |
      = expected EOI, statement_command_rem, statement_command_defaultdelay, statement_command_delay, statement_command_stringln, statement_command_string, statement_command_key, statement_command_inject_mod, statement_command_wait_for_button_press, statement_command_button_def, statement_command_disable_button, statement_command_enable_button, statement_command_led_off, statement_command_led_r, statement_command_led_g, statement_command_save_attackmode, statement_command_restore_attackmode, statement_command_attackmode, statement_command_define, statement_command_random_lowercase_letter, statement_command_random_uppercase_letter, statement_command_random_letter, statement_command_random_number, statement_command_random_special, statement_command_random_char, statement_command_vid_random, statement_command_pid_random, statement_command_man_random, statement_command_prod_random, statement_command_serial_random, statement_command_hold, statement_command_release, statement_command_restart_payload, statement_command_stop_payload, statement_command_reset, statement_command_hide_payload, statement_command_restore_payload, statement_command_wait_for_caps_on, statement_command_wait_for_caps_off, statement_command_wait_for_caps_change, statement_command_wait_for_num_on, statement_command_wait_for_num_off, statement_command_wait_for_num_change, statement_command_wait_for_scroll_on, statement_command_wait_for_scroll_off, statement_command_wait_for_scroll_change, statement_command_save_host_keyboard_lock_state, statement_command_restore_host_keyboard_lock_state, statement_command_exfil, statement_command_import, statement_variable_declaration, statement_variable_assignment, statement_block_if, or statement_block_while"
    ),
}
