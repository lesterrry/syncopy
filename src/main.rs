use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token: String;
    if let Ok(env_token) = env::var("SYNCOPY_YADISK_OAUTH_TOKEN") {
        token = env_token
    } else {

    }

    let api = api::DiskApi { token: token };
    Ok(())
}