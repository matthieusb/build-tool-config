/// This file contains argument handling for managing setting arguments

use model::arguments::ManageSettingsArguments;
use model::enums::BuildTool;
use model::enums::BuildTool::*;
use model::settings::SettingsView;

use file_manager::maven_manager::*;

pub fn handle_manage_settings_arguments_behavior(manage_settings_arguments: ManageSettingsArguments, build_tool_chosen: &BuildTool) {
    match (manage_settings_arguments.unset_settings.as_slice(), manage_settings_arguments.display_settings) {
        ([], Some(display_settings_value)) => handle_display_settings_arguments_behavior(display_settings_value, build_tool_chosen),
        (unset_settings_values, None) => {
            if !unset_settings_values.is_empty() {
                handle_unset_settings_arguments_behavior(unset_settings_values.to_vec(), build_tool_chosen)
            }
        },
        (_,_) => {}
    }
}

fn handle_unset_settings_arguments_behavior(unset_settings_value: Vec<String>, build_tool_chosen: &BuildTool) {
    match *build_tool_chosen {
        MAVEN => handle_unset_maven_settings_arguments_behavior(unset_settings_value),
        GRADLE => handle_unset_gradle_settings_arguments_behavior(unset_settings_value),
        ALL => handle_unset_all_tools_settings_arguments_behavior(unset_settings_value)
    }
}

fn handle_unset_maven_settings_arguments_behavior(unset_settings_value: Vec<String>) {
    let unset_settings_joined = unset_settings_value.join(" ");
    println!("Unsetting following settings for maven: {}", unset_settings_joined);
}

fn handle_unset_gradle_settings_arguments_behavior(unset_settings_value: Vec<String>) {
    let unset_settings_joined = unset_settings_value.join(" ");
    println!("Unsetting following settings for gradle: {}", unset_settings_joined);
}

fn handle_unset_all_tools_settings_arguments_behavior(unset_settings_value: Vec<String>) {
    let unset_settings_joined = unset_settings_value.join(" ");
    println!("Unsetting following settings for all tools: {}", unset_settings_joined);
}

fn handle_display_settings_arguments_behavior(display_settings_value: String, build_tool_chosen: &BuildTool) {
    match *build_tool_chosen {
        MAVEN => {
            match get_maven_settings_from_home_config() {
                Ok(build_tool_settings) => build_tool_settings.display_settings(display_settings_value, build_tool_chosen),
                Err(error) => eprintln!("{}", error)
            }
        }
        GRADLE => println!("TODO-GRAGLE"),
        ALL => println!("TODO-ALL"),
    }
}