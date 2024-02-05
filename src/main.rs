use std::path::PathBuf;

// use lib::auth::authorize;
use lib::{constants::*, user_config::{load_user_config, UserConfig}};
use rspotify::{model::user, AuthCodePkceSpotify, Config, Credentials, OAuth}; 
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

fn main() {
    let user_conf: UserConfig = load_user_config(CONFIG_PATH);
    let mut config = Config::default(); 
    
    // if it does not exist it will use the default path. 
    if let Some(cache_path) = user_conf.get_token_cache_path() {
        config.cache_path = PathBuf::from(cache_path);
    }

    let oauth = oauth_setup(&user_conf);
    let creds = Credentials::new_pkce(user_conf.get_client_id().as_str());
    let mut pkce = AuthCodePkceSpotify::new(creds, oauth);

    let auth_code = authorize(&mut pkce,&user_conf);
}
