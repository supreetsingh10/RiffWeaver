use core::fmt;
use std::fs::File;
use std::io::BufReader; 
use serde_json::from_reader; 
use serde::{Deserialize, Serialize};
use crate::rustipy::constants::REQUEST_TOKEN_LINK;
use chrono::Utc;
use reqwest::{Client,Method};


#[derive(Serialize, Deserialize, Debug, Clone)]
struct Secrets {
    client_id: String,
    client_secret: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessToken {
    access_token: String, 
    token_type: String,
    redirect_uri: String,
    expires_in: i64,
    expires_at: Option<i64>,
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

    pub fn get_access_token_string(&self) -> String {
        self.access_token.to_string()
    }

    fn get_redirect_uri(&self) -> String {
        self.redirect_uri.clone()
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
async fn generate_access_token(creds: Secrets) -> Result<AccessTokenResponse, String> {
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
pub async fn get_access_token() -> Result<AccessToken, String> {
    let creds = match get_creds() {
        Ok(cred) => cred, 
        Err(e) => {
            return Err(e.to_string());
        }
    };

     generate_access_token(creds).await
        .map(|auth| {
            AccessToken::new(auth)
                .set_expires_at()
                .clone()
        })
}
