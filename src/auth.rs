use crate::user_config::UserConfig;
use core::panic;
use std::io::{Read, Write};
use rspotify::{clients::{BaseClient, OAuthClient}, AuthCodePkceSpotify, ClientResult};
use std::net::{TcpListener, TcpStream};


// creates the config file on the config path
// manually make them enter the values
// takes user config, oauth 
// makes crednentials from user config 

// returns the authorization token which is later used to request access_token.
fn request_authorization(auth_url: String, user_conf: &UserConfig) -> Option<String> {
   let listener = TcpListener::bind(format!("127.0.0.1:{}", user_conf.get_port()));

   match listener {
        Ok(ls) => {
            match webbrowser::open(auth_url.as_str()) {
                Ok(_) => println!("Successfully opened in your browser"),
                Err(e) => panic!("Failed to open in your browser {}", e.to_string()),
            };

            for stream in ls.incoming() {
                match stream {
                    Ok(stream) => {
                        if let Some(code) = handle_connection(stream) {
                             return code.split("code=")
                                 .nth(1) 
                                 .and_then(|s| s.split('&').next())
                                 .map(|s| s.to_owned());
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
        Err(e) => {
            panic!("Failed to start the listener, Error: {}", e.to_string());
        }
    }

   None
}


fn handle_connection(mut stream: TcpStream) -> Option<String> {
    // The request will be quite large (> 512) so just assign plenty just in case
    let mut buffer = [0; 1000];
    let _ = stream.read(&mut buffer).unwrap();

    // convert buffer into string and 'parse' the URL
    match String::from_utf8(buffer.to_vec()) {
        Ok(request) => {
            let split: Vec<&str> = request.split_whitespace().collect();

            if split.len() > 1 {
                respond_with_success(stream);
                return Some(split[1].to_string());
            }
        }
        Err(e) => {
            respond_with_error(format!("Invalid UTF-8 sequence: {}", e), stream);
        }
    };

    None
}

fn respond_with_success(mut stream: TcpStream) {
  let contents = include_str!("redirect_uri.html");

  let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

  stream.write_all(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}

fn respond_with_error(error_message: String, mut stream: TcpStream) {
  println!("Error: {}", error_message);
  let response = format!(
    "HTTP/1.1 400 Bad Request\r\n\r\n400 - Bad Request - {}",
    error_message
  );

  stream.write_all(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}

// change all of this to pkce, and fix refreshing tomorrow. 
// checks the cache, regenerates the token if required. 
// checks if the user is authorized or not, if the user is authorized then it 
pub async fn get_access_token(pkce: &mut AuthCodePkceSpotify, user_conf: &UserConfig) -> ClientResult<()> {
    match pkce.read_token_cache(true).await {
        Ok(token) => {
            // since we are allowing expired tokens here, we will check if we need to refresh it
            // here. 
            match token {
                Some(t) => {
                    // Since we are reading from cache hence setting the value to the token is
                    // correct.
                    // Regardless the pkce object will not have token here at this point. 
                    *pkce.token.lock().await.unwrap() = Some(t.clone());
                    if t.is_expired() {
                        log::info!("Refreshed token here");
                        pkce.refresh_token().await?
                    }

                    log::info!("The unexpired token was used");

                    Ok(())
                },
                None => {
                    // reauthorize if for some reason the cache cannot be read;
                    let auth_url = pkce.get_authorize_url(Some(69))?;
                    let auth_code = match request_authorization(auth_url, user_conf) {
                        Some(code) => code,
                        None => panic!("Failed to get the authorization code"),
                    };

                    log::info!("Failed to get the cached token hence reauthentication again");
                    pkce.request_token(auth_code.as_str()).await
                }
            }
        },
        Err(_) => {
            // authorize here if for some reason we are not able to read_cache; 
            let auth_url = pkce.get_authorize_url(Some(69))?;
            let auth_code = match request_authorization(auth_url, user_conf) {
                Some(code) => code,
                None => panic!("Failed to get the authorization code"),
            };
            log::info!("Authentication");
            pkce.request_token(auth_code.as_str()).await
        }
    }
}
