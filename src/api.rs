use std::{collections::HashMap, error::Error};
use serde_json::Value;

use reqwest::header::AUTHORIZATION;

mod endpoints {
    pub const GENERIC: &str = "https://cloud-api.yandex.net/v1/disk";
}

pub struct DiskApi {
    token: String,
    http_client: reqwest::Client
}

impl DiskApi {
    pub fn new(token: String) -> Self {
        DiskApi { token: token, http_client: reqwest::Client::new() }
    }

    pub async fn get_dir_contents(&self, directory: Option<&str>) -> Result<Vec<&str>, Box<dyn Error>> {
        let endpoint = format!("{}/resources/?path=app:/{}", endpoints::GENERIC.to_owned(), directory.unwrap_or(""));
        let response_text = &self.http_client
            .get(endpoint)
            .header(AUTHORIZATION, format!("OAuth {}", &self.token))
            .send()
            .await?
            .text()
            .await?;

        let response = serde_json::from_str::<Value>(&response_text).expect("JSON parse error");
        println!("{:#?}", response);
        Ok(vec!(""))
    }
}
