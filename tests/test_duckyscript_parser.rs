extern crate pest_duckyscript;
extern crate predicates;
extern crate pretty_assertions;

use pest_duckyscript::duckyscript::{
    ast::{Statement, StatementCommand, StatementEnd, StatementVariable},
    parser::parse_document,
};
use pretty_assertions::assert_eq;
mod vec_eq;

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
            assert!(vec_eq::vec_eq(output, expected_statements));

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
                Statement::Command(StatementCommand {
                    name: String::from("DELAY"),
                    value: String::from("5000"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // DEFAULTDELAY
        test_parser_input_valid_command_defaultdelay: (
            r#"DEFAULTDELAY 500"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("DEFAULTDELAY"),
                    value: String::from("500"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // Keystroke Injection.
        // STRINGLN
        test_parser_input_valid_command_stringln: (
            r#"STRINGLN Hello, Friend"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("STRINGLN"),
                    value: String::from("Hello, Friend"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // STRING
        test_parser_input_valid_command_string: (
            r#"STRING Hello, Friend"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("STRING"),
                    value: String::from("Hello, Friend"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        test_parser_input_valid_command_string_url: (
            r#"STRING https://www.youtube.com/watch?v=dQw4w9WgXcQ"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("STRING"),
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
                    Statement::Command(StatementCommand {
                        name: String::from("BACKSPACE"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // DELETE
            test_parser_input_valid_command_delete: (
                r#"DELETE"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("DELETE"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // DEL
            test_parser_input_valid_command_del: (
                r#"DEL"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("DEL"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // DOWNARROW
            test_parser_input_valid_command_downarrow: (
                r#"DOWNARROW"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("DOWNARROW"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // DOWN
            test_parser_input_valid_command_down: (
                r#"DOWN"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("DOWN"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // END
            test_parser_input_valid_command_end: (
                r#"END"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("END"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // HOME
            test_parser_input_valid_command_home: (
                r#"HOME"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("HOME"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // INSERT
            test_parser_input_valid_command_insert: (
                r#"INSERT"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("INSERT"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // LEFTARROW
            test_parser_input_valid_command_leftarrow: (
                r#"LEFTARROW"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("LEFTARROW"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // LEFT
            test_parser_input_valid_command_left: (
                r#"LEFT"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("LEFT"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // PAGEDOWN
            test_parser_input_valid_command_pagedown: (
                r#"PAGEDOWN"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("PAGEDOWN"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // PAGEUP
            test_parser_input_valid_command_pageup: (
                r#"PAGEUP"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("PAGEUP"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // RIGHTARROW
            test_parser_input_valid_command_rightarrow: (
                r#"RIGHTARROW"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("RIGHTARROW"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // RIGHT
            test_parser_input_valid_command_right: (
                r#"RIGHT"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("RIGHT"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // SPACE
            test_parser_input_valid_command_space: (
                r#"SPACE"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("SPACE"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // TAB
            test_parser_input_valid_command_tab: (
                r#"TAB"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("TAB"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // UPARROW
            test_parser_input_valid_command_uparrow: (
                r#"UPARROW"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("UPARROW"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // UP
            test_parser_input_valid_command_up: (
                r#"UP"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("UP"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
        // System Keys.
            // APP
            test_parser_input_valid_command_app: (
                r#"APP"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("APP"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // BREAK
            test_parser_input_valid_command_break: (
                r#"BREAK"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("BREAK"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // ENTER
            test_parser_input_valid_command_enter: (
                r#"ENTER"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("ENTER"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // ESCAPE
            test_parser_input_valid_command_escape: (
                r#"ESCAPE"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("ESCAPE"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F0
            test_parser_input_valid_command_f0: (
                r#"F0"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("F0"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F10
            test_parser_input_valid_command_f10: (
                r#"F10"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("F10"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F11
            test_parser_input_valid_command_f11: (
                r#"F11"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("F11"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F12
            test_parser_input_valid_command_f12: (
                r#"F12"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("F12"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F1
            test_parser_input_valid_command_f1: (
                r#"F1"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("F1"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F2
            test_parser_input_valid_command_f2: (
                r#"F2"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("F2"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F3
            test_parser_input_valid_command_f3: (
                r#"F3"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("F3"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F4
            test_parser_input_valid_command_f4: (
                r#"F4"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("F4"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F5
            test_parser_input_valid_command_f5: (
                r#"F5"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("F5"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F6
            test_parser_input_valid_command_f6: (
                r#"F6"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("F6"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F7
            test_parser_input_valid_command_f7: (
                r#"F7"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("F7"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F8
            test_parser_input_valid_command_f8: (
                r#"F8"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("F8"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // F9
            test_parser_input_valid_command_f9: (
                r#"F9"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("F9"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // MENU
            test_parser_input_valid_command_menu: (
                r#"MENU"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("MENU"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // PAUSE
            test_parser_input_valid_command_pause: (
                r#"PAUSE"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("PAUSE"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // PRINTSCREEN
            test_parser_input_valid_command_printscreen: (
                r#"PRINTSCREEN"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("PRINTSCREEN"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
        // Basic Modifier Keys.
            // ALT
            test_parser_input_valid_command_alt: (
                r#"ALT"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("ALT"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // COMMAND
            test_parser_input_valid_command_command: (
                r#"COMMAND"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("COMMAND"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // CONTROL
            test_parser_input_valid_command_control: (
                r#"CONTROL"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("CONTROL"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            test_parser_input_valid_command_control_shift_esc: (
                r#"CONTROL SHIFT ESC"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("CONTROL"),
                        value: String::from("SHIFT ESC"),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            test_parser_input_valid_command_control_s: (
                r#"CONTROL s"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("CONTROL"),
                        value: String::from("s"),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // CTRL
            test_parser_input_valid_command_ctrl: (
                r#"CTRL"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("CTRL"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // GUI
            test_parser_input_valid_command_gui: (
                r#"GUI"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("GUI"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // SHIFT
            test_parser_input_valid_command_shift: (
                r#"SHIFT"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("SHIFT"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // WINDOWS
            test_parser_input_valid_command_windows: (
                r#"WINDOWS r"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("WINDOWS"),
                        value: String::from("r"),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
        // Advanced Modifier Keys.
            // OPTION
            test_parser_input_valid_command_option: (
                r#"OPTION"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("OPTION"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
        // Standalone Modifier Keys.
            // INJECT_MOD
            test_parser_input_valid_command_inject_mod: (
                r#"INJECT_MOD"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("INJECT_MOD"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
        // Lock Keys.
            // CAPSLOCK
            test_parser_input_valid_command_capslock: (
                r#"CAPSLOCK"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("CAPSLOCK"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // NUMLOCK
            test_parser_input_valid_command_numlock: (
                r#"NUMLOCK"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("NUMLOCK"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
            // SCROLLOCK
            test_parser_input_valid_command_scrollock: (
                r#"SCROLLOCK"#,
                vec![
                    Statement::Command(StatementCommand {
                        name: String::from("SCROLLOCK"),
                        value: String::from(""),
                    }),
                    Statement::End(StatementEnd {}),
                ]
            ),
    // Button.
        // WAIT_FOR_BUTTON_PRESS
        test_parser_input_valid_command_wait_for_button_press: (
            r#"WAIT_FOR_BUTTON_PRESS"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("WAIT_FOR_BUTTON_PRESS"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // BUTTON_DEF
        test_parser_input_valid_command_button_def: (
            r#"BUTTON_DEF"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("BUTTON_DEF"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // DISABLE_BUTTON
        test_parser_input_valid_command_disable_button: (
            r#"DISABLE_BUTTON"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("DISABLE_BUTTON"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // ENABLE_BUTTON
        test_parser_input_valid_command_enable_button: (
            r#"ENABLE_BUTTON"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("ENABLE_BUTTON"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // LED.
        // LED_OFF
        test_parser_input_valid_command_led_off: (
            r#"LED_OFF"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("LED_OFF"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // LED_R
        test_parser_input_valid_command_led_r: (
            r#"LED_R"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("LED_R"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // LED_G
        test_parser_input_valid_command_led_g: (
            r#"LED_G"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("LED_G"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // Attack Mode.
        // SAVE_ATTACKMODE
        test_parser_input_valid_command_save_attackmode: (
            r#"SAVE_ATTACKMODE"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("SAVE_ATTACKMODE"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // RESTORE_ATTACKMODE
        test_parser_input_valid_command_restore_attackmode: (
            r#"RESTORE_ATTACKMODE"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("RESTORE_ATTACKMODE"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // ATTACKMODE
        test_parser_input_valid_command_attackmode: (
            r#"ATTACKMODE"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("ATTACKMODE"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // Constants.
        // DEFINE
        test_parser_input_valid_command_define: (
            r#"DEFINE WAIT 1000"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("DEFINE"),
                    value: String::from("WAIT 1000"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // Variables.
        // VAR
        test_parser_input_valid_command_var: (
            r#"VAR $BLINK = TRUE"#,
            vec![
                Statement::Variable(StatementVariable {
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
                Statement::Command(StatementCommand {
                    name: String::from("RANDOM_LOWERCASE_LETTER"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // RANDOM_UPPERCASE_LETTER
        test_parser_input_valid_command_random_uppercase_letter: (
            r#"RANDOM_UPPERCASE_LETTER"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("RANDOM_UPPERCASE_LETTER"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // RANDOM_LETTER
        test_parser_input_valid_command_random_letter: (
            r#"RANDOM_LETTER"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("RANDOM_LETTER"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // RANDOM_NUMBER
        test_parser_input_valid_command_random_number: (
            r#"RANDOM_NUMBER"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("RANDOM_NUMBER"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // RANDOM_SPECIAL
        test_parser_input_valid_command_random_special: (
            r#"RANDOM_SPECIAL"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("RANDOM_SPECIAL"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // RANDOM_CHAR
        test_parser_input_valid_command_random_char: (
            r#"RANDOM_CHAR"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("RANDOM_CHAR"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // ATTACKMODE.
        // VID_RANDOM
        test_parser_input_valid_command_vid_random: (
            r#"VID_RANDOM"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("VID_RANDOM"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // PID_RANDOM
        test_parser_input_valid_command_pid_random: (
            r#"PID_RANDOM"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("PID_RANDOM"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // MAN_RANDOM
        test_parser_input_valid_command_man_random: (
            r#"MAN_RANDOM"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("MAN_RANDOM"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // PROD_RANDOM
        test_parser_input_valid_command_prod_random: (
            r#"PROD_RANDOM"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("PROD_RANDOM"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // SERIAL_RANDOM
        test_parser_input_valid_command_serial_random: (
            r#"SERIAL_RANDOM"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("SERIAL_RANDOM"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // Holding Keys.
        // HOLD
        test_parser_input_valid_command_hold: (
            r#"HOLD"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("HOLD"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // RELEASE
        test_parser_input_valid_command_release: (
            r#"RELEASE"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("RELEASE"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // Payload Control.
        // RESTART_PAYLOAD
        test_parser_input_valid_command_restart_payload: (
            r#"RESTART_PAYLOAD"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("RESTART_PAYLOAD"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // STOP_PAYLOAD
        test_parser_input_valid_command_stop_payload: (
            r#"STOP_PAYLOAD"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("STOP_PAYLOAD"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // RESET
        test_parser_input_valid_command_reset: (
            r#"RESET"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("RESET"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // Payload Hiding.
        // HIDE_PAYLOAD
        test_parser_input_valid_command_hide_payload: (
            r#"HIDE_PAYLOAD"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("HIDE_PAYLOAD"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // RESTORE_PAYLOAD
        test_parser_input_valid_command_restore_payload: (
            r#"RESTORE_PAYLOAD"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("RESTORE_PAYLOAD"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // Lock Keys.
        // WAIT_FOR_CAPS_ON
        test_parser_input_valid_command_wait_for_caps_on: (
            r#"WAIT_FOR_CAPS_ON"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("WAIT_FOR_CAPS_ON"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // WAIT_FOR_CAPS_OFF
        test_parser_input_valid_command_wait_for_caps_off: (
            r#"WAIT_FOR_CAPS_OFF"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("WAIT_FOR_CAPS_OFF"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // WAIT_FOR_CAPS_CHANGE
        test_parser_input_valid_command_wait_for_caps_change: (
            r#"WAIT_FOR_CAPS_CHANGE"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("WAIT_FOR_CAPS_CHANGE"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // WAIT_FOR_NUM_ON
        test_parser_input_valid_command_wait_for_num_on: (
            r#"WAIT_FOR_NUM_ON"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("WAIT_FOR_NUM_ON"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // WAIT_FOR_NUM_OFF
        test_parser_input_valid_command_wait_for_num_off: (
            r#"WAIT_FOR_NUM_OFF"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("WAIT_FOR_NUM_OFF"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // WAIT_FOR_NUM_CHANGE
        test_parser_input_valid_command_wait_for_num_change: (
            r#"WAIT_FOR_NUM_CHANGE"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("WAIT_FOR_NUM_CHANGE"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // WAIT_FOR_SCROLL_ON
        test_parser_input_valid_command_wait_for_scroll_on: (
            r#"WAIT_FOR_SCROLL_ON"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("WAIT_FOR_SCROLL_ON"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // WAIT_FOR_SCROLL_OFF
        test_parser_input_valid_command_wait_for_scroll_off: (
            r#"WAIT_FOR_SCROLL_OFF"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("WAIT_FOR_SCROLL_OFF"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // WAIT_FOR_SCROLL_CHANGE
        test_parser_input_valid_command_wait_for_scroll_change: (
            r#"WAIT_FOR_SCROLL_CHANGE"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("WAIT_FOR_SCROLL_CHANGE"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // 'SAVE' & 'RESTORE' Commands.
        // SAVE_HOST_KEYBOARD_LOCK_STATE
        test_parser_input_valid_command_save_host_keyboard_lock_state: (
            r#"SAVE_HOST_KEYBOARD_LOCK_STATE"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("SAVE_HOST_KEYBOARD_LOCK_STATE"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
        // RESTORE_HOST_KEYBOARD_LOCK_STATE
        test_parser_input_valid_command_restore_host_keyboard_lock_state: (
            r#"RESTORE_HOST_KEYBOARD_LOCK_STATE"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("RESTORE_HOST_KEYBOARD_LOCK_STATE"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // Exfiltration.
        // EXFIL
        test_parser_input_valid_command_exfil: (
            r#"EXFIL"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("EXFIL"),
                    value: String::from(""),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),
    // Control Flow.
        // REPEAT
        test_parser_input_valid_command_repeat: (
            r#"REPEAT 10"#,
            vec![
                Statement::Command(StatementCommand {
                    name: String::from("REPEAT"),
                    value: String::from("10"),
                }),
                Statement::End(StatementEnd {}),
            ]
        ),

    // Multiple Instructions.
    test_parser_input_valid_rem_string_import_string: (
r#"REM Hello, Friend
STRING Yes, this is dog.
DELAY 30
STRING Is that so?"#,
        vec![
            Statement::Command(StatementCommand {
                name: String::from("REM"),
                value: String::from("Hello, Friend"),
            }),
            Statement::Command(StatementCommand {
                name: String::from("STRING"),
                value: String::from("Yes, this is dog."),
            }),
            Statement::Command(StatementCommand {
                name: String::from("DELAY"),
                value: String::from("30"),
            }),
            Statement::Command(StatementCommand {
                name: String::from("STRING"),
                value: String::from("Is that so?"),
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
      = expected EOI, keyword_command, or statement_variable"
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
      = expected keyword_variable"
    ),
}
