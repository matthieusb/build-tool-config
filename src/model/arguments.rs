use structopt::StructOpt;

/// The Arguments type, used to parse arguments from the command line, thanks to the structopt library
/// It contains several other structs which are not subcommands.
#[derive(StructOpt, Debug)]
#[structopt(name = "build_tool_config_arguments")]
pub struct Arguments {
    #[structopt(flatten)]
    pub build_tool_arguments: BuildToolArguments,

    #[structopt(flatten)]
    pub proxy_arguments: ProxyArguments,

    #[structopt(flatten)]
    pub repository_arguments: RepositoryArguments,

    #[structopt(flatten)]
    pub manage_settings_arguments: ManageSettingsArguments
}

impl Arguments {
    /// Re-Implementation of from_args, used to create the structure from the program's cli arguments.
    /// This is mandatory to use this function on an Arguments struct from another module
    pub fn from_args() -> Arguments { 
        <Arguments as StructOpt>::from_args() 
    }
}

/// The BuildToolArguments type. Represents the "-m"/"--maven" "g"/"--gradle" and "-a"/"--all-tols" arguments.
/// These arguments are used to choose which build tool to configure. At least one of them is mandatory
#[derive(StructOpt, Debug)]
#[structopt(name = "build_tools_arguments")]
pub struct BuildToolArguments {
    #[structopt(name = "maven_build_tool", short = "m", long = "maven", group = "build_tools_arguments")]
    /// Configure maven build tool
    pub maven: bool,

    #[structopt(name = "gradle_build_tool", short = "g", long = "gradle", group = "build_tools_arguments")]
    /// Configure gradle build tool
    pub gradle: bool,

    #[structopt(name = "all_build_tools", short = "a", long = "all-tools", group = "build_tools_arguments")]
    /// Configure all build tools
    pub all_tools: bool
}

impl BuildToolArguments {
    // TODO If necessary
}

/// The ProxyArguments type, represents the "--set-http-proxy", "--set-https-proxy" and "--set-proxy" arguments.
/// These arguments are used to choose which proxy to set.
#[derive(StructOpt, Debug)]
#[structopt(name = "proxy_arguments")]
pub struct ProxyArguments {
    #[structopt(name = "http_proxy", long = "set-http-proxy", group = "proxy_arguments")]
    /// Configure http proxy
    pub http_proxy: Option<String>,

    #[structopt(name = "https_proxy", long = "set-https-proxy", group = "proxy_arguments")]
    /// Configure httpŝ proxy
    pub https_proxy: Option<String>,

    #[structopt(name = "all_proxy", long = "set-proxy", group = "proxy_arguments")]
    /// Configure all proxies (http and https)
    pub all_proxy: Option<String>
}

impl ProxyArguments {
    /// Tests whether any proxy argument had been entered
    pub fn proxy_argument_is_present(&self) -> bool {
        return self.http_proxy.is_some() || self.https_proxy.is_some() || self.all_proxy.is_some()
    }
}

/// The RepositoryArguments type, represents the "--set-repository" argument alone.
/// This argument is used to choose which repository to set.
#[derive(StructOpt, Debug)]
#[structopt(name = "repository_arguments")]
pub struct RepositoryArguments {
    #[structopt(name = "repository", long = "set-repository", group = "repository_arguments")]
    /// Configure artifact repository
    pub repository: Option<String>
}

impl RepositoryArguments {
    // -- Getters
    fn get_repository_arg(&self) -> &Option<String> {
        &self.repository
    }
}

/// The ManageSettingsArguments type, represents the "--unset-settings", "--save-current-settings", "--restore-settings" and "--list-settings" arguments
/// These arguments are used to manage preferences on build tools.
#[derive(StructOpt, Debug)]
#[structopt(name = "manage_settings_arguments")]
pub struct ManageSettingsArguments {
    #[structopt(name = "settings-to-unset", long = "unset-settings", group = "manage_settings_arguments")]
    /// Unset parameters, possible values: all, http-proxy, https-proxy, all-proxy, repository
    pub unset_settings: Vec<String>,

    #[structopt(name = "settings-save-name", long = "save-current-settings", group = "manage_settings_arguments")]
    /// Save current configuration to a particular name
    pub save_current_settings: Option<String>,

    #[structopt(name = "settings-delete-name", long = "delete-settings", group = "manage_settings_arguments")]
    pub delete_settings: Option<String>,

    #[structopt(name = "settings-to-restore", long = "restore-settings", group = "manage_settings_arguments")]
    /// Restore pre-stored settings
    pub restore_settings: Option<String>,

    #[structopt(name = "settings-to-display", long = "list-settings", group = "manage_settings_arguments")]
    /// Display all, current or saved settings, possible values; all, current, saved
    pub display_settings: Option<String>
}

impl ManageSettingsArguments {
    // TODO
}

