use common::*;

#[cfg(test)]
mod manage_config_cli_integration_test {
    use super::*;
    use assert_cli;
    

    // ------------------------------------------------------
    // ------- DISPLAY CONFIGURATIONS
    // ------------------------------------------------------

    // --------------------------
    // ------- MAVEN
    // --------------------------

    // ! TODO Rewrite Tests using main_binary

    #[test]
    fn calling_btc_maven_display_proxy_configuration() {
        // PREPARE
        setup_maven_env_variables(String::from("M3"));

        let path_to_resources = get_maven_test_resources_path()
            .join("proxy_repository_settings_home");
        let args = ["--maven", "--list-settings", "proxy"];

        // EXECUTE/ASSERT
        assert_cli::Assert::main_binary()
            .with_args(&args)
            .succeeds()
            .stdout().is(indoc!("----------------- MAVEN -----------------
                ---- Http Proxy Setting ----
                http-proxy: localhost:3128
                non http-proxy hosts: localhost|*.msb.info|*.test.fr
                ---- Https Proxy Setting ----
                https-proxy: localhost:3128
                non https-proxy hosts: localhost|*.msb.info|*.test.fr
            "))
        .unwrap();

        // RESET
        teardown_env_variables();
    }

    // TODO Add tests for other settings files

    #[test]
    fn calling_btc_maven_display_repository_configuration() {// TODO
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--maven", "--list-settings", "repository"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout().contains("")
        .unwrap();
    }

    #[test]
    fn calling_btc_maven_display_all_configuration() {// TODO
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--maven", "--list-settings", "all"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout().contains("")
        .unwrap();
    }

    // --------------------------
    // ------- GRADLE
    // --------------------------

    #[test]
    fn calling_btc_gradle_display_proxy_configuration() { // TODO Activate and amend when working on gradle
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--gradle", "--list-settings", "proxy"]);

        // EXECUTE/ASSERT
        // assert_cli::Assert::command(&calling_btc_display_current_configuration[..])
        //     .succeeds()
        //     .stdout().contains("")
        // .unwrap();
    }

    #[test]
    fn calling_btc_gradle_display_repository_configuration() { // TODO Activate and amend when working on gradle
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--gradle", "--list-settings", "repository"]);

        // EXECUTE/ASSERT
        // assert_cli::Assert::command(&calling_btc_display_current_configuration[..])
        //     .succeeds()
        //     .stdout().contains("")
        // .unwrap();
    }

    #[test]
    fn calling_btc_gradle_display_all_configuration() { // TODO Activate and amend when working on gradle
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--gradle", "--list-settings", "all"]);

        // EXECUTE/ASSERT
        // assert_cli::Assert::command(&callin_btc_display_saved_configurations[..])
        //     .succeeds()
        //     .stdout().contains("")
        // .unwrap();
    }

    // --------------------------
    // ------- ALL TOOLS
    // --------------------------

    #[test]
    fn calling_btc_all_tools_display_proxy_configuration() { // TODO Activate and amend when working on gradle
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--all-tools", "--list-settings", "proxy"]);

        // EXECUTE/ASSERT
        // assert_cli::Assert::command(&calling_btc_display_current_configuration[..])
        //     .succeeds()
        //     .stdout().contains("")
        // .unwrap();
    }

    #[test]
    fn calling_btc_all_tools_display_repository_configuration() { // TODO Activate and amend when working on gradle
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--all-tools", "--list-settings", "repository"]);

        // EXECUTE/ASSERT
        // assert_cli::Assert::command(&calling_btc_display_current_configuration[..])
        //     .succeeds()
        //     .stdout().contains("")
        // .unwrap();
    }

    #[test]
    fn calling_btc_all_tools_display_all_configuration() { // TODO Activate and amend when working on gradle
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--all-tools", "--list-settings", "all"]);

        // EXECUTE/ASSERT
        // assert_cli::Assert::command(&callin_btc_display_saved_configurations[..])
        //     .succeeds()
        //     .stdout().contains("")
        // .unwrap();
    }

}