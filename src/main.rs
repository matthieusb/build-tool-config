#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "build_tool_config_arguments")]
struct Arguments {
    #[structopt(flatten)]
    build_tool_arguments: BuildToolArguments,

    #[structopt(flatten)]
    proxy_arguments: ProxyArguments,

    #[structopt(flatten)]
    repository_arguments: RepositoryArguments,

    #[structopt(flatten)]
    manage_settings_arguments: ManageSettingsArguments
}

#[derive(StructOpt, Debug)]
#[structopt(name = "build_tools_arguments")]
struct BuildToolArguments {
    #[structopt(name = "maven_build_tool", short = "m", long = "maven", group = "build_tools_arguments")]
    /// Configure maven build tool
    maven: bool,

    #[structopt(name = "gradle_build_tool", short = "g", long = "gradle", group = "build_tools_arguments")]
    /// Configure gradle build tool
    gradle: bool,

    #[structopt(name = "all_build_tools", short = "a", long = "all-tools", group = "build_tools_arguments")]
    /// Configure all build tools
    all_tools: bool
}

#[derive(StructOpt, Debug)]
#[structopt(name = "proxy_arguments")]
struct ProxyArguments {
    #[structopt(name = "http_proxy", long = "set-http-proxy", group = "proxy_arguments")]
    /// Configure http proxy
    http_proxy: Option<String>,

    #[structopt(name = "https_proxy", long = "set-https-proxy", group = "proxy_arguments")]
    /// Configure httpŝ proxy
    https_proxy: Option<String>,

    #[structopt(name = "all_proxy", long = "set-proxy", group = "proxy_arguments")]
    /// Configure all proxies (http and https)
    all_proxy: Option<String>
}

#[derive(StructOpt, Debug)]
#[structopt(name = "repository_arguments")]
struct RepositoryArguments {
    #[structopt(name = "repository", long = "set-repository", group = "repository_arguments")]
    /// Configure artifact repository
    repository: Option<String>
}

#[derive(StructOpt, Debug)]
#[structopt(name = "manage_settings_arguments")]
struct ManageSettingsArguments {
    #[structopt(name = "settings-to-unset", long = "unset-settings", group = "manage_settings_arguments")]
    /// Unset parameters, possible values: all, http-proxy, https-proxy, all-proxy, repository
    unset_settings: Vec<String>,

    #[structopt(name = "settings-save-name", long = "save-current-settings", group = "manage_settings_arguments")]
    /// Save current configuration to a particular name
    save_current_settings: Option<String>,

    #[structopt(name = "settings-to-restore", long = "restore-settings", group = "manage_settings_arguments")]
    /// Restore pre-stored settings
    restore_settings: Option<String>,

    #[structopt(name = "settings-to-display", long = "list-settings", group = "manage_settings_arguments")]
    /// Display all, current or saved settings, possible values; all, current, saved
    display_settings: Option<String>
}

fn main() {
    let arguments = handle_arguments();
    println!("{:?}", arguments);
}

fn handle_arguments() -> Arguments {
    Arguments::from_args()
}