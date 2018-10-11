/// This file contains argument handling for managing setting arguments

use model::arguments::ManageSettingsArguments;

use model::enums::BuildTool;
use model::enums::BuildTool::*;

pub fn handle_manage_settings_arguments_behavior(manage_settings_arguments: ManageSettingsArguments, build_tool_chosen: &BuildTool) {
    match (manage_settings_arguments.unset_settings.as_slice(), manage_settings_arguments.save_current_settings,
    manage_settings_arguments.restore_settings, manage_settings_arguments.delete_settings, manage_settings_arguments.display_settings) {
        (unset_settings_values, None, None, None, None) => handle_unset_settings_arguments_behavior(unset_settings_values.to_vec(), build_tool_chosen),
        ([], Some(save_current_settings_value), None, None, None) => handle_save_current_settings_arguments_behavior(save_current_settings_value, build_tool_chosen),
        ([], None, Some(restore_settings_value), None, None) => handle_restore_settings_arguments_behavior(restore_settings_value, build_tool_chosen),
        ([], None, None, Some(delete_settings_value), None) => handle_delete_settings_arguments_behavior(delete_settings_value, build_tool_chosen),
        ([], None, None, None, Some(display_settings_value)) => handle_display_settings_arguments_behavior(display_settings_value, build_tool_chosen),
        (_,_,_,_,_) => {}
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

fn handle_save_current_settings_arguments_behavior(save_current_settings_value: String, build_tool_chosen: &BuildTool) {
    match *build_tool_chosen {
        MAVEN => println!("Saving current maven configuration as '{}'", save_current_settings_value),
        GRADLE => println!("Saving current gradle configuration as '{}'", save_current_settings_value),
        ALL => println!("Saving all configurations for current tools as '{}'", save_current_settings_value),
    }
}

fn handle_restore_settings_arguments_behavior(restore_settings_value: String, build_tool_chosen: &BuildTool) {
    match *build_tool_chosen {
        MAVEN => println!("Restoring current maven configuration to '{}'", restore_settings_value),
        GRADLE => println!("Restoring current gradle configuration to '{}'", restore_settings_value),
        ALL => println!("Restoring current all tools configuration to '{}'", restore_settings_value),
    }
}

fn handle_delete_settings_arguments_behavior(delete_settings_value: String, build_tool_chosen: &BuildTool) {
    match *build_tool_chosen {
        MAVEN => println!("Delete '{}' configuration for maven", delete_settings_value),
        GRADLE => println!("Delete '{}' configuration for gradle", delete_settings_value),
        ALL => println!("Delete '{}' configuration for all tools", delete_settings_value),
    }
}

fn handle_display_settings_arguments_behavior(display_settings_value: String, build_tool_chosen: &BuildTool) {
    // TODO Once the tests determine the display needed, develop this
}

/*  Arguments { 
    build_tool_arguments: BuildToolArguments
        { maven: false, gradle: false, all_tools: false },
    proxy_arguments: ProxyArguments
        { http_proxy: None, https_proxy: None, all_proxy: None },
    repository_arguments: RepositoryArguments
        { repository: None },
    manage_settings_arguments: ManageSettingsArguments
        { unset_settings: [], save_current_settings: None, restore_settings: None, delete_settings: None, display_settings: None }
} */