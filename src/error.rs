
use reqwest;

#[derive(Debug)]
pub struct SpasmError {
    pub kind: String,
    pub message: String,
}

impl From<reqwest::Error> for SpasmError {
    fn from(error: reqwest::Error) -> Self {
        SpasmError {
            kind: String::from("reqwest"),
            message: error.to_string(),
        }
    }
}

impl From<std::io::Error> for SpasmError {
    fn from(error: std::io::Error) -> Self {
        SpasmError {
            kind: String::from("std::io::Error"),
            message: error.to_string(),
        }
    }
}