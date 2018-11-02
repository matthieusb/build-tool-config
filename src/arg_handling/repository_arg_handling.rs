/// This file contains argument handling for repository setting arguments

use model::arguments::RepositoryArguments;

use model::enums::BuildTool;
use model::enums::BuildTool::*;

pub fn handle_repository_arguments_behavior(repository_arguments: RepositoryArguments, build_tool_chosen: &BuildTool) {
    if let Some(repository_value) = repository_arguments.repository  {
        match *build_tool_chosen {
            MAVEN => handle_maven_repository_setting(repository_value),
            GRADLE => println!("Setting gradle repository to {}", repository_value),
            ALL => println!("Setting all tools repository to {}", repository_value),
        }
    }
}

fn handle_maven_repository_setting(repository_value: String) {
    println!("Setting maven repository to {}", repository_value); // TODO Move to once done
}