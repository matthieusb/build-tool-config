/// Contains settings models to store build tool settings data and behaviors

use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use model::enums::BuildTool;
use model::enums::BuildTool::*;

/// Structure that holds build tool settings.
/// * TODO Some features are missing here (Argument handling shall be changed too):
/// * - http/https proxy no proxy hosts distinction (Maybe use a tuple)
/// * - Repository pattern feature (Using a HashMap might do the trick)
pub struct BuildToolSettings {
    http_proxy: Option<String>,
    https_proxy: Option<String>,
    no_proxy: Vec<String>,
    repository: Option<String>
}

impl BuildToolSettings {
    /// Instantiates a new BuildToolSettings object
    pub fn new(http: Option<String>, https: Option<String>, no: Vec<String>, repo: Option<String>) -> BuildToolSettings {
        BuildToolSettings { http_proxy: http, https_proxy: https, no_proxy: no, repository: repo }
    }
}

/// Handles display of Settings
pub trait SettingsView {
    /// Displays all settings available
    fn display_settings(&self, display_chosen: String, build_tool: &BuildTool);

    /// Display only http and https proxy settings
    fn display_proxy_settings(&self);

    /// Display no proxy hosts settings: does not make the distinction between http and https no proxy hosts
    /// * FIXME Implement http/https no proxy hosts capability
    fn display_no_proxy_settings(&self);

    /// Display repository settings: does not use with repository pattern
    /// * FIXME Implement repository pattern display option
    fn display_repository_settings(&self);
}

impl SettingsView for BuildToolSettings {
    fn display_settings(&self, display_chosen: String, build_tool: &BuildTool) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
        
        match *build_tool {
            MAVEN => println!("----------------- MAVEN -----------------"),
            GRADLE => println!("----------------- GRADLE -----------------"),
            ALL => println!("----------------- ALL TOOLS -----------------"),
        }

        stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))).unwrap();

        match display_chosen.as_ref() { // ? FIXME There might be a better way to do this
            "all" => {        
                self.display_proxy_settings();
                self.display_no_proxy_settings();
                self.display_repository_settings();
            },
            "proxy" => {        
                self.display_proxy_settings();
                self.display_no_proxy_settings();
            },
            "repository" => {
                self.display_repository_settings();
            },
            _ => {}
        }  
    }

    fn display_proxy_settings(&self) {
        let http_proxy = self.http_proxy.clone();
        let https_proxy = self.https_proxy.clone();

        println!("---- Http Proxy Setting ----");
        match http_proxy {
            None => println!("Http proxy is not set"),
            Some(value) => println!("http-proxy: {}", value)
        }

        println!("---- Https Proxy Setting ----");
        match https_proxy {
            None => println!("Https proxy is not set"),
            Some(value) => println!("https-proxy: {}", value)
        }
    }

    fn display_no_proxy_settings(&self) {
        let no_proxy = self.no_proxy.clone();
        println!("---- No Proxy Hosts ----");

        match no_proxy.as_slice() {
            [] => println!("No proxy hosts are not set"),
            values => {
                println!("no proxy hosts: {}", values.join(", "))
            }
        }
    }

    fn display_repository_settings(&self) {
        let repository = self.repository.clone();
        println!("---- Repository Setting ----");

        match repository {
            None => println!("Repository is not set"),
            Some(value) => println!("repository url: {}", value)            
        }
    }
}