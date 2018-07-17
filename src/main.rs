extern crate build_tool_config;

use build_tool_config::model::arguments::Arguments;

fn main() {
    let arguments = Arguments::from_args();
    println!("{:?}", arguments); // ! TODO To remove once features are correctly analyzed
}