use crate::{config::Config, Backup};
use std::{cmp::Ordering, env, error::Error, fs::File, io::Read};

pub fn get_disk_token(config: Option<&Config>) -> Result<String, Box<dyn Error>> {
    if let Ok(env_token) = env::var("SYNCOPY_YADISK_OAUTH_TOKEN") {
        return Ok(env_token);
    } else if let Some(cfg) = config {
        if let Some(token) = &cfg.secrets.disk_token {
            return Ok(token.clone());
        }
    }

    let mut file = File::open(".disk_token")?;

    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;

    Ok(file_content.trim().to_string())
}

pub fn get_latest_backup(backups: &[Backup]) -> Option<&Backup> {
    backups.iter().max_by(|a, b| {
        let date_cmp = b.created_at.date().cmp(&a.created_at.date());
        if date_cmp == Ordering::Equal {
            b.created_at.time().cmp(&a.created_at.time())
        } else {
            date_cmp
        }
    })
}

pub fn construct_backup_file_name(prefix: &str, date: &str) -> String {
    format!("{}_{}.tar.gz", prefix, date)
}

pub struct Logger {
    pub enabled: bool,
}

impl Logger {
    pub fn log(&self, message: &str) {
        match &self.enabled {
            true => println!("{}", message),
            false => return,
        }
    }
}
