use core::{fmt, panic};
use std::fs::File;
use std::io::{BufReader, Write}; 
use serde_json::from_reader; 
use serde::{Deserialize, Serialize};
use chrono::Utc;
use reqwest::{Client,Method};

use crate::rustipy::constants::REQUEST_TOKEN_LINK;
use crate::utility::{open_file, create_tree_for_file};

use super::constants::RUSTIPY_CACHE;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Secrets {
    client_id: String,
    client_secret: String,
}

// gets the client secrts, client id and makes a structure that will be refenced again and again. 

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AccessTokenResponse {
    access_token: String,
    token_type: String, 
    expires_in: i64,
}

impl fmt::Display for AccessToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{} {} {} ", self.access_token, self.token_type, self.expires_in) 
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessToken {
    access_token: String, 
    token_type: String,
    redirect_uri: String,
    expires_in: i64,
    expires_at: Option<i64>,
}


// So this access token will be transferred around and this will be used for authentication.
// Now the creds of the user, the client and secret will be taken up from the file. 
// I will be storing the response of the in it. 
impl AccessToken {
    fn default() -> Self {
        AccessToken { 
            access_token: String::new(),
            token_type: String::new(),
            redirect_uri: String::from("http://localhost"),
            expires_in: 0i64,
            expires_at: None,
        }
    }

    fn new(auth: AccessTokenResponse) -> Self {
        AccessToken { 
            access_token: auth.access_token,
            token_type: auth.token_type, 
            expires_in: auth.expires_in,
            ..AccessToken::default()
        }
    }

    fn set_access_token(&mut self, at: impl Into<String>) -> &mut Self {
        self.access_token = at.into();
        self
    }

    fn set_token_type(&mut self, tt: impl Into<String>) -> &mut Self {
        self.token_type = tt.into(); 
        self
    }

    fn set_redirect_uri(&mut self, ru: impl Into<String>) -> &mut Self {
        self.redirect_uri = ru.into(); 
        self
    }

    fn set_expires_at(&mut self) -> &mut Self {
        let now_dt = Utc::now();
        self.expires_at = Some(now_dt.timestamp() + self.expires_in); 
        self
    }

    // checks for the present time and the time got from the cached token
    // returns false if not expired
    // returns true if expired 
    fn check_if_expired(&self) -> bool {
        let dt = Utc::now();
        (dt.timestamp() - self.expires_at.unwrap()) > 3550
    }

    pub fn get_access_token_string(&self) -> String {
        self.access_token.to_string()
    }

    fn get_redirect_uri(&self) -> String {
        self.redirect_uri.clone()
    }

    fn write_to_file(self, file_path: impl Into<String>) -> Self {
        if let Ok(mut f) = File::create(file_path.into()) {
            if let Ok(access_str) = serde_json::to_string_pretty(&self) {
                f.write_all(access_str.as_bytes()).
                    map_err(|e| {
                        println!("Failed to write the file {}", e);
                    });
                self
            } else {
                panic!("Failed to turn to string"); 
            }
        } else {
            panic!("Unable to make file");
        }
    }

    fn expired(&self) -> bool {
        let dt = Utc::now(); 
        // if expired is not none, then check if the current timestamp has excedded the time set
        self.expires_at.is_some_and(|exp_dt| dt.timestamp() > exp_dt)
    }
}

// read the creds.json file which is present in the src directory
// then generate an access token
fn get_creds() -> Result<Secrets, std::io::Error> {
     match File::open("./src/creds.json") {
        Ok(f) => {
            let reader = BufReader::new(f);
            let creds: Secrets = from_reader(reader)?;
            Ok(creds)
        }
        Err(e) => {
            println!("This is the error {}", e.to_string());
            return Err(e);
        }
    }
}

// used to generate an access token which stays for 3600 seconds.
// So the HTTP-POST request will be going in 2 blocks,
// There will be a header, and there will be the body 
// So the header needs to have this {Content-Type, Authorization {Client-ID, Client-Secret}}
// The body needs to have {Last-Code, Redirect-URI & Grant-Type}
// There is no actual redirection it is just for authentication
async fn request_access_token(creds: Secrets) -> Result<AccessTokenResponse, String> {
     let resp = Client::new()
        .request(Method::POST, REQUEST_TOKEN_LINK)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(String::from("grant_type=client_credentials&client_id=") 
            + creds.client_id.as_str() 
            + "&client_secret=" 
            + creds.client_secret.as_str()
            )
        .send()
        .await;

    if let Ok(re) = resp {
        re.json::<AccessTokenResponse>().await
            .map(|r| {
                r
            })
        .map_err(|e| {
            e.to_string()
        })
    } else {
        panic!("Failed to get the response"); 
    } 
}

// make a cache for the token 
// Check if it exists or it has expired. If it has expired then 
// regeneate the Authorization token. 
async fn generate_access_token() -> Result<AccessToken, String> {
    let creds = match get_creds() {
        Ok(cred) => cred, 
        Err(e) => {
            return Err(e.to_string());
        }
    };

     request_access_token(creds).await
        .map(|auth| {
            AccessToken::new(auth)
                .set_expires_at()
                .clone()
        })

}

// read the cache file check if the token has expired if not then return the access_token
// if it has then generate a new token
pub async fn get_access_token() -> Result<AccessToken, String> {
    match open_file(RUSTIPY_CACHE) {
        Some(f) => {
            if let Ok(cached_token) = process_file_for_tokens(f) {
                if cached_token.check_if_expired() { 
                    generate_access_token().await.map(|at| at.write_to_file(RUSTIPY_CACHE))
                } else { 
                    Ok(cached_token) 
                }
            } else {
                // if failing to process the file regenerate the token and write to it. 
                generate_access_token().await.map(|at| at.write_to_file(RUSTIPY_CACHE))
            }

        }
        None => {
            create_tree_for_file(RUSTIPY_CACHE).is_some().then(|| {
                println!("Okay the file was created");
            });
            generate_access_token().await.map(|at| at.write_to_file(RUSTIPY_CACHE))
        }
    }
    
}

// Gets the files, serializes into a json object gets the tokens
fn process_file_for_tokens(file_ptr: std::fs::File) -> Result<AccessToken, String> {
    let reader = BufReader::new(file_ptr);
    from_reader(reader)
    .map_err(|e| {
        e.to_string()
    })
}

// async function that will be checking if the directory for the cache exists or not
// if it does not exist then create the file and the folder. 
// everytime a request is made check for the written values. 


