use model::arguments::Arguments;
use model::arguments::BuildToolArguments;

use model::enums::BUILD_TOOL;

/// Mother function for argument handling, calls different other functions according to the data provided.
/// The arguments instance here is not a reference because we don't want to use it after invoking this function
pub fn handle_arguments(arguments: Arguments) {
    println!("{:?}", arguments); // ! TODO To remove once features are correctly analyzed

    match arguments {
        Arguments { build_tool_arguments: bta_value, proxy_arguments: pa_value, repository_arguments: ra_value, manage_settings_arguments: msa_value} => {
            match bta_value {
                _ => println!("Pouet")
            }

            
        },
        
        _ => println!("These options do not appear to be handled by the application, try again") // ? TODO Maybe write this on the error channrl
    }

    // ! TODO Use actual error handling with exception and all
}

/// TODO
pub fn handle_build_tool_arguments (build_tool_arguments: BuildToolArguments) -> Result<BUILD_TOOL, String> {
    Ok(BUILD_TOOL::MAVEN)
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