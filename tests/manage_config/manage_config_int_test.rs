use common::get_base_cargo_run_command;

#[cfg(test)]
mod manage_config_cli_integration_test {
    use super::*;
    use assert_cli;
    

    // ------------------------------------------------------
    // ------- DISPLAY CONFIGURATIONS
    // ------------------------------------------------------

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