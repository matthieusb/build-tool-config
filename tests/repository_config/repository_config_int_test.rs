use common::*;

use assert_fs::prelude::*;
use assert_fs::TempDir;

use predicates::prelude::*;

#[cfg(test)]
mod repository_config_cli_integration_test {
    use super::*;
    use assert_cli;

    // ! TODO Rewrite Tests using main_binary

    // ------------------------------------------------------
    // ------- SET SETTINGS
    // ------------------------------------------------------

    // --------------------------
    // ------- MAVEN
    // --------------------------

    #[test]
    fn calling_btc_maven_repository_argument() {
        // PREPARE: setup temp dir to create a new file in it
        let temp_parent_dir = TempDir::new().unwrap();
        let path_to_parent_dir = temp_parent_dir.path();
        let temp_conf_dir = TempDir::new_in(path_to_parent_dir).unwrap(); // This will not work, we need a /conf directory inside our repository folder
        let path_to_resources = temp_conf_dir.path();
        
        println!("Path to temp dir: {:?}", path_to_resources); // ! TODO Remove this

        let args = ["--maven", "--set-repository", "http://url:port"];

        setup_maven_env_variables(path_to_resources.to_str().unwrap().to_string());

        // EXECUTE/ASSERT
        assert_cli::Assert::main_binary()
            .with_args(&args)
            .succeeds()
            .stdout().is("Setting maven repository to http://url:port")
        .unwrap();

        // ! TODO Complete this test

        // RESET
        teardown_env_variables();
    }

    // TODO Create test that also create a temp file to test modification rather than pure creatin

    // --------------------------
    // ------- GRADLE
    // --------------------------

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

    // --------------------------
    // ------- ALL TOOLS
    // --------------------------

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

    // --------------------------
    // ------- MAVEN
    // --------------------------
    
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

    // --------------------------
    // ------- GRADLE
    // --------------------------

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

    // --------------------------
    // ------- ALL TOOLS
    // --------------------------

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