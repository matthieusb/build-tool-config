use common::get_base_cargo_run_command;

#[cfg(test)]
mod general_cli_integration_test {
    use super::*;
    use assert_cli;

    // * IMPORTANT: It is possible to change ENV variable in tests with assert-CLI
    // * This could be used to do real integration tests on this app
    // * ALSO You can change the current directory used by the app

    // ! TODO Add error where cases where arguments are missing (Do this in a different file)

    // ------------------------------------------------------
    // ------- GENERAL
    // ------------------------------------------------------

    /// Test for command "cargo run"
    #[test]
    fn calling_btc_withtout_args() {
        assert_cli::Assert::main_binary().succeeds().unwrap();
    }

    /// Test for command "cargo run --bin build_tool_config -- -h"
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

    /// Test for command "cargo run --bin build_tool_config -- --all-tools --unset-settings all"
    #[test]
    fn calling_btc_unset_all_settings_all_tools() {
        // PREPARE
        let mut calling_btc_all_tools_unset_all_settings_argument = get_base_cargo_run_command();
        calling_btc_all_tools_unset_all_settings_argument.extend(vec!["--all-tools", "--unset-settings", "all"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&calling_btc_all_tools_unset_all_settings_argument[..])
            .succeeds()
            .stdout()
            .contains("Unsetting all settings for all tools")
            .unwrap();
    }

    // TODO See where to put this test
    /// Test for command "cargo run --bin build_tool_config -- --all-tools --unset-settings http-proxy repository"
    #[test]
    fn calling_btc_unset_http_proxy_and_repository_settings_all_tools() {
        // PREPARE
        let mut calling_btc_all_tools_unset_http_proxy_and_repository_settings_argument = get_base_cargo_run_command();
        calling_btc_all_tools_unset_http_proxy_and_repository_settings_argument.extend(vec!["--all-tools", "--unset-settings", "http-proxy", "repository"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&calling_btc_all_tools_unset_http_proxy_and_repository_settings_argument[..])
            .succeeds()
            .stdout().contains("Unsetting")
            .stdout().contains("http-proxy")
            .stdout().contains("repository")
            .stdout().contains("settings for all tools")
            .unwrap();
    }


}
