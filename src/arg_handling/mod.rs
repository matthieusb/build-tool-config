use model::arguments::Arguments;
use model::arguments::BuildToolArguments;
use model::enums::BuildTool;

use simple_error::SimpleError;
use simple_error::SimpleResult;

/// Mother function for argument handling, calls different other functions according to the data provided.
/// The arguments instance here is not a reference because we don't want to use it after invoking this function
pub fn handle_arguments(arguments: Arguments) {
    println!("{:?}", arguments); // ! TODO To remove once features are correctly analyzed

    // ! TODO Use actual error handling with custom errors

    match arguments {
        Arguments { build_tool_arguments: bta_value, proxy_arguments: pa_value, repository_arguments: ra_value, manage_settings_arguments: msa_value} => {
            match build_tool_arguments_to_enumeration(bta_value) {
                Ok(returned_value) => print!("Yay"),
                Err(returned_error) => print!("{}", returned_error)
            }

            
        },
    }

    
}

/// Determines the Enum value of the chosen build tool
pub fn build_tool_arguments_to_enumeration (build_tool_arguments: BuildToolArguments) -> SimpleResult<BuildTool> {
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