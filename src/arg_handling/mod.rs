/// This file is the main module for global argument handling. Its takes ang Arguments structopt.
/// 
/// ! Don't forget to update this model if it changes ! 
/// Example without any arguments mentioned in the command :
///
/// Arguments { 
///    build_tool_arguments: BuildToolArguments
///        { maven: false, gradle: false, all_tools: false },
///    proxy_arguments: ProxyArguments
///        { http_proxy: None, https_proxy: None, all_proxy: None },
///    repository_arguments: RepositoryArguments
///        { repository: None },
///    manage_settings_arguments: ManageSettingsArguments
///        { unset_settings: [], save_current_settings: None, restore_settings: None, delete_settings: None, display_settings: None }
///}

use simple_error::SimpleError;
use simple_error::SimpleResult;

use model::arguments::Arguments;
use model::arguments::BuildToolArguments;
use model::enums::BuildTool;

mod proxy_arg_handling;
mod repository_arg_handling;
mod manage_settings_arg_handling;

/// Mother function for argument handling, calls different other functions according to the data provided.
/// The arguments instance here is not a reference because we don't want to use it after invoking this function
pub fn handle_arguments(arguments: Arguments) {
    match arguments {
        Arguments { build_tool_arguments: bta_value, proxy_arguments: pa_value, no_proxy_arguments: npa_value, repository_arguments: ra_value, manage_settings_arguments: msa_value} => {
            match build_tool_arguments_to_enumeration(bta_value) {
                Ok(build_tool_chosen) => {
                    // * FIXME Maybe add a first step check to avoid any "too many arguments problem", so that we don't have to handle those afterwards
                    proxy_arg_handling::handle_proxy_arguments_behavior(pa_value, npa_value, &build_tool_chosen);
                    repository_arg_handling::handle_repository_arguments_behavior(ra_value, &build_tool_chosen);
                    manage_settings_arg_handling::handle_manage_settings_arguments_behavior(msa_value, &build_tool_chosen);
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
        _ => Err(SimpleError::new("Can't find a correct build tool mentioned in the command line"))
    }
}

