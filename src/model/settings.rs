/// Contains settings models to store build tool settings data and behaviors

use model::enums::BuildTool;
use model::enums::BuildTool::*;


pub trait SettingsView {
    /// * TODO Documentation
    fn display_settings(&self, display_chosen: String, build_tool: &BuildTool);

    /// * TODO Documentation
    fn display_proxy_settings(&self);

    /// * TODO Documentation
    fn display_no_proxy_settings(&self);

    /// * TODO Documentation
    fn display_repository_settings(&self);
}

pub struct BuildToolSettings {
    http_proxy: Option<String>,
    https_proxy: Option<String>,
    no_proxy: Vec<String>,
    repository: Option<String>
}

/// * TODO Documentation
impl BuildToolSettings {
    pub fn new(http: Option<String>, https: Option<String>, no: Vec<String>, repo: Option<String>) -> BuildToolSettings {
        BuildToolSettings { http_proxy: http, https_proxy: https, no_proxy: no, repository: repo }
    }
}

impl SettingsView for BuildToolSettings {
    fn display_settings(&self, display_chosen: String, build_tool: &BuildTool) {
        match *build_tool {
            MAVEN => println!("----------------- MAVEN -----------------"),
            GRADLE => println!("----------------- GRADLE -----------------"),
            ALL => println!("----------------- ALL TOOLS -----------------"),
        }

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