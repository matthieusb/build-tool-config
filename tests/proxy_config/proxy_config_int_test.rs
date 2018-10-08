use common::get_base_cargo_run_command;

#[cfg(test)]
mod proxy_config_cli_integration_test {
    use super::*;
    use assert_cli;

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

    // ------------------------------------------------------
    // ------- UNSET SETTINGS
    // ------------------------------------------------------

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
}