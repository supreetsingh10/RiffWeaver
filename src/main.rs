use std::path::PathBuf;
use lib::auth::get_access_token; 
// use lib::auth::authorize;
use lib::{utils::generate_abs_path, constants::*, user_config::{load_user_config, UserConfig}};
use rspotify::{AuthCodePkceSpotify, Config, Credentials, OAuth}; 


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
    env_logger::init();
    let user_conf: UserConfig = load_user_config(&generate_abs_path(CONFIG_PATH));
    let mut config = Config::default(); 
    
    // if it does not exist it will use the default path. 
    if let Some(cache_path) = user_conf.get_token_cache_path() {
        log::info!("Token cache path set {}", generate_abs_path(&cache_path));
        config.cache_path = PathBuf::from(generate_abs_path(&cache_path));
        config.token_cached = true;
    }

    log::info!("Config {:?}", &config);
    let oauth = oauth_setup(&user_conf);
    let creds = Credentials::new_pkce(user_conf.get_client_id().as_str());

    // for some reason the cache is not being read;
    let mut pkce = AuthCodePkceSpotify::with_config(creds, oauth, config);
    let ass_token = get_access_token(&mut pkce, &user_conf).await;
    match ass_token {
        Some(ass) => println!("{:?}", ass),
        None => println!("Found nothing"),
    };

    // check for access token if we have anyhting cached or not. 
}
