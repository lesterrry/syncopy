use std::{collections::HashMap, error::Error};
use serde_json::{Value, from_value};
use serde::Deserialize;

use reqwest::header::AUTHORIZATION;

pub const GENERIC_ENDPOINT: &str = "https://cloud-api.yandex.net/v1/disk";

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct DiskItem {
    pub created: String,
    pub modified: String,
    pub name: String,
    pub path: String,
    pub resource_id: String
}

pub struct DiskApi {
    token: String,
    http_client: reqwest::Client
}

impl DiskApi {
    pub fn new(token: String) -> Self {
        DiskApi { token: token, http_client: reqwest::Client::new() }
    }

    pub async fn get_directory_contents(&self, directory: Option<&str>) -> Result<Vec<DiskItem>, Box<dyn Error>> {
        let endpoint = format!("{}/resources/?path=app:/{}", GENERIC_ENDPOINT.to_owned(), directory.unwrap_or(""));
        let response_text = &self.http_client
            .get(endpoint)
            .header(AUTHORIZATION, format!("OAuth {}", &self.token))
            .send()
            .await?
            .text()
            .await?;

        let response = serde_json::from_str::<Value>(&response_text)?;

        let parse_error_message = "JSON parse error.";

        let items: Vec<DiskItem> = from_value(response
            .get("_embedded")
            .ok_or_else(|| parse_error_message)?
            .get("items")
            .ok_or_else(|| parse_error_message)?
            .clone()
        )?;

        Ok(items)
    }
}
