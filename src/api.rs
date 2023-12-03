use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use serde_json::{from_value, Value};
use std::{error::Error, fs::File, io::Read};

pub const GENERIC_ENDPOINT: &str = "https://cloud-api.yandex.net/v1/disk";

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct DiskItem {
    pub created: String,
    pub modified: String,
    pub name: String,
    pub path: String,
    pub resource_id: String,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct UploadOperation {
    pub operation_id: String,
    pub href: String,
    pub method: String,
}

pub struct DiskApi {
    token: String,
    http_client: reqwest::Client,
}

impl DiskApi {
    pub fn new(token: String) -> Self {
        DiskApi {
            token: token,
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn get_directory_contents(
        &self,
        directory: Option<&str>,
    ) -> Result<Vec<DiskItem>, Box<dyn Error>> {
        let endpoint = format!(
            "{}/resources/?path=app:/{}",
            GENERIC_ENDPOINT.to_owned(),
            directory.unwrap_or("")
        );

        let response_text = &self.get_request_text(&endpoint).await?;

        let response = serde_json::from_str::<Value>(&response_text)?;

        let parse_error_message = "JSON parse error";

        let items: Vec<DiskItem> = from_value(
            response
                .get("_embedded")
                .ok_or_else(|| parse_error_message)?
                .get("items")
                .ok_or_else(|| parse_error_message)?
                .clone(),
        )?;

        Ok(items)
    }

    pub async fn get_upload_operation(
        &self,
        file_path: &str,
    ) -> Result<UploadOperation, Box<dyn Error>> {
        let endpoint = format!(
            "{}/resources/upload/?path=app:/{}",
            GENERIC_ENDPOINT.to_owned(),
            file_path
        );

        let response_text = &self.get_request_text(&endpoint).await?;

        let response = serde_json::from_str::<UploadOperation>(&response_text)?;
        if response.href.is_empty() || response.method != "PUT" {
            return Err(format!(
                "Couldn't get upload operation (method: {}, href: {})",
                response.href, response.method
            )
            .into());
        }

        Ok(response)
    }

    pub async fn upload_file(
        &self,
        url: &str,
        file_path: &str,
    ) -> Result<Option<String>, Box<dyn Error>> {
        let mut file = File::open(file_path)?;

        let mut file_contents = Vec::new();

        file.read_to_end(&mut file_contents)?;

        let response = &self.http_client.put(url).body(file_contents).send().await?;
        assert!(response.status() == 201 || response.status() == 202);
        if response.status() != 201 && response.status() != 202 {
            return Err(format!("Request failed with status {}", response.status()).into());
        }

        match response.headers()["location"].to_str() {
            Ok(value) => Ok(Some(value.to_owned())),
            Err(_) => Ok(None),
        }
    }

    async fn get_request_text(&self, endpoint: &str) -> Result<String, Box<dyn Error>> {
        let response = self
            .http_client
            .get(endpoint)
            .header(AUTHORIZATION, format!("OAuth {}", &self.token))
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(response.text().await.unwrap_or("Unknown response error".to_string()).into())
        }
        let text = response
            .text()
            .await?;

        Ok(text.to_string())
    }
}
