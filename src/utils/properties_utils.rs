use super::config::*;
use std::sync::RwLock;

/// Static declarations of global variables
lazy_static! {
    static ref PROPERTIES: RwLock<Config> = RwLock::new({
        let mut properties = Config::default();
        properties.merge(File::with_name("resources/main/config/properties.toml")).unwrap();
        properties
    });
}

pub fn get_property(property_first_level: &str, property_key: &str) -> Option<String> {
    match &PROPERTIES.read().unwrap().get_table(property_first_level) {
        Ok(properties) => {
            match properties.get(property_key) {
                Some(property_value) => Some(property_value.clone().into_str().unwrap()),
                _ => None
            }  
        },
        _ => None
    }
}