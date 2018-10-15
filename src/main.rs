extern crate build_tool_config;

use build_tool_config::model::arguments::Arguments;
use build_tool_config::arg_handling::handle_arguments;

/// Main function of this program, simply calls the argument handling.
fn main() {
    let arguments = Arguments::from_args();
    handle_arguments(arguments);
}