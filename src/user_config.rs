use serde::{Deserialize, Serialize};
use crate::{constants::CONFIG_PATH, utils::generate_abs_path};
use std::{fs::File,io::{self, Write,stdin, stdout, BufReader}};

#[derive(Serialize, Deserialize)]
pub struct UserConfig 
{
    client_id: String,
    redirect_uri: String,
    port: u16,
    token_cache_path: Option<String>, 
    keybinds: Option<String>
}

impl UserConfig {
    fn new(c_id: String) -> Self {
        UserConfig {
            client_id: c_id,
            redirect_uri: String::from("http://localhost:8888/riff"),
            port: 8080,
            token_cache_path: None,
            keybinds: None
        }
    }

    pub fn get_client_id(&self) -> String {
        self.client_id.clone()
    }

    pub fn get_redirect_uri(&self) -> String {
        self.redirect_uri.clone()
    }

    pub fn get_token_cache_path(&self) -> Option<String> {
        self.token_cache_path.clone()
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn create_config(self) -> Self {
        match File::create(generate_abs_path(CONFIG_PATH)) {
            Ok(mut f) => {
                let t = serde_json::to_string(&self)
                    .expect("Failed to parse to string, please check the config file");
                let _ = f.write_all(t.as_bytes());
                self
            }
            Err(e) => {
                panic!(
                    "Failed to create the file at {}, please check the permissions, Error {}",
                    CONFIG_PATH,
                    e.to_string()
                );
            }
        }
    }
}


// parses the config file and returns a UserConfig object which can be later used.
fn parse_config(file_path: &str) -> std::io::Result<UserConfig> {
    File::open(std::path::Path::new(&file_path)).map(|fp| {
        let reader = BufReader::new(fp);
        serde_json::from_reader(reader).unwrap()
    })
}


// take the user input for the client id and redirect_uri
fn take_values() -> io::Result<UserConfig> {
    println!("Since the config was not found here {}", CONFIG_PATH);
    println!("So we will be creating that file for you.");
    println!("Enter the client id from the spotify dashboard");
    let mut c_id = String::new();
    stdout().flush()?;
    stdin().read_line(&mut c_id)?;
    stdout().flush()?;
    println!("We are going to use the default redirect_uri which will be https://localhost:8080");
    Ok(UserConfig::new(c_id.to_owned()))
}

pub fn load_user_config(config_path: &str) -> UserConfig {
    match parse_config(config_path) {
        Ok(uc) => uc,
        Err(_) => {
            take_values()
                .expect("Failed to take values")
                .create_config()
        }
    }
}
