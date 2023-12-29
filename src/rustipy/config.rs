use core::panic;
use std::{io::BufReader, fs};

use serde::{Serialize, Deserialize}; 
use serde_json::from_reader; 

#[derive(Serialize, Deserialize)]
pub struct AuthConfig {
    pub client_id: String, 
    pub redirect_uri: String,
}

pub fn get_auth_config(config_path: impl Into<String>) -> AuthConfig {
    let f = match fs::File::open(&config_path.into()) {
        Ok(f) => f,
        Err(e) => panic!("Config file not found at path: Error -> {}", e), 
    };

    let reader = BufReader::new(f); 
    if let Ok(ac) = from_reader(reader) {
        ac
    } else {
        panic!("Failed to deserialize the authconfig"); 
    }
}
