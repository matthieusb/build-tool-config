/// This file contains argument handling for proxy setting arguments

use model::arguments::ProxyArguments;

use model::enums::BuildTool;
use model::enums::BuildTool::*;

pub fn handle_proxy_arguments_behavior(proxy_arguments: ProxyArguments, build_tool_chosen: &BuildTool) {
    match (proxy_arguments.http_proxy, proxy_arguments.https_proxy, proxy_arguments.all_proxy)  {
        (Some(http_proxy_value), None, None) => handle_http_proxy_arguments_behavior(http_proxy_value, build_tool_chosen),
        (None, Some(https_proxy_value), None) => handle_https_proxy_arguments_behavior(https_proxy_value, build_tool_chosen),
        (None, None, Some(all_proxy_value)) => handle_all_proxy_arguments_behavior(all_proxy_value, build_tool_chosen),
        (_,_,_) => {}
    }
}

fn handle_http_proxy_arguments_behavior(http_proxy_value: String, build_tool_chosen: &BuildTool) {
    match *build_tool_chosen {
        MAVEN => println!("Setting maven http proxy to {}", http_proxy_value),
        GRADLE => println!("Setting gradle http proxy to {}", http_proxy_value),
        ALL => println!("Setting all tools http proxy to {}", http_proxy_value),
    }
}

fn handle_https_proxy_arguments_behavior(https_proxy_value: String, build_tool_chosen: &BuildTool) {
    match *build_tool_chosen {
        MAVEN => println!("Setting maven https proxy to {}", https_proxy_value),
        GRADLE => println!("Setting gradle https proxy to {}", https_proxy_value),
        ALL => println!("Setting all tools https proxy to {}", https_proxy_value),
    }
}

fn handle_all_proxy_arguments_behavior(all_proxy_value: String, build_tool_chosen: &BuildTool) {
    match *build_tool_chosen {
        MAVEN => println!("Setting maven proxies to {}", all_proxy_value),
        GRADLE => println!("Setting gradle proxies to {}", all_proxy_value),
        ALL => println!("Setting all tools proxies to {}", all_proxy_value)
    }
}