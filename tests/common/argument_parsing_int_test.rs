use common::get_base_cargo_run_command;

#[cfg(test)]
mod general_cli_integration_test {
    use super::*;
    use assert_cli;

    // ! TODO Add error where cases where arguments are missing (Do this in a different file)

    // ! TODO Rewrite Tests using main_binary

    // ------------------------------------------------------
    // ------- GENERAL
    // ------------------------------------------------------

    /// Test for command "cargo run"
    #[test]
    fn calling_btc_withtout_args() {
        assert_cli::Assert::main_binary().succeeds().unwrap();
    }

    #[test]
    fn calling_btc_help_argument_displays_help() {
        // PREPARE
        let mut help_command = get_base_cargo_run_command();
        help_command.extend(vec!["-h"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&help_command[..])
            .succeeds()
            .stdout()
            .contains("USAGE")
            .stdout()
            .contains("FLAGS")
            .stdout()
            .contains("OPTIONS")
            .unwrap();
    }

    // ------------------------------------------------------
    // ------- UNSET SETTINGS
    // ------------------------------------------------------

    #[test]
    fn calling_btc_unset_all_settings_all_tools() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--all-tools", "--unset-settings", "all"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Unsetting following settings for all tools: all")
            .unwrap();
    }
    
    #[test]
    fn calling_btc_unset_http_proxy_and_repository_settings_all_tools() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--all-tools", "--unset-settings", "http-proxy", "repository"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Unsetting following settings for all tools: http-proxy repository")
            .unwrap();
    }
}
