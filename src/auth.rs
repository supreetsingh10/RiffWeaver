use crate::user_config::UserConfig;
use std::{io::{Read, Write}, ops::Deref};
use rspotify::{clients::{BaseClient, OAuthClient}, AuthCodePkceSpotify, Token};
use std::net::{TcpListener, TcpStream};


// creates the config file on the config path
// manually make them enter the values
// takes user config, oauth 
// makes crednentials from user config 
fn authorize(pkce: &mut AuthCodePkceSpotify, user_conf: &UserConfig) -> Option<String> {
    let auth_url = match pkce.get_authorize_url(Some(69)) {
        Ok(url) => url,
        Err(e) => {
            panic!("Failed to get the AuthUrl {}", e.to_string());
        }
    };

    request_authorization(auth_url, &user_conf)
}

async fn authorize_and_process_token(pkce: &mut AuthCodePkceSpotify, user_conf: &UserConfig) -> Option<Token> {
    if let Some(code) = authorize(pkce, &user_conf) {
        if let Err(e) = pkce.request_token(code.as_str()).await {
            println!("Failed to request a token {}", e.to_string());
            None
        } else {
            let token = pkce.get_token(); 
            let access_token = token.lock().await.expect("Failed to lock the token and failed unwrapping");
            access_token.deref().clone()
        }
    } else {
        None
    }
}

// returns the authorization token which is later used to request access_token.
fn request_authorization(auth_url: String, user_conf: &UserConfig) -> Option<String> {
   let listener = TcpListener::bind(format!("127.0.0.1:{}", user_conf.get_port()));

   match listener {
        Ok(ls) => {
            match webbrowser::open(auth_url.as_str()) {
                Ok(_) => println!("Successfully opened in your browser"),
                Err(e) => println!("Failed to open in your browser {}", e.to_string()),
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
                    Err(e) => println!("Error: {}", e.to_string()),
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

// checks the cache, regenerates the token if required. 
// should I check for authorization here as well. 
pub async fn get_access_token(pkce: &mut AuthCodePkceSpotify, user_conf: &UserConfig) -> Option<Token> {
    // return None, when the token is expired
    match pkce.read_token_cache(false).await {
        Ok(token) => {
            match token {
                Some(t) => Some(t),
                None => {
                    // refetch the token here. 
                    match pkce.refetch_token().await {
                        Ok(ref_token) => {
                            match ref_token {
                                Some(ref_token) => Some(ref_token),
                                None => {
                                    // reauthorize the client here as well. 
                                    authorize_and_process_token(pkce, user_conf).await
                                }
                            }
                        },
                        Err(e) => {
                            println!("Failed to get the refresh token {}", e.to_string());
                            return None;
                        }
                    }
                }
            }
        },
        Err(_) => {
            // in case there is an error related to reading the token cache, reauthorize the
            // client. 
            authorize_and_process_token(pkce, user_conf).await
        }
    }
}
