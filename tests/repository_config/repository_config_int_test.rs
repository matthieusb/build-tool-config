use common::get_base_cargo_run_command;

#[cfg(test)]
mod repository_config_cli_integration_test {
    use super::*;
    use assert_cli;

    // ! TODO Rewrite Tests using main_binary

    // ------------------------------------------------------
    // ------- SET SETTINGS
    // ------------------------------------------------------

    #[test]
    fn calling_btc_maven_repository_argument() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec![
            "--maven",
            "--set-repository",
            "http://url:port",
        ]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Setting maven repository to http://url:port")
            .unwrap();
    }

    #[test]
    fn calling_btc_gradle_repository_argument() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec![
            "--gradle",
            "--set-repository",
            "http://url:port",
        ]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Setting gradle repository to http://url:port")
            .unwrap();
    }

    #[test]
    fn calling_btc_all_tools_repository_argument() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec![
            "--all-tools",
            "--set-repository",
            "http://url:port",
        ]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Setting all tools repository to http://url:port")
            .unwrap();
    }


    // ------------------------------------------------------
    // ------- UNSET SETTINGS
    // ------------------------------------------------------
    #[test]
    fn calling_btc_unset_repository_settings_maven() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--maven", "--unset-settings", "repository"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Unsetting following settings for maven: repository")
            .unwrap();
    }

    #[test]
    fn calling_btc_unset_repository_settings_gradle() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--gradle", "--unset-settings", "repository"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Unsetting following settings for gradle: repository")
            .unwrap();
    }


    #[test]
    fn calling_btc_unset_repository_settings_all_tools() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--all-tools", "--unset-settings", "repository"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout()
            .contains("Unsetting following settings for all tools: repository")
            .unwrap();
    }


}