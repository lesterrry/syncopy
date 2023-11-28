use serde::Deserialize;
use std::{error::Error, fs::File, io::Read};

#[derive(Deserialize, Debug)]
pub struct Backups {
    pub input_directory: String,
    pub output_directory: String,
}

#[derive(Deserialize, Debug)]
pub struct Cronitor {
    pub enabled: bool,
}

#[derive(Deserialize, Debug)]
pub struct Secrets {
    pub disk_token: Option<String>,
    pub cronitor_token: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub backups: Backups,
    pub cronitor: Cronitor,
    pub secrets: Secrets,
}

impl Config {
    pub fn parse(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let decoded: Config = toml::from_str(&contents)?;

        Ok(decoded)
    }
}
