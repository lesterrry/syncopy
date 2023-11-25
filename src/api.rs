use std::{collections::HashMap, error::Error};

mod endpoints {
    pub const GENERIC: &str = "https://cloud-api.yandex.net/v1/disk";
}

pub struct DiskApi {
    pub token: String,
}

impl DiskApi {
    async fn get_dir_contents(directory: &str) -> Result<Vec<&str>, Box<dyn Error>> {
        let endpoint = format!("{}/resources/?path=app:/{}/", endpoints::GENERIC.to_owned(), directory);
        let resp = reqwest::get("https://httpbin.org/ip")
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        println!("{:#?}", resp);
        Ok(vec!(""))
    }
}
