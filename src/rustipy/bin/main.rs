// Added for testing purposes. 
// use this to test the spotify api interaction thing. 
use libra::{rustipy::{authorization::{self, if_user_authorized, get_authorize_token}, constants::{RUSTIPY_CACHE, AUTH_CONFIG}, config::get_auth_config}, utility}; 

fn main() {
    // check if the user is authorized already? 
    let mut cache= utility::get_env_var("HOME");
    let mut auth_config = cache.clone(); 
    cache.push_str(RUSTIPY_CACHE); 
    if !if_user_authorized(cache) {
        // Authentication direction 
        auth_config.push_str(AUTH_CONFIG);
        let ac = get_auth_config(auth_config);
        get_authorize_token(ac);
        // This ac will be used to populate the auth code
    } else {
        // Refresh token direction
    }
}
