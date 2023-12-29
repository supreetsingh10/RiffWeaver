// The request will be made through one channel only 
// Client id will be taken from the file. 

// Only constructed when is run the first time. 
use std::{path::Path}; 
use crate::{rustipy::config::AuthConfig, utility}; 
use serde::{Serialize, Deserialize};
use sha256::digest; 
use base64::{engine::general_purpose::STANDARD, Engine as _};


#[derive(Serialize, Deserialize, Clone, Debug)] 
struct AuthorizationBuilder {
    client_id: String, 
    response_type: String, 
    scope: String, 
    code_challenge_method: String, 
    code_challenge: String, 
    redirect_uri: String,
}

impl AuthorizationBuilder {
    fn default() -> Self {
        AuthorizationBuilder { 
            client_id: String::new(),
            response_type: String::from("code"),
            scope: String::new(),
            code_challenge_method: String::from("S256"),
            code_challenge: String::new(),
            redirect_uri:  String::new(),
        }
    }

    fn set_config_values(&mut self, ac: AuthConfig) -> &mut Self {
        self.client_id = ac.client_id; 
        self.redirect_uri = ac.redirect_uri; 
        self
    }

    fn set_auth_scope(&mut self, in_scope: impl Into<String>) -> &mut Self {
        self.scope = in_scope.into(); 
        self
    }

    fn set_code_challenge(&mut self, cc: impl Into<String>) -> &mut Self {
        self.code_challenge = cc.into();
        self
    }
}

// if user is authorized then redirect and use the refresh tokens. 
// check token buffer exists. 
pub fn if_user_authorized(f_path: impl Into<String>) -> bool {
    Path::exists(Path::new(&f_path.into()).into())
}

// It will return a response. 
pub fn get_authorize_token(auth_config: AuthConfig) {
    println!("{:#?}", prepare_for_auth(auth_config));
}

fn prepare_for_auth(auth_config: AuthConfig) -> Option<AuthorizationBuilder> {
    Some(AuthorizationBuilder::default()
        .set_config_values(auth_config)
        .set_auth_scope("user_read_private user_read_email")
        .set_code_challenge(
            utility::generate_random_string(64)
            .map(|s| digest(s))
            .map(|s| STANDARD.encode(s))
            .unwrap_or(String::from("Failed to generate the code challenge"))
        )
        .clone())
}
