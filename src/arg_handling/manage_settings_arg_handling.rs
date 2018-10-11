/// This file contains argument handling for managing setting arguments

use model::arguments::ManageSettingsArguments;

use model::enums::BuildTool;
use model::enums::BuildTool::*;

pub fn handle_manage_settings_arguments_behavior(manage_settings_arguments: ManageSettingsArguments, build_tool_chosen: &BuildTool) {
    match (manage_settings_arguments.unset_settings.as_slice(), manage_settings_arguments.save_current_settings,
    manage_settings_arguments.restore_settings, manage_settings_arguments.delete_settings, manage_settings_arguments.display_settings) {
        (unset_settings_values, None, None, None, None) => handle_unset_settings_arguments_behavior(unset_settings_values.to_vec(), build_tool_chosen),
        ([], Some(save_current_settings_value), None, None, None) => handle_save_current_settings_arguments_behavior(save_current_settings_value),
        ([], None, Some(restore_settings_value), None, None) => handle_restore_settings_arguments_behavior(restore_settings_value, build_tool_chosen),
        ([], None, None, Some(delete_settings_value), None) => handle_delete_settings_arguments_behavior(delete_settings_value, build_tool_chosen),
        ([], None, None, None, Some(display_settings_value)) => handle_display_settings_arguments_behavior(display_settings_value, build_tool_chosen),
        (_,_,_,_,_) => {}
    }
}

/// * TODO Documentation
fn handle_unset_settings_arguments_behavior(unset_settings_value: Vec<String>, build_tool_chosen: &BuildTool) {
/*     match *build_tool_chosen {
        MAVEN => println!("Setting maven repository to {}", repository_value),
        GRADLE => println!("Setting gradle repository to {}", repository_value),
        ALL => println!("Setting all tools repository to {}", repository_value),
    } */
}

fn handle_save_current_settings_arguments_behavior(save_current_settings_value: String) {
    println!("Saving current configuration as '{}'", save_current_settings_value);
}

fn handle_restore_settings_arguments_behavior(restore_settings_value: String, build_tool_chosen: &BuildTool) {
    /* match *build_tool_chosen {
        MAVEN => println!("Setting maven repository to {}", repository_value),
        GRADLE => println!("Setting gradle repository to {}", repository_value),
        ALL => println!("Setting all tools repository to {}", repository_value),
    } */
}

fn handle_delete_settings_arguments_behavior(delete_settings_value: String, build_tool_chosen: &BuildTool) {
    /* match *build_tool_chosen {
        MAVEN => println!("Setting maven repository to {}", repository_value),
        GRADLE => println!("Setting gradle repository to {}", repository_value),
        ALL => println!("Setting all tools repository to {}", repository_value),
    } */
}

fn handle_display_settings_arguments_behavior(display_settings_value: String, build_tool_chosen: &BuildTool) {
    /* match *build_tool_chosen {
        MAVEN => println!("Setting maven repository to {}", repository_value),
        GRADLE => println!("Setting gradle repository to {}", repository_value),
        ALL => println!("Setting all tools repository to {}", repository_value),
    } */
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