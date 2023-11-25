use std::{error::Error, env, fs::File, io::Read, cmp::Ordering};
use chrono::NaiveDateTime;

use crate::Backup;

pub fn get_token() -> Result<String, Box<dyn Error>> {
    if let Ok(env_token) = env::var("SYNCOPY_YADISK_OAUTH_TOKEN") {
        Ok(env_token)
    } else {
        let mut file = File::open(".token")?;

        let mut file_content = String::new();
        file.read_to_string(&mut file_content)?;

        Ok(file_content.trim().to_string())
    }
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


pub struct Logger {
    pub verbose: bool
}

impl Logger {
    pub fn log(&self, message: &str) {
        match &self.verbose {
            true => println!("{}", message),
            false => return
        }
    }
}