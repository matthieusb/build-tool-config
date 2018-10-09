use simple_error::SimpleError;
use simple_error::SimpleResult;

use model::arguments::Arguments;
use model::arguments::BuildToolArguments;
use model::enums::BuildTool;

mod proxy_arg_handling;

/// Mother function for argument handling, calls different other functions according to the data provided.
/// The arguments instance here is not a reference because we don't want to use it after invoking this function
pub fn handle_arguments(arguments: Arguments) {
    // ! TODO Use actual error handling with custom errors

    match arguments {
        Arguments { build_tool_arguments: bta_value, proxy_arguments: pa_value, repository_arguments: ra_value, manage_settings_arguments: msa_value} => {
            match build_tool_arguments_to_enumeration(bta_value) {
                Ok(build_tool_chosen) => {
                    proxy_arg_handling::handle_proxy_arguments_behavior(pa_value, build_tool_chosen);
                    
                    // TODO Add other arguments
                },
                Err(build_tool_chosen_error) => eprintln!("{}", build_tool_chosen_error)
            }
        },
    }   
}

/// Determines the Enum value of the chosen build tool
fn build_tool_arguments_to_enumeration (build_tool_arguments: BuildToolArguments) -> SimpleResult<BuildTool> {
    match build_tool_arguments {
        BuildToolArguments { maven: true, gradle: false, all_tools: false } => Ok(BuildTool::MAVEN),
        BuildToolArguments { maven: false, gradle: true, all_tools: false } => Ok(BuildTool::GRADLE),
        BuildToolArguments { maven: false, gradle: false, all_tools: true } => Ok(BuildTool::ALL),
        _ => Err(SimpleError::new("cannot do foo"))
    }
}

/*
Example without any arguments:

Arguments { 
    build_tool_arguments: BuildToolArguments
        { maven: false, gradle: false, all_tools: false },
    proxy_arguments: ProxyArguments
        { http_proxy: None, https_proxy: None, all_proxy: None },
    repository_arguments: RepositoryArguments
        { repository: None },
    manage_settings_arguments: ManageSettingsArguments
        { unset_settings: [], save_current_settings: None, restore_settings: None, delete_settings: None, display_settings: None }
}
*/