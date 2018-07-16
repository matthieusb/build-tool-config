extern crate build_tool_config;

use build_tool_config::model::arguments::Arguments;

fn main() {
    let arguments = handle_arguments();
    println!("{:?}", arguments);
}

fn handle_arguments() -> Arguments {
    Arguments::from_args()
}