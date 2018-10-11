use common::get_base_cargo_run_command;

#[cfg(test)]
mod manage_config_cli_integration_test {
    use super::*;
    use assert_cli;
    

    // ------------------------------------------------------
    // ------- SAVE/DELETE/RESTORE/DISPLAY CONFIGURATIONS
    // ------------------------------------------------------

    #[test]
    fn calling_btc_save_maven_configuration() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--maven", "--save-current-settings", "config_name"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout().contains("Saving current maven configuration as 'config_name'")
        .unwrap();
    }

    #[test]
    fn calling_btc_save_gradle_configuration() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--gradle", "--save-current-settings", "config_name"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout().contains("Saving current gradle configuration as 'config_name'")
        .unwrap();
    }

    #[test]
    fn calling_btc_save_all_configuration() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--all-tools", "--save-current-settings", "config_name"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout().contains("Saving all configurations for current tools as 'config_name'")
        .unwrap();
    }

    #[test]
    fn calling_btc_delete_maven_configuration() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--maven", "--delete-settings", "config_name"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout().contains("Delete 'config_name' configuration for maven")
        .unwrap();
    }

    #[test]
    fn calling_btc_delete_gradle_configuration() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--gradle", "--delete-settings", "config_name"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout().contains("Delete 'config_name' configuration for gradle")
        .unwrap();
    }

    #[test]
    fn calling_btc_delete_all_configuration() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--all-tools", "--delete-settings", "config_name"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout().contains("Delete 'config_name' configuration for all tools")
        .unwrap();
    }

    #[test]
    fn calling_btc_restore_maven_configuration() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--maven", "--restore-settings", "config_name"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout().contains("Restoring current maven configuration to 'config_name'")
        .unwrap();
    }

    #[test]
    fn calling_btc_restore_gradle_configuration() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--gradle", "--restore-settings", "config_name"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout().contains("Restoring current gradle configuration to 'config_name'")
        .unwrap();
    }

    #[test]
    fn calling_btc_restore_all_tools_configuration() {
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--all-tools", "--restore-settings", "config_name"]);

        // EXECUTE/ASSERT
        assert_cli::Assert::command(&command[..])
            .succeeds()
            .stdout().contains("Restoring current all tools configuration to 'config_name'")
        .unwrap();
    }

    #[test]
    fn calling_btc_display_current_configuration() {// ! TODO Add maven/gradle/all-tools handling
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--list-settings", "current"]);

        // EXECUTE/ASSERT
        // ! TODO After display output has been decided
        // assert_cli::Assert::command(&calling_btc_display_current_configuration[..])
        //     .succeeds()
        //     .stdout().contains("")
        // .unwrap();
    }

    #[test]
    fn calling_btc_display_saved_configurations() {// ! TODO Add maven/gradle/all-tools handling
        // PREPARE
        let mut command = get_base_cargo_run_command();
        command.extend(vec!["--list-settings", "saved"]);

        // EXECUTE/ASSERT
        // ! TODO After display output has been decided
        // assert_cli::Assert::command(&callin_btc_display_saved_configurations[..])
        //     .succeeds()
        //     .stdout().contains("")
        // .unwrap();
    }

}