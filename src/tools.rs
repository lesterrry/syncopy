use std::{error::Error, env, fs::File, io::Read};

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