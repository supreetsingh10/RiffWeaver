use std::{ops::Deref, path::PathBuf};

// use lib::auth::authorize;
use lib::{utils::generate_abs_path, constants::*, user_config::{load_user_config, UserConfig}};
use rspotify::{ClientCredsSpotify, clients::{BaseClient,OAuthClient}, AuthCodePkceSpotify, Config, Credentials, OAuth}; 
use lib::auth::authorize; 


// Get creds and oauth
// AuthCodePkceSpotify::new(creds, oauth);
//
fn oauth_setup(usr_conf: &UserConfig) -> OAuth {
    let mut oauth = OAuth::default();
    oauth.redirect_uri = usr_conf.get_redirect_uri();
    oauth.scopes.insert(String::from("user-modify-playback-state"));
    oauth
}

#[tokio::main]
async fn main() {
    let user_conf: UserConfig = load_user_config(&generate_abs_path(CONFIG_PATH));
    let mut config = Config::default(); 
    
    // if it does not exist it will use the default path. 
    if let Some(cache_path) = user_conf.get_token_cache_path() {
        config.cache_path = PathBuf::from(cache_path);
    }

    let oauth = oauth_setup(&user_conf);
    let creds = Credentials::new_pkce(user_conf.get_client_id().as_str());

    let client_creds: ClientCredsSpotify = ClientCredsSpotify::with_config(creds, config);
    match client_creds.read_token_cache().await {
        Ok(cres) => {
            println!("{:?}", cres.unwrap());
        },
        Err(e) => println!("{:?}", e),
    };
    let mut pkce = AuthCodePkceSpotify::new(client_creds.get_creds().clone(), oauth);

    // check for access token if we have anyhting cached or not. 
    if let Some(auth_code) = authorize(&mut pkce,&user_conf) {
        println!("{}", auth_code.clone());
        // check out the access token here.
        if let Err(e) = pkce.request_token(auth_code.as_str()).await {
            println!("Failed to request token because of {}", e.to_string());
        } else {
            let t = pkce.get_token();
            let to = t.deref();
            let tok = to.lock().await.unwrap();
            let toke = tok.deref().clone().unwrap();
            println!("{:?}", toke);
        }
    }
}
