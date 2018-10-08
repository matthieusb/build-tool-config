use std;

// -- Module imports
pub mod argument_parsing_int_test;

// -- Common methods

/// Method to get cargo run command line base to launch. Imitates a ./ on the generated executable file.
pub fn get_base_cargo_run_command<'a>() -> std::vec::Vec<&'a str> {
    vec!["cargo", "run", "--bin", "build_tool_config", "--"]
}