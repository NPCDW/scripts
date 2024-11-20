use std::{collections::HashMap, str::FromStr};

use anyhow::anyhow;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

const USER_AGENT_KEY: &str = "User-Agent";
const USER_AGENT_VALUE: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0";

pub async fn post(url: &str, body: String) -> anyhow::Result<String> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT_KEY, HeaderValue::from_static(USER_AGENT_VALUE));
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let response = client.post(url).body(body).send().await?;
    if !response.status().is_success() {
        return Err(anyhow!("Error: {:?} Error Body: {:?}", response.status(), response.text().await));
    }
    anyhow::Ok(response.text().await?)
}

pub async fn get(url: &str, header_map: HashMap<String, String>) -> anyhow::Result<String> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT_KEY, HeaderValue::from_static(USER_AGENT_VALUE));
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    for ele in header_map {
        headers.insert(HeaderName::from_str(&ele.0)?, HeaderValue::from_str(&ele.1)?);
    }
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let response = client.get(url).send().await?;
    if !response.status().is_success() {
        return Err(anyhow!("Error: {:?} Error Body: {:?}", response.status(), response.text().await));
    }
    anyhow::Ok(response.text().await?)
}