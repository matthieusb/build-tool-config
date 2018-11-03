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

    #[test]
    fn calling_btc_maven_display_proxy_complete_config_file() {
        // PREPARE
        let path_to_resources = get_maven_display_test_resources_path()
            .join("all_proxy_repository_settings_home");
        let args = ["--maven", "--list-settings", "proxy"];

        setup_maven_env_variables(path_to_resources.to_str().unwrap().to_string());

        // EXECUTE/ASSERT
        assert_cli::Assert::main_binary()
            .with_args(&args)   
            .succeeds()
            .stdout().is(indoc!("----------------- MAVEN -----------------
                ---- Http Proxy Setting ----
                http-proxy: localhost:3128
                ---- Https Proxy Setting ----
                https-proxy: localhost:3128
                ---- No Proxy Hosts ----
                no proxy hosts: localhost, *.msb.info, *.test.fr 
            "))
        .unwrap();

        // RESET
        teardown_env_variables();
    }

    #[test]
    fn calling_btc_maven_display_repository_complete_config_file() {
        // PREPARE
        let path_to_resources = get_maven_display_test_resources_path()
            .join("all_proxy_repository_settings_home");
        let args = ["--maven", "--list-settings", "repository"];

        setup_maven_env_variables(path_to_resources.to_str().unwrap().to_string());

        // EXECUTE/ASSERT
        assert_cli::Assert::main_binary()
            .with_args(&args)   
            .succeeds()
            .stdout().is(indoc!("----------------- MAVEN -----------------
                ---- Repository Setting ----
                repository url: http://host-test.msb.info/nexus/content/groups/public-maven/ 
            "))
        .unwrap();

        // RESET
        teardown_env_variables();
    }

    #[test]
    fn calling_btc_maven_display_all_complete_config_file() {
        // PREPARE
        let path_to_resources = get_maven_display_test_resources_path()
            .join("all_proxy_repository_settings_home");
        let args = ["--maven", "--list-settings", "all"];

        setup_maven_env_variables(path_to_resources.to_str().unwrap().to_string());

        // EXECUTE/ASSERT
        assert_cli::Assert::main_binary()
            .with_args(&args)   
            .succeeds()
            .stdout().is(indoc!("----------------- MAVEN -----------------
                ---- Http Proxy Setting ----
                http-proxy: localhost:3128
                ---- Https Proxy Setting ----
                https-proxy: localhost:3128
                ---- No Proxy Hosts ----
                no proxy hosts: localhost, *.msb.info, *.test.fr
                ---- Repository Setting ----
                repository url: http://host-test.msb.info/nexus/content/groups/public-maven/ 
            "))
        .unwrap();

        // RESET
        teardown_env_variables();
    }

    #[test]
    fn calling_btc_maven_display_all_only_http_proxy_config_file() {
        // PREPARE
        let path_to_resources = get_maven_display_test_resources_path()
            .join("http_proxy_settings_home");
        let args = ["--maven", "--list-settings", "all"];

        setup_maven_env_variables(path_to_resources.to_str().unwrap().to_string());

        // EXECUTE/ASSERT
        assert_cli::Assert::main_binary()
            .with_args(&args)   
            .succeeds()
            .stdout().is(indoc!("----------------- MAVEN -----------------
                ---- Http Proxy Setting ----
                http-proxy: localhost:3128
                ---- Https Proxy Setting ----
                Https proxy is not set
                ---- No Proxy Hosts ----
                no proxy hosts: localhost, *.msb.info, *.test.fr
                ---- Repository Setting ----
                Repository is not set
            "))
        .unwrap();

        // RESET
        teardown_env_variables();
    }

    #[test]
    fn calling_btc_maven_display_all_empty_config_file() {
        // PREPARE
        let path_to_resources = get_maven_display_test_resources_path()
            .join("empty_settings_home");
        let args = ["--maven", "--list-settings", "all"];

        setup_maven_env_variables(path_to_resources.to_str().unwrap().to_string());

        // EXECUTE/ASSERT
        assert_cli::Assert::main_binary()
            .with_args(&args)   
            .succeeds()
            .stdout().is(indoc!("----------------- MAVEN -----------------
                ---- Http Proxy Setting ----
                Http proxy is not set
                ---- Https Proxy Setting ----
                Https proxy is not set
                ---- No Proxy Hosts ----
                No proxy hosts are not set
                ---- Repository Setting ----
                Repository is not set
            "))
        .unwrap();

        // RESET
        teardown_env_variables();
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
    fn calling_btc_all_tools_display_proxy_configuration() { // TODO Activate and amend when working on all tools
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
    fn calling_btc_all_tools_display_repository_configuration() { // TODO Activate and amend when working on all tools
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
    fn calling_btc_all_tools_display_all_configuration() { // TODO Activate and amend when working on all tools
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