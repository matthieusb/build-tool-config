use common::get_base_cargo_run_command;

#[cfg(test)]
mod repository_config_cli_integration_test {
    use super::*;
    use assert_cli;


    // ------------------------------------------------------
    // ------- SET SETTINGS
    // ------------------------------------------------------

        /// Test for command "cargo run --bin build_tool_config -- --all-tools --set-repository http://url:port"
    #[test]
    fn calling_btc_all_tools_repository_argument() {
        // PREPARE
        let mut set_all_tools_respository_command = get_base_cargo_run_command();
        set_all_tools_respository_command.extend(vec![
            "--all-tools",
            "--set-repository",
            "http://url:port",
        ]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&set_all_tools_respository_command[..])
            .succeeds()
            .stdout()
            .contains("Setting all tools repository to http://url:port")
            .unwrap();
    }


    // ------------------------------------------------------
    // ------- UNSET SETTINGS
    // ------------------------------------------------------

        /// Test for command "cargo run --bin build_tool_config -- --all-tools --unset-settings repository"
    #[test]
    fn calling_btc_unset_repository_settings_all_tools() {
        // PREPARE
        let mut calling_btc_all_tools_unset_repository_settings_argument = get_base_cargo_run_command();
        calling_btc_all_tools_unset_repository_settings_argument.extend(vec!["--all-tools", "--unset-settings", "repository"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&calling_btc_all_tools_unset_repository_settings_argument[..])
            .succeeds()
            .stdout()
            .contains("Unsetting repository settings for all tools")
            .unwrap();
    }


}