use std::collections::HashMap;
use std::{env, panic, process};
use std::fs::File;
use std::io::prelude::*;
use clap::{App, Arg};

mod api;
mod tools;

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

    let logger = tools::Logger { verbose: matches.is_present("verbose") };
    
    logger.log("Initializing...");
    let token = tools::get_token().expect("Env var 'SYNCOPY_YADISK_OAUTH_TOKEN' was not found, and file '.token' failed to open.");
    let api = api::DiskApi::new(token);

    logger.log("Reading backup directory...");
    let dir_contents = api.get_dir_contents(None).await;

    println!("{:?}", dir_contents)
}