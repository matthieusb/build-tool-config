use std::path::PathBuf;

use simple_error::SimpleResult;
use simple_error::SimpleError;

/// Gets a two-layer property value from the PROPERTIES defined on app startup
/// 
/// ```
/// use std::path::PathBuf;
/// use build_tool_config::utils::file_utils::get_file_if_exists;
/// 
/// let existing_file_result = get_file_if_exists(PathBuf::from("resources").join("tests").join(".gitkeep"));
/// assert!(existing_file_result.is_ok());
/// 
/// let non_existing_file_result = get_file_if_exists(PathBuf::from("resources").join("tests").join(".notatallhere"));
/// assert!(non_existing_file_result.is_err());
/// ```
pub fn get_file_if_exists(path_to_settings: PathBuf) -> SimpleResult<PathBuf> {
    match path_to_settings.exists() {
        true => Ok(path_to_settings),
        false => Err(SimpleError::new(format!("Could not find the following file {} ", path_to_settings.to_str().unwrap())))
    }
}