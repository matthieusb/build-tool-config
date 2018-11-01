/// * TODO Documentation

extern crate xml;
extern crate dirs;

use std::env;
use std::fs::File;
use std::path::PathBuf;
use self::dirs::home_dir;

use simple_error::SimpleError;
use simple_error::SimpleResult;

use self::xml::reader::{EventReader, XmlEvent};

use model::settings::BuildToolSettings;
use model::enums::MavenXmlConfigNodeType;
use utils::properties_utils::get_property;
use utils::file_utils::get_file_if_exists;


/// Get BuildToolSettings from the maven local configuration
pub fn get_maven_settings_from_home_config() -> SimpleResult<BuildToolSettings> {
    match get_maven_home_config_file_path() {
        Ok(maven_home_path) => {
            Ok(extract_build_tool_settings_from_maven_settings_file(maven_home_path))
        }
        Err(error) => Err(SimpleError::new(format!("An error ocurred getting maven config file path : {}", error)))
    }
}

/// Calls all the methods to extract all settings from a found maven configuration file
fn extract_build_tool_settings_from_maven_settings_file(maven_home_path: PathBuf) -> BuildToolSettings {
    let http_proxy = extract_proxy_settings(get_parser_from_maven_home_path(&maven_home_path).unwrap(), "http");
    let https_proxy = extract_proxy_settings(get_parser_from_maven_home_path(&maven_home_path).unwrap(), "https");
    let repository = extract_repository_settings(get_parser_from_maven_home_path(&maven_home_path).unwrap());
    let no_proxy: Vec<String> = extract_no_proxy_hosts(get_parser_from_maven_home_path(&maven_home_path).unwrap());
    
    BuildToolSettings::new(http_proxy, https_proxy, no_proxy, repository)
}

/// Parse XML to extract no proxy hosts
fn extract_no_proxy_hosts(parser: EventReader<std::fs::File>) -> Vec<String> {
    let mut no_proxy: Vec<String> = Vec::new();

    let mut current_node_type = MavenXmlConfigNodeType::Other;
    for element in parser {
        match element {
            Ok(XmlEvent::StartElement { name, .. }) => {
                match name.local_name.as_ref() {
                    "nonProxyHosts" => current_node_type = MavenXmlConfigNodeType::NoProxyHosts,
                    _ => current_node_type = MavenXmlConfigNodeType::Other,
                }
            }
            Ok(XmlEvent::Characters(value)) => {
                if no_proxy.is_empty() {
                    if let MavenXmlConfigNodeType::NoProxyHosts = current_node_type {
                        for s in value.split('|') {
                            no_proxy.push(s.to_string());
                        }
                    }
                }
            }
            _ => {}
        }
    }
    no_proxy
}

