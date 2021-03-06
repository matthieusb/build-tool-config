use common::*;

#[cfg(test)]
mod proxy_config_cli_integration_test {
    use super::*;
    use assert_cli;

    // ! TODO Rewrite Tests using main_binary

    // ------------------------------------------------------
    // ------- SET SETTINGS
    // ------------------------------------------------------

    // ! TODO Add more accurate tests on stdout

    // ! TODO Add accurate tests on no_proxy values

    #[test]
    fn calling_btc_maven_http_proxy_argument() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--maven", "--set-http-proxy", "http://url:port"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Setting maven http proxy to http://url:port")
            .unwrap();
    }

    #[test]
    fn calling_btc_maven_https_proxy_argument() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--maven", "--set-https-proxy", "http://url:port"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Setting maven https proxy to http://url:port")
            .unwrap();
    }

    #[test]
    fn calling_btc_maven_proxies_argument() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--maven", "--set-proxy", "http://url:port"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Setting maven proxies to http://url:port")
            .unwrap();
    }

    #[test]
    fn calling_btc_maven_proxies_argument_with_no_proxy() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--maven", "--set-proxy", "http://url:port", "--no-proxy", "localhost", "*.msb.fr"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Setting maven proxies to http://url:port")
            .unwrap();
    }

    #[test]
    fn calling_btc_gradle_http_proxy_argument() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--gradle", "--set-http-proxy", "http://url:port"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Setting gradle http proxy to http://url:port")
            .unwrap();
    }

    #[test]
    fn calling_btc_gradle_https_proxy_argument() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--gradle", "--set-https-proxy", "http://url:port"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Setting gradle https proxy to http://url:port")
            .unwrap();
    }

    #[test]
    fn calling_btc_gradle_proxies_argument() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--gradle", "--set-proxy", "http://url:port"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Setting gradle proxies to http://url:port")
            .unwrap();
    }

    #[test]
    fn calling_btc_all_tools_http_proxy_argument() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--all-tools", "--set-http-proxy", "http://url:port"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Setting all tools http proxy to http://url:port")
            .unwrap();
    }

    #[test]
    fn calling_btc_all_tools_https_proxy_argument() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--all-tools","--set-https-proxy", "http://url:port"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Setting all tools https proxy to http://url:port")
            .unwrap();
    }

    #[test]
    fn calling_btc_all_tools_proxies_argument() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--all-tools", "--set-proxy", "http://url:port"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Setting all tools proxies to http://url:port")
            .unwrap();
    }

    // ------------------------------------------------------
    // ------- UNSET SETTINGS
    // ------------------------------------------------------

    #[test]
    fn calling_btc_unset_http_proxy_settings_maven() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--maven", "--unset-settings", "http-proxy"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Unsetting following settings for maven: http-proxy")
            .unwrap();
    }

    #[test]
    fn calling_btc_unset_https_proxy_settings_maven() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--maven", "--unset-settings", "https-proxy"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Unsetting following settings for maven: https-proxy")
            .unwrap();
    }

    #[test]
    fn calling_btc_unset_all_proxy_settings_maven() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--maven", "--unset-settings", "all-proxy"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Unsetting following settings for maven: all-proxy")
            .unwrap();
    }

    #[test]
    fn calling_btc_unset_all_settings_maven() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--maven", "--unset-settings", "all"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Unsetting following settings for maven: all")
            .unwrap();
    }

    #[test]
    fn calling_btc_unset_http_proxy_settings_gradle() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--gradle", "--unset-settings", "http-proxy"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Unsetting following settings for gradle: http-proxy")
            .unwrap();
    }

    #[test]
    fn calling_btc_unset_https_proxy_settings_gradle() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--gradle", "--unset-settings", "https-proxy"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Unsetting following settings for gradle: https-proxy")
            .unwrap();
    }

    #[test]
    fn calling_btc_unset_all_proxy_settings_gradle() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--gradle", "--unset-settings", "all-proxy"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Unsetting following settings for gradle: all-proxy")
            .unwrap();
    }

    #[test]
    fn calling_btc_unset_all_settings_gradle() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--gradle", "--unset-settings", "all"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Unsetting following settings for gradle: all")
            .unwrap();
    }

    #[test]
    fn calling_btc_unset_http_proxy_settings_all_tools() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--all-tools", "--unset-settings", "http-proxy"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Unsetting following settings for all tools: http-proxy")
            .unwrap();
    }

    #[test]
    fn calling_btc_unset_https_proxy_settings_all_tools() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--all-tools", "--unset-settings", "https-proxy"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Unsetting following settings for all tools: https-proxy")
            .unwrap();
    }

    #[test]
    fn calling_btc_unset_all_proxy_settings_all_tools() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--all-tools", "--unset-settings", "all-proxy"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Unsetting following settings for all tools: all-proxy")
            .unwrap();
    }

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
}