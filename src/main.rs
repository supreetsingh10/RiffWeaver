use std::{ops::Deref, path::PathBuf};
use lib::{utils::generate_abs_path, constants::*, auth::get_access_token, user_config::{load_user_config, UserConfig}};
use rspotify::{clients::OAuthClient, scopes, AuthCodePkceSpotify, Config, Credentials, OAuth}; 


fn oauth_setup(usr_conf: &UserConfig) -> OAuth {
    // all the scopes are added, since we are working with AuthCodePkceSpotify we will be able to 
    // use them all. 
    let scopes = scopes!(
            "user-read-email",
            "user-read-private",
            "user-top-read",
            "user-read-recently-played",
            "user-follow-read",
            "user-library-read",
            "user-read-currently-playing",
            "user-read-playback-state",
            "user-read-playback-position",
            "playlist-read-collaborative",
            "playlist-read-private",
            "user-follow-modify",
            "user-library-modify",
            "user-modify-playback-state",
            "playlist-modify-public",
            "playlist-modify-private",
            "ugc-image-upload"
        );

    let mut oauth = rspotify::OAuth::default();
    oauth.redirect_uri = usr_conf.get_redirect_uri();
    oauth.scopes = scopes;
    oauth
}

// to implemenent the commandline user arguments, to take inputs. 
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

    log::info!("Credentials {:?}", creds);

    // for some reason the cache is not being read;
    let mut pkce = AuthCodePkceSpotify::with_config(creds, oauth, config);
    if let Err(e) = get_access_token(&mut pkce, &user_conf).await {
        println!("Failed to get the access token {}", e);
    }

}
