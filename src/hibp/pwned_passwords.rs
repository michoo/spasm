use reqwest::Response;
use sha1::{Digest, Sha1};

use crate::error::SpasmError;
use super::http::fetch;
use std::str::FromStr;

const PWNED_PASSWORD_URL: &str = "https://api.pwnedpasswords.com/range";

pub struct PwnedPassword {
    pub pwned: bool,
    pub times: u32,
}

struct HashedPassword {
    prefix: String,
    suffix: String,
}

fn hash(raw: &str) -> HashedPassword {
    let mut hasher = Sha1::new();
    hasher.input(raw);
    let result = hasher.result();
    let hash = format!("{:x}", result);
    let prefix = hash[0..5].to_owned();
    let suffix = hash[5..].to_owned();
    HashedPassword { prefix, suffix }
}

async fn check_password_hash(
    res: Response,
    hashed_password: HashedPassword,
) -> Result<PwnedPassword, SpasmError> {
    let res_text = res.text().await?;
    for hash in res_text.lines() {
        let hash_vec = hash.split(':').collect::<Vec<&str>>();
        if hash_vec[0] == hashed_password.suffix.to_uppercase() {
            let times = u32::from_str(hash_vec[1]).unwrap_or(0);
            return Ok(PwnedPassword { pwned: true, times });
        }
    }

    Ok(PwnedPassword {
        pwned: false,
        times: 0,
    })
}


pub async fn pwned_passwords( password: &str) -> Result<PwnedPassword, SpasmError> {
    let hashed_password = hash(password);
    let url = format!("{}/{}", PWNED_PASSWORD_URL, hashed_password.prefix);
    let res = fetch(&url).await?;
    check_password_hash(res, hashed_password).await
}

