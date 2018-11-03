use std;
use std::env;

use std::path::PathBuf;

pub mod argument_parsing_int_test;

/// Method to get cargo run command line base to launch. Imitates a ./ on the generated executable file.
/// The command piece generated is "cargo run --bin build_tool_config --"
pub fn get_base_cargo_run_command<'a>() -> std::vec::Vec<&'a str> {
    vec!["cargo", "run", "--bin", "build_tool_config", "--"]
} // ! TODO To delete once tests are correctly refactored with main_binary

// ----------------------------------
// -------- PATH
// ----------------------------------

pub fn get_maven_display_test_resources_path() -> PathBuf {
        get_maven_test_resources_path().join("display")
}

pub fn get_maven_test_resources_path() -> PathBuf {
    get_test_resources_path().join("maven")
}

pub fn get_gradle_test_resources_path() -> PathBuf {
    get_test_resources_path().join("gradle")
}

pub fn get_test_resources_path() -> PathBuf {
    std::env::current_dir().unwrap()
        .join("resources")
        .join("tests")
}

// ----------------------------------
// -------- ENV VARIABLES
// ----------------------------------

// * WARNING, envionment variable are shared
// * If you have to use these methods in your tests, don't forget to run them with the --test-threads=1 flag
// * This prevents tests from being run parallel (which is a shame)
// * This should be used if the with_env of assert_cli does not work

/// Sets up envionment variables for both gradle and maven testing (M2_HOME M3_HOME GRADLE_HOME)
pub fn setup_env_variables(maven_home_value: String, gradle_home_value: String) {
    setup_maven_env_variables(maven_home_value);
    setup_gradle_env_variables(gradle_home_value);
}

/// Sets up envionment variables for maven testing (M2_HOME and M3_HOME)
pub fn setup_maven_env_variables(maven_home_value: String) {
    env::set_var("M2_HOME", &maven_home_value);
    env::set_var("M3_HOME", &maven_home_value);
    env::set_var("MAVEN_HOME", maven_home_value);
}

/// Sets up envionment variables for gradle testing (GRADLE_HOME)
pub fn setup_gradle_env_variables(gradle_home_value: String) {
    env::set_var("GRADLE_HOME", gradle_home_value);
}

/// Removes environment variables for both gradle and maven (M2_HOME M3_HOME GRADLE_HOME)
pub fn teardown_env_variables() {
    teardown_maven_env_variables();
    teardown_gradle_env_variables();
}

/// Removes envionment variables for maven testing (M2_HOME and M3_HOME)
pub fn teardown_maven_env_variables() {
    env::remove_var("M2_HOME");
    env::remove_var("M3_HOME");
    env::remove_var("MAVEN_HOME");
}

/// Removes envionment variables for gradle testing (GRADLE_HOME)
pub fn teardown_gradle_env_variables() {
    env::remove_var("GRADLE_HOME");
}