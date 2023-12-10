use chrono::{NaiveDateTime, Utc};
use clap::{App, Arg};
use regex::Regex;
use std::{env, fs, panic, process};

mod api;
mod config;
mod pack;
mod tools;

const BACKUP_FILE_PREFIX: &str = "SYNCOPY_BACKUP";
const DATE_FORMAT: &str = "%d_%m_%Y_%H_%M";

const QUIET_ARG_ID: &str = "quiet";
const CHECK_ARG_ID: &str = "check";
const DRY_RUN_ARG_ID: &str = "dry run";
const DIRTY_ARG_ID: &str = "dirty";

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Backup {
    name: String,
    created_at: NaiveDateTime,
    disk_path: Option<String>,
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
            Arg::with_name(QUIET_ARG_ID)
                .short("q")
                .long("quiet")
                .help("Do not output logs")
                .required(false),
        )
        .arg(
            Arg::with_name(CHECK_ARG_ID)
                .short("c")
                .long("check")
                .help("Only check previous backups")
                .required(false),
        )
        .arg(
            Arg::with_name(DRY_RUN_ARG_ID)
                .short("d")
                .long("dry-run")
                .help("Do not upload upon completion")
                .required(false),
        )
        .arg(
            Arg::with_name(DIRTY_ARG_ID)
                .short("D")
                .long("dirty")
                .help("Do not clean upon completion")
                .required(false),
        )
        .get_matches();

    let quiet = matches.is_present(QUIET_ARG_ID);

    let logger = tools::Logger { enabled: !quiet };

    logger.log("Initializing...");
    let current_date = Utc::now().naive_utc();
    let config = config::Config::parse(None)
        .unwrap_or_else(|e| panic!("Config parse error: {}", e));
    let token: String =
        tools::get_disk_token(Some(&config))
            .unwrap_or_else(|e| panic!("Env var 'SYNCOPY_YADISK_OAUTH_TOKEN' was not found, config had no token, and file '.disk_token' failed to open: {}", e));

    let api = api::DiskApi::new(token);

    logger.log("Getting previous backups...");
    let dir_contents = api.get_directory_contents(None).await.unwrap();

    let regex = tools::construct_backup_file_regex(&config.backups.output_suffix);

    let filtered: Vec<Backup> = dir_contents
        .iter()
        .filter(|i| Regex::new(&regex).unwrap().is_match(&i.name))
        .map(|i| Backup {
            name: i.name.clone(),
            created_at: NaiveDateTime::parse_from_str(
                &i.name,
                &tools::construct_backup_file_name(
                    BACKUP_FILE_PREFIX,
                    &config.backups.output_suffix,
                    DATE_FORMAT,
                ),
            )
            .expect("Date parse error"),
            disk_path: Some(i.path.clone()),
        })
        .collect();

    let latest_created_at = if let Some(latest) = tools::get_latest_backup(&filtered) {
        let delta = tools::get_delta_string(current_date, latest.created_at);
        latest.created_at.format("%d.%m.%Y %H:%M").to_string() + &format!(" ({} ago)", delta)
    } else {
        "never".to_string()
    };

    logger.log(&format!(
        "  Total backups: {}\n  Latest backup: {}",
        filtered.len(),
        latest_created_at
    ));

    if !matches.is_present(CHECK_ARG_ID) {
        logger.log("Preparing backup...");

        let output_file_name = tools::construct_backup_file_name(
            BACKUP_FILE_PREFIX,
            &config.backups.output_suffix,
            &current_date.format(DATE_FORMAT).to_string(),
        );
        let output_file = format!("{}{}", config.backups.output_directory, output_file_name);
        let input_paths = config.backups.include;

        let excluded = tools::parse_ignore_string(config.backups.exclude)
            .unwrap_or_else(|e| panic!("Invalid exclusion config: {}", e));

        logger.log(&format!("Packing into file '{}'...", output_file));

        let file_size = pack::pack_paths(&input_paths, &output_file, excluded, !quiet)
            .unwrap_or_else(|e| panic!("Pack failed: {}", e));

        logger.log(format!(
            "  File size: {}",
            tools::get_bytes_string(file_size)
        ));

        if !matches.is_present(DRY_RUN_ARG_ID) {
            logger.log("Preparing upload...");

            let upload_operation = api.get_upload_operation(&output_file_name).await.unwrap();
            logger.log(&format!("Uploading to {}...", upload_operation.href));

            let destination = api
                .upload_file(&upload_operation.href, &output_file)
                .await
                .unwrap();

            logger.force_log(if destination.is_some() {
                format!("  Available at {}", destination.unwrap())
            } else {
                "Upload done".to_string()
            });
        }

        if !matches.is_present(DIRTY_ARG_ID) {
            logger.log("Cleaning...");
            fs::remove_file(output_file).unwrap_or_else(|e| panic!("Deletion failed: {}", e));
        }
    }

    logger.force_log(format!(
        "  Done in {}",
        tools::get_delta_string(Utc::now().naive_utc(), current_date)
    ))
}
