use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest;
use reqwest::{Response, Client};

use crate::error::SpasmError;
use crate::vars;

pub async fn fetch(url: &str) -> Result<Response, SpasmError> {
    let client: Client = reqwest::Client::new();
    //let api_key: String = vars::hbip_api().clone();
    let api_key: String;

    match vars::hbip_api().clone(){
        Ok(v) => api_key = v,
        Err(_) => return Err(SpasmError {
            kind: "HIBP".to_string(),
            message: "No api key".to_string(),
        })
    };

    if api_key.is_empty(){
        return Err(SpasmError {
            kind: "HIBP".to_string(),
            message: "No api key".to_string(),
        })
    }
    let res: Response = client
        .get(url)
        .header("hibp-api-key", api_key)
        .header("user-agent", "spasm")
        .send().await?;

    let status_code = res.status().as_u16();

    match status_code {
        200 => return Ok(res),
        400 => {
            return Err(SpasmError {
                kind: "HIBP".to_string(),
                message: "Bad request".to_string(),
            })
        }
        401 => {
            return Err(SpasmError {
                kind: "HIBP".to_string(),
                message: "Unauthorised".to_string(),
            })
        }
        403 => {
            return Err(SpasmError {
                kind: "HIBP".to_string(),
                message: "Forbidden".to_string(),
            })
        }
        404 => {
            return Err(SpasmError {
                kind: "HIBP".to_string(),
                message: "Not found".to_string(),
            })
        }
        429 => {
            return Err(SpasmError {
                kind: "HIBP".to_string(),
                message: "Too many requests".to_string(),
            })
        }
        503 => {
            return Err(SpasmError {
                kind: "HIBP".to_string(),
                message: "Service unavailable".to_string(),
            })
        }
        666 => {
            return Err(SpasmError {
                kind: "HIBP".to_string(),
                message: "No api key available".to_string(),
            })
        }
        _ => {
            return Err(SpasmError {
                kind: "HIBP".to_string(),
                message: "Unknown error".to_string(),
            })
        }
    }
}

pub fn percent_encode(raw: &str) -> String {
    utf8_percent_encode(raw, NON_ALPHANUMERIC).to_string()
}