/// * TODO Documentation
/// ! TODO This function should be refactored, it is way too long
fn extract_proxy_settings(parser: EventReader<std::fs::File>, proxy_kind: &str) -> Option<String> {
    let mut host: Option<String> = None;
    let mut port: Option<String> = None;
    let mut protocol: Option<String> = None;
    let mut proxy: Option<String> = None;

    let mut current_node_type = MavenXmlConfigNodeType::Other;
    for element in parser {
        match element {
            Ok(XmlEvent::StartElement { name, .. }) => {
                match name.local_name.as_ref() {
                    "proxy" => current_node_type = MavenXmlConfigNodeType::Proxy,
                    "host" => current_node_type = MavenXmlConfigNodeType::ProxyHost,
                    "port" => current_node_type = MavenXmlConfigNodeType::ProxyPort,
                    "protocol" => current_node_type = MavenXmlConfigNodeType::ProxyProtocol,
                    _ => current_node_type = MavenXmlConfigNodeType::Other,
                }
            }
            Ok(XmlEvent::Characters(value)) => {
                match current_node_type {
                    MavenXmlConfigNodeType::ProxyHost => host = Some(value),
                    MavenXmlConfigNodeType::ProxyPort => port = Some(value),
                    MavenXmlConfigNodeType::ProxyProtocol => protocol = Some(value),
                    _ => {}
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if let "proxy" = name.local_name.as_ref() {
                    if proxy.is_none() {
                        proxy = match (&protocol, &host, &port) {
                            (Some(protocol_value), Some(host_value), Some(port_value)) => {
                                if protocol_value.as_str() == proxy_kind {
                                    Some(format!("{}:{}", host_value, port_value))
                                } else {
                                    None
                                }
                            }
                            _ => None
                        }
                    }
                }
            }
            _ => {}
        }
    }

    proxy
}

fn get_parser_from_maven_home_path(maven_home_path: &PathBuf) -> SimpleResult<EventReader<std::fs::File>> {
    match File::open(maven_home_path) {
        Ok(maven_settings_file) => {
            Ok(EventReader::new(maven_settings_file))
        }
        Err(error) => Err(SimpleError::new(format!("An error ocurred opening maven config file : {}", error)))
    }
}

/// Extracts the repository url from a maven settings file
fn extract_repository_settings(parser: EventReader<std::fs::File>) -> Option<String> {
    let mut repository: Option<String> = None;
    
    let mut current_node_type = MavenXmlConfigNodeType::Other;
    for element in parser {
        match element {
            Ok(XmlEvent::StartElement { name, .. }) => {
                match name.local_name.as_ref() {
                    "mirror" => current_node_type = MavenXmlConfigNodeType::Mirror,
                    "url" => {
                        if current_node_type == MavenXmlConfigNodeType::Mirror {
                            current_node_type = MavenXmlConfigNodeType::MirrorUrl;
                        }
                    }
                    _ => {}
                }
            }
            Ok(XmlEvent::Characters(value)) => {
                if let MavenXmlConfigNodeType::MirrorUrl = current_node_type {
                    repository = Some(value.clone());
                } 
                
            }
            _ => {}
        }
    }

    repository
}

/// Tries all the different ways to get the maven configuration file. 
/// Goes through environment variables first, then the home directory.
fn get_maven_home_config_file_path() -> SimpleResult<PathBuf> { // ? This could be refactored iteraing through a vec rather
    let m2_home_file_path = get_env_var_config_file(get_property("env_vars_keys", "m2_home").unwrap().as_str());

    match m2_home_file_path {
        Ok(_) => m2_home_file_path,
        Err(_) => {
            let m3_home_file_path = get_env_var_config_file(get_property("env_vars_keys", "m3_home").unwrap().as_str());
            match m3_home_file_path {
                Ok(_) => m3_home_file_path,
                Err(_) => {
                    let maven_home_file_path = get_env_var_config_file(get_property("env_vars_keys", "maven_home").unwrap().as_str());
                    match maven_home_file_path {
                        Ok(_) => maven_home_file_path,
                        Err(_) => get_home_dir_config_file()
                    }
                }
            }       
        }
    }
}

/// Gets maven config file path from an environment variable ONLY if the file exists.
/// This can be used with M2_HOME, M3_HOME or MAVEN_HOME
/// Globally, it justs adds "/conf/settings.xml" to the maven variable given.
fn get_env_var_config_file(env_var_key: &str) -> SimpleResult<PathBuf> {
    let env_var = env::var(env_var_key);
    match env_var {
        Ok(env_var_value) => {
            let path_to_settings = PathBuf::from(env_var_value)
                .join(get_property("config_locations", "conf").unwrap().as_str())
                .join(get_property("config_locations", "settings_xml").unwrap().as_str());

            get_file_if_exists(path_to_settings)
        },
        Err(_) => Err(SimpleError::new(format!("Could not find {} env variable", env_var_key)))
    }
}

/// Gets maven config file path from the user home dir ONLY if the file exists. 
/// For example:
/// /home/user/.m2/settings.xml
fn get_home_dir_config_file() -> SimpleResult<PathBuf> {
    match home_dir() {
        Some(home_dir_value) => {
            let path_to_settings = home_dir_value
                .join(get_property("config_locations", "m2").unwrap().as_str())
                .join(get_property("config_locations", "settings_xml").unwrap().as_str());            

            get_file_if_exists(path_to_settings)
        },
        None => Err(SimpleError::new("No home directory could be retrieved"))
    }
}