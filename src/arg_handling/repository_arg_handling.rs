/// This file contains argument handling for repository setting arguments

use model::arguments::RepositoryArguments;

use model::enums::BuildTool;
use model::enums::BuildTool::*;

pub fn handle_proxy_arguments_behavior(repository_arguments: RepositoryArguments, build_tool_chosen: &BuildTool) {
    match repository_arguments.repository  {
        Some(repository_value) => handle_repository_arguments_behavior(repository_value, build_tool_chosen),
        None => {}
    }
}

/// * TODO Documentation
fn handle_repository_arguments_behavior(repository_value: String, build_tool_chosen: &BuildTool) {
    match *build_tool_chosen {
        MAVEN => println!("Setting maven repository to {}", repository_value),
        GRADLE => println!("Setting gradle repository to {}", repository_value),
        ALL => println!("Setting all tools repository to {}", repository_value),
    }
}