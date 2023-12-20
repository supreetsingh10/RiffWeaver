use std::fs::File;
use std::io::BufReader; 
use serde_json::from_reader; 
use serde::{Deserialize, Serialize};
use crate::rustipy::constants::REQUEST_TOKEN_LINK;
use reqwest::{Client,Method};
use crate::types::AccessToken; 


#[derive(Serialize, Deserialize, Debug, Clone)]
struct Creds {
    client_id: String,
    client_secret: String,
}


// read the creds.json file which is present in the src directory
// then generate an access token
fn get_creds() -> Result<Creds, std::io::Error> {
     match File::open("./src/creds.json") {
        Ok(f) => {
            let reader = BufReader::new(f);
            let creds: Creds = from_reader(reader)?;
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
async fn generate_access_token(creds: Creds) -> Result<AccessToken, String> {
     match Client::new()
        .request(Method::POST, REQUEST_TOKEN_LINK)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("grant_type=client_credentials&client_id=".to_owned() + creds.client_id.as_str() + "&client_secret=" + creds.client_secret.as_str())
        .send()
        .await
        {
            Ok(r) => {
                let access_token: AccessToken = match r.json().await {
                    Ok(at) => at,
                    Err(e) =>  {
                        println!("This is the error {}", e.to_string());
                        return Err(e.to_string()); 
                    }
                }; 
                Ok(access_token)
            },
            Err(e) => {
                println!("This is the error {}", e.to_string());
                return Err(e.to_string());
            }
        }
}

pub async fn get_access_token() -> Result<AccessToken, String> {
    let creds = match get_creds() {
        Ok(cred) => cred, 
        Err(e) => {
            return Err(e.to_string());
        }
    };

    generate_access_token(creds).await
}
