// Integration tests on argument handling through the CLI
extern crate assert_cli;


fn get_base_cargo_run_command<'a>() -> std::vec::Vec<&'a str> {
    vec!["cargo", "run", "--bin", "build_tool_config", "--"]
}

#[cfg(test)]
mod cli_integration_test {
    use super::*;
    use assert_cli;

    
    // * IMPORTANT: It is possible to change ENV variable in tests with assert-CLI
    // * This could be used to do real integration tests on this app
    // * ALSO You can change the current directory used by the app

    /**
     * Test for command "cargo run"
     */
    #[test]
    fn calling_btc_withtout_args() {
        assert_cli::Assert::main_binary()
        .succeeds()
        .unwrap();
    }

    /**
     * Test for command "cargo run --bin build_tool_config -- -h"
     */
    #[test]
    fn calling_btc_with_help_argument() {
        // PREPARE
        let mut help_command = get_base_cargo_run_command();
        help_command.extend(vec!["-h"]);
    
        // EXECUTE/ASSERT
        
        assert_cli::Assert::command(&help_command[..])
            .stdout().contains("USAGE")
            .stdout().contains("FLAGS")
            .stdout().contains("OPTIONS")
        .unwrap();
    }
}