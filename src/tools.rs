use crate::{config::Config, Backup};
use chrono::NaiveDateTime;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::{cmp::Ordering, env, error::Error, fs::File, io::Read, path::Path};

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
    backups.iter().min_by(|a, b| {
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

pub fn get_delta_string(a: NaiveDateTime, b: NaiveDateTime) -> String {
    let duration = a.signed_duration_since(b);
    let seconds = duration.num_seconds();
    match seconds {
        0..=59 => format!("{}s", seconds),                     // 0 to 59 seconds
        60..=3599 => format!("{}m", seconds / 60),             // 1 minute to 59 minutes
        3600..=86399 => format!("{}h", seconds / 3600),        // 1 hour to 23 hours
        86400..=604799 => format!("{}d", seconds / 86400),     // 1 day to 6 days
        _ => format!("{}w", seconds / 604800),                 // 7 days and above
    }
}

pub fn parse_ignore_string(patterns: Vec<String>) -> Result<Gitignore, Box<dyn Error>> {
    let mut builder = GitignoreBuilder::new(Path::new("/"));
    for i in patterns {
        builder.add_line(None, &i)?;
    }

    Ok(builder.build()?)
}

pub fn get_bytes_string(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

pub struct Logger {
    pub enabled: bool,
}

impl Logger {
    fn _log<T: AsRef<str>>(&self, message: T, force: bool) {
        match self.enabled || force {
            true => println!("{}", message.as_ref()),
            false => return,
        }
    }
    pub fn log<T: AsRef<str>>(&self, message: T) {
        self._log(message, false)
    }
    pub fn force_log<T: AsRef<str>>(&self, message: T) {
        self._log(message, true)
    }
}
