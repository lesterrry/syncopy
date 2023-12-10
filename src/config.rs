use serde::Deserialize;
use std::{error::Error, fs::File, io::Read};

#[derive(Deserialize, Debug)]
pub struct Backups {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    pub output_directory: String,
    pub output_suffix: String
}

#[derive(Deserialize, Debug)]
pub struct Secrets {
    pub disk_token: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub backups: Backups,
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
