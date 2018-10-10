use common::get_base_cargo_run_command;

#[cfg(test)]
mod manage_config_cli_integration_test {
    use super::*;
    use assert_cli;
    

    // ------------------------------------------------------
    // ------- SAVE/DELETE/RESTORE/DISPLAY CONFIGURATIONS
    // ------------------------------------------------------

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