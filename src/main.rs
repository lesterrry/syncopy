use std::collections::HashMap;
use std::{env, panic, process};
use std::fs::File;
use std::io::prelude::*;
use chrono::{NaiveDateTime};
use clap::{App, Arg};
use regex::Regex;

use crate::tools::get_latest_backup;

mod api;
mod tools;

const BACKUP_FILE_REGEX: &str = "/SYNCOPY_BACKUP_(\\d{2}_\\d{2}_\\d{4}_\\d{2}_\\d{2}).tar/gm";
const DATE_FORMAT: &str = "%d_%m_%Y_%H_%M";


#[derive(Clone)]
struct Backup {
    name: String,
    created_at: NaiveDateTime
}

#[tokio::main]
async fn main() {
    // Setting hook to panic prettier
	panic::set_hook(Box::new(move |panic_info| {
		eprintln!("FATAL: {}", panic_info);
		process::exit(1);
	}));

    let matches = App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about("Backup utility tethered with Yandex Disk")
		.arg(
			Arg::with_name("verbose")
                .short("v")
                .long("verbose")
				.help("display rich output info")
				.required(false)
		)
		.get_matches();

    let logger = tools::Logger{ verbose: true};//matches.is_present("verbose") };
    
    logger.log("Initializing...");
    let token: String =
    tools::get_token().unwrap_or_else(|e| panic!("Env var 'SYNCOPY_YADISK_OAUTH_TOKEN' was not found, and file '.token' failed to open.\n{}", e));

    let api = api::DiskApi::new(token);

    logger.log("Reading backup directory...");
    let dir_contents = api.get_directory_contents(None).await.unwrap();

    let filtered: Vec<Backup> = dir_contents
        .iter()
        .filter(|i| Regex::new(BACKUP_FILE_REGEX).unwrap().is_match(&i.name))
        .map(|i| Backup{name: i.name.clone(), created_at: NaiveDateTime::parse_from_str(&i.name, DATE_FORMAT).expect("Date parse error")})
        .collect();

    let latest_created_at =
        if let Some(latest) = get_latest_backup(&filtered) {
            latest.created_at.format(DATE_FORMAT).to_string()
        } else {
            "never".to_string()
        };
    
    logger.log(&format!("  Total backups: {}\n  Latest backup: {}", filtered.len(), latest_created_at));
    println!("{:?}", dir_contents)
}