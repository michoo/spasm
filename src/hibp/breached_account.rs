use serde::Deserialize;

use crate::error::SpasmError;
use super::http::{fetch, percent_encode};
use chrono::{NaiveDate, DateTime, Utc};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BreachedAccount {
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Breach {
    pub name: String,
    pub title: String,
    pub domain: String,
    pub breach_date: NaiveDate,
    pub added_date: DateTime<Utc>,
    pub modified_date: DateTime<Utc>,
    pub pwn_count: u32,
    pub description: String,
    pub data_classes: Vec<String>,
    pub is_verified: bool,
    pub is_fabricated: bool,
    pub is_sensitive: bool,
    pub is_retired: bool,
    pub is_spam_list: bool,
    pub logo_path: String,
}

pub async fn breached_account_full( account: &str) -> Result<Vec<Breach>, SpasmError> {
    let encoded_account = percent_encode(account);
    let url = format!(
        "https://haveibeenpwned.com/api/v3/breachedaccount/{}?truncateResponse=false",
        encoded_account
    );
    let res = fetch(&url).await?;
    let breaches: Vec<Breach> = res.json::<Vec<Breach>>().await?;
    Ok(breaches)
}


