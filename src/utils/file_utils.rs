use std::path::PathBuf;

use simple_error::SimpleResult;
use simple_error::SimpleError;

pub fn get_file_if_exists(path_to_settings: PathBuf) -> SimpleResult<PathBuf> {
    match path_to_settings.exists() {
        true => Ok(path_to_settings),
        false => Err(SimpleError::new(format!("Could not find the following file {} ", path_to_settings.to_str().unwrap())))
    }
}