// file: argument_parsing_int_test.rs
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

    // TODO Split theses integration tests

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
    // ------- SET SETTINGS
    // ------------------------------------------------------

    /// Test for command "cargo run --bin build_tool_config -- --maven --set-http-proxy http://url:port"
    #[test]
    fn calling_btc_maven_http_proxy_argument() {
        // PREPARE
        let mut set_maven_proxy_command = get_base_cargo_run_command();
        set_maven_proxy_command.extend(vec!["--maven", "--set-http-proxy", "http://url:port"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&set_maven_proxy_command[..])
            .succeeds()
            .stdout()
            .contains("Setting maven http proxy to http://url:port")
            .unwrap();
    }

    /// Test for command "cargo run --bin build_tool_config -- --gradle --set-http-proxy http://url:port"
    #[test]
    fn calling_btc_gradle_http_proxy_argument() {
        // PREPARE
        let mut set_gradle_proxy_command = get_base_cargo_run_command();
        set_gradle_proxy_command.extend(vec!["--gradle", "--set-http-proxy", "http://url:port"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&set_gradle_proxy_command[..])
            .succeeds()
            .stdout()
            .contains("Setting gradle http proxy to http://url:port")
            .unwrap();
    }

    /// Test for command "cargo run --bin build_tool_config -- --all-tools --set-http-proxy http://url:port"
    #[test]
    fn calling_btc_all_tools_http_proxy_argument() {
        // PREPARE
        let mut set_all_tools_http_proxy_command = get_base_cargo_run_command();
        set_all_tools_http_proxy_command.extend(vec![
            "--all-tools",
            "--set-http-proxy",
            "http://url:port",
        ]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&set_all_tools_http_proxy_command[..])
            .succeeds()
            .stdout()
            .contains("Setting all tools http proxy to http://url:port")
            .unwrap();
    }

    /// Test for command "cargo run --bin build_tool_config -- --all-tools --set-https-proxy http://url:port"
    #[test]
    fn calling_btc_all_tools_https_proxy_argument() {
        // PREPARE
        let mut set_all_tools_https_proxy_command = get_base_cargo_run_command();
        set_all_tools_https_proxy_command.extend(vec![
            "--all-tools",
            "--set-https-proxy",
            "http://url:port",
        ]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&set_all_tools_https_proxy_command[..])
            .succeeds()
            .stdout()
            .contains("Setting all tools https proxy to http://url:port")
            .unwrap();
    }

    /// Test for command "cargo run --bin build_tool_config -- --all-tools --set-proxy http://url:port"
    #[test]
    fn calling_btc_all_tools_proxies_argument() {
        // PREPARE
        let mut set_all_tools_proxies_command = get_base_cargo_run_command();
        set_all_tools_proxies_command.extend(vec!["--all-tools", "--set-proxy", "http://url:port"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&set_all_tools_proxies_command[..])
            .succeeds()
            .stdout()
            .contains("Setting all tools proxies to http://url:port")
            .unwrap();
    }

    /// Test for command "cargo run --bin build_tool_config -- --all-tools --set-proxy http://url:port"
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

    /// Test for command "cargo run --bin build_tool_config -- --all-tools --unset-settings http-proxy"
    #[test]
    fn calling_btc_unset_http_proxy_settings_all_tools() {
        // PREPARE
        let mut calling_btc_all_tools_unset_http_proxy_settings_argument = get_base_cargo_run_command();
        calling_btc_all_tools_unset_http_proxy_settings_argument.extend(vec!["--all-tools", "--unset-settings", "http-proxy"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&calling_btc_all_tools_unset_http_proxy_settings_argument[..])
            .succeeds()
            .stdout()
            .contains("Unsetting http-proxy settings for all tools")
            .unwrap();
    }

    /// Test for command "cargo run --bin build_tool_config -- --all-tools --unset-settings https-proxy"
    #[test]
    fn calling_btc_unset_https_proxy_settings_all_tools() {
        // PREPARE
        let mut calling_btc_all_tools_unset_https_proxy_settings_argument = get_base_cargo_run_command();
        calling_btc_all_tools_unset_https_proxy_settings_argument.extend(vec!["--all-tools", "--unset-settings", "https-proxy"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&calling_btc_all_tools_unset_https_proxy_settings_argument[..])
            .succeeds()
            .stdout()
            .contains("Unsetting https-proxy settings for all tools")
            .unwrap();
    }

    /// Test for command "cargo run --bin build_tool_config -- --all-tools --unset-settings all-proxy"
    #[test]
    fn calling_btc_unset_all_proxy_settings_all_tools() {
        // PREPARE
        let mut calling_btc_all_tools_unset_all_proxy_settings_argument = get_base_cargo_run_command();
        calling_btc_all_tools_unset_all_proxy_settings_argument.extend(vec!["--all-tools", "--unset-settings", "all-proxy"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&calling_btc_all_tools_unset_all_proxy_settings_argument[..])
            .succeeds()
            .stdout()
            .contains("Unsetting all proxy settings for all tools")
            .unwrap();
    }

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

    // ------------------------------------------------------
    // ------- SAVE/DELETE/RESTORE/DISPLAY CONFIGURATIONS
    // ------------------------------------------------------

    /// Test for command "cargo run --bin build_tool_config -- --save-current-settings config_name"
    #[test]
    fn calling_btc_save_configuration() {
        // PREPARE
        let mut calling_btc_save_current_settings_as_config_name = get_base_cargo_run_command();
        calling_btc_save_current_settings_as_config_name.extend(vec!["--save-current-settings", "config_name"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&calling_btc_save_current_settings_as_config_name[..])
            .succeeds()
            .stdout().contains("Saving current configuration as 'config_name'")
        .unwrap();
    }

    /// Test for command "cargo run --bin build_tool_config -- --delete-settings config_name"
    #[test]
    fn calling_btc_delete_configuration() {
        // PREPARE
        let mut calling_btc_restore_config_name_settings = get_base_cargo_run_command();
        calling_btc_restore_config_name_settings.extend(vec!["--delete-settings", "config_name"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&calling_btc_restore_config_name_settings[..])
            .succeeds()
            .stdout().contains("Delete 'config_name' configuration")
        .unwrap();
    }

    /// Test for command "cargo run --bin build_tool_config -- --restore-settings config_name"
    #[test]
    fn calling_btc_restore_configuration() {
        // PREPARE
        let mut calling_btc_restore_config_name_settings = get_base_cargo_run_command();
        calling_btc_restore_config_name_settings.extend(vec!["--restore-settings", "config_name"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&calling_btc_restore_config_name_settings[..])
            .succeeds()
            .stdout().contains("Restoring current configuration to 'config_name'")
        .unwrap();
    }

    /// Test for command "cargo run --bin build_tool_config -- --list-settings current"
    #[test]
    fn calling_btc_display_current_configuration() {
        // PREPARE
        let mut calling_btc_display_current_configuration = get_base_cargo_run_command();
        calling_btc_display_current_configuration.extend(vec!["--list-settings", "current"]);

        // EXECUTE/ASSERT
        // ! TODO After display output has been decided
        // assert_cli::Assert::command(&calling_btc_display_current_configuration[..])
        //     .succeeds()
        //     .stdout().contains("")
        // .unwrap();
    }

    #[test]
    fn calling_btc_display_saved_configurations() {
        // PREPARE
        let mut callin_btc_display_saved_configurations = get_base_cargo_run_command();
        callin_btc_display_saved_configurations.extend(vec!["--list-settings", "saved"]);

        // EXECUTE/ASSERT
        // ! TODO After display output has been decided
        // assert_cli::Assert::command(&callin_btc_display_saved_configurations[..])
        //     .succeeds()
        //     .stdout().contains("")
        // .unwrap();
    }
}
