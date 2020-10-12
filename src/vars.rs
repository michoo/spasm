use std::env::{var, VarError};
use crate::utils;


pub fn hbip_api() -> Result<String, VarError> {
    utils::load_env_variables();

    var("HIBP_API")
}
pub fn diceware_file_path() -> String {
    utils::load_env_variables();

    var("DICEWARE_FILE_PATH").expect("DICEWARE FILE is not set")
}

pub fn password_path() -> String {
    utils::load_env_variables();

    var("PW_PATH").expect("PASSWORD FILE is not set")
}